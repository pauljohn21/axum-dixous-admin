# 后端优化设计文档

> 日期：2026-07-20
> 状态：已批准
> 范围：axum-dixous-admin 后端全栈优化（性能 + 可靠性 + 安全就绪 + 架构重构）

## 1. 背景与现状分析

### 1.1 当前架构

后端采用分层结构：`gateway` → `api` → `service` → `model (dao/dto)` → `utils`。

- **配置**：`config.yml` 通过 `include_str!` 编译期内嵌，`once_cell::Lazy` 全局持有
- **数据库**：SeaORM + MySQL，`OnceCell<DatabaseConnection>` 全局单例，`db_conn!()` 宏获取连接
- **权限**：Casbin `CachedEnforcer`，`Arc<RwLock<>>` 包裹，通过 `service::enforcer::set_enforcer()` 注入
- **认证**：自定义 `AuthLayer` 中间件，JWT 验证 + Casbin enforce
- **错误**：Service 层返回 `anyhow::Result<T>`，API 层用 `AppError` 转换
- **响应**：统一 `R<T>` 结构 `{ code, message, data }`

### 1.2 发现的问题

| # | 问题 | 影响 | 位置 |
|---|------|------|------|
| 1 | Redis 已配置但从未使用 | JWT 黑名单、Casbin 缓存等全部走 MySQL | `config.yml` 有配置，代码无引用 |
| 2 | JWT 黑名单每次请求查 DB | 高频请求时数据库压力大 | `auth-layer/src/middleware.rs:104-111` |
| 3 | 无请求日志/链路追踪 | 线上问题难以排查 | `gateway/src/main.rs` 无 TraceLayer |
| 4 | 无优雅关闭 | 部署时请求被粗暴中断 | `gateway/src/main.rs:48` |
| 5 | `reqwest::get` 每次新建客户端 | 连接开销浪费 | `service/src/sys_user_service.rs:241` |
| 6 | 仪表盘 4 个 count 查询串行执行 | 响应时间叠加 | `service/src/sys_user_service.rs:257-268` |
| 7 | 无压缩/超时/限流中间件 | 带宽浪费、无超时保护、登录无限流 | `gateway/src/main.rs` |
| 8 | 健康检查过于简单 | DB 挂了仍报健康 | `api/src/lib.rs:154-156` |
| 9 | Service 层返回 `anyhow::Result` | 错误类型信息丢失 | 所有 `service/src/*.rs` |
| 10 | 全局静态变量（DB/CONFIG/Enforcer） | 难以测试、难以多实例 | `utils/src/db.rs`、`utils/src/config.rs` |
| 11 | 无 Service trait 抽象 | 无法 mock 测试 | 所有 `service/src/*.rs` |
| 12 | 配置不支持环境变量覆盖 | Docker 部署需 sed 替换 | `utils/src/config.rs` |
| 13 | 测试覆盖极低 | 仅 2 个密码工具测试 | `utils/tests/test_password_utils.rs` |

## 2. 优化目标

- **性能**：Redis 缓存 JWT 黑名单和热点数据，并行查询，HTTP 客户端复用，响应压缩
- **可靠性**：优雅关闭，请求链路追踪，健康检查增强，请求超时保护
- **架构**：AppState 共享状态注入，统一领域错误层级，配置环境变量覆盖，Service trait 抽象
- **质量**：三层测试套件（Service 单元测试 + API 集成测试 + Mock 测试）
- **安全就绪**：标准化中间件栈（结构搭好，参数宽松，开发阶段不收紧）

## 3. 分阶段实施计划

### 3.1 依赖关系

```
P1 基础层（必须先做）
  ├── 配置环境变量覆盖 ← 其他所有模块依赖配置
  ├── AppState 共享状态 ← 中间件、Service、API 都依赖 State
  └── 统一错误层级 ← Service 和 API 层的错误处理基础
        │
P2 中间件与缓存层（依赖 P1）
  ├── 中间件链重构 ← 依赖 AppState
  └── Redis 缓存层 ← 依赖 AppState + Config
        │
P3 Service 与测试层（依赖 P1 + P2）
  ├── Service trait 抽象 ← 依赖错误类型 + AppState
  └── 测试套件 ← 依赖 trait（mock）+ AppState（集成测试）
```

### 3.2 阶段定义

| 阶段 | 内容 | 影响范围 | 风险 |
|------|------|----------|------|
| P1 | 配置环境变量 + AppState + 统一错误 | `utils/`、`gateway/`、所有 `service/` 和 `api/` 签名 | 高 — 全量签名变更 |
| P2 | 中间件链 + Redis 缓存层 | `gateway/main.rs`、`auth-layer/`、新增 `utils/redis.rs` | 中 — 新增为主 |
| P3 | Service trait + 测试套件 | `service/`、新增 `tests/` | 低 — 追加为主 |

## 4. P1 基础层设计

### 4.1 配置环境变量覆盖

**改造文件：** `utils/src/config.rs`

**方案：** 加载 `config.yml` 作为默认值，然后用环境变量覆盖关键字段。

**环境变量命名规范：** `ADMIN_{SECTION}_{FIELD}`

| 环境变量 | 覆盖字段 | 说明 |
|----------|----------|------|
| `ADMIN_SERVER_HOST` | `server.host` | 服务监听地址 |
| `ADMIN_SERVER_PORT` | `server.port` | 服务监听端口 |
| `ADMIN_DB_HOST` | `datasource.host` | MySQL 主机 |
| `ADMIN_DB_PORT` | `datasource.port` | MySQL 端口 |
| `ADMIN_DB_DATABASE` | `datasource.database` | 数据库名 |
| `ADMIN_DB_USERNAME` | `datasource.username` | 数据库用户名 |
| `ADMIN_DB_PASSWORD` | `datasource.password` | 数据库密码 |
| `ADMIN_REDIS_HOST` | `cache.host` | Redis 主机 |
| `ADMIN_REDIS_PORT` | `cache.port` | Redis 端口 |
| `ADMIN_REDIS_PASSWORD` | `cache.password` | Redis 密码 |
| `ADMIN_JWT_SECRET` | `jwt.secret` | JWT 密钥 |
| `ADMIN_JWT_EXPIRE_HOURS` | `jwt.expire_hours` | JWT 过期时间 |
| `ADMIN_WECHAT_APPID` | `wechat.appid` | 微信 AppID |
| `ADMIN_WECHAT_SECRET` | `wechat.secret` | 微信 Secret |

**实现：**

```rust
impl Config {
    pub fn load() -> Self {
        let data = include_str!("../../config.yml");
        let mut config: Config = serde_yaml::from_str(data).expect("无法读取配置信息");
        // 环境变量覆盖
        if let Ok(v) = env::var("ADMIN_SERVER_HOST") { config.server.host = v; }
        if let Ok(v) = env::var("ADMIN_SERVER_PORT") { config.server.port = v.parse().unwrap_or(config.server.port); }
        if let Ok(v) = env::var("ADMIN_DB_HOST") { config.datasource.host = v; }
        if let Ok(v) = env::var("ADMIN_DB_PORT") { config.datasource.port = v.parse().unwrap_or(config.datasource.port); }
        if let Ok(v) = env::var("ADMIN_DB_DATABASE") { config.datasource.database = v; }
        if let Ok(v) = env::var("ADMIN_DB_USERNAME") { config.datasource.username = v; }
        if let Ok(v) = env::var("ADMIN_DB_PASSWORD") { config.datasource.password = v; }
        if let Ok(v) = env::var("ADMIN_REDIS_HOST") { config.cache.host = v; }
        if let Ok(v) = env::var("ADMIN_REDIS_PORT") { config.cache.port = v.parse().unwrap_or(config.cache.port); }
        if let Ok(v) = env::var("ADMIN_REDIS_PASSWORD") { config.cache.password = v; }
        if let Ok(v) = env::var("ADMIN_JWT_SECRET") { config.jwt.secret = v; }
        if let Ok(v) = env::var("ADMIN_JWT_EXPIRE_HOURS") { config.jwt.expire_hours = v.parse().unwrap_or(config.jwt.expire_hours); }
        if let Ok(v) = env::var("ADMIN_WECHAT_APPID") { config.wechat.appid = v; }
        if let Ok(v) = env::var("ADMIN_WECHAT_SECRET") { config.wechat.secret = v; }
        config
    }
}
```

**Dockerfile 影响：** 不再需要 `sed` 替换 `localhost` → `mysql`/`redis`，改用 `ENV` 环境变量。

### 4.2 AppState 共享状态

**新增文件：** `utils/src/state.rs`

**AppState 结构：**

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub http_client: reqwest::Client,
    pub config: Config,
    // P2 阶段追加:
    // pub redis: ConnectionManager,
}
```

**初始化（`gateway/src/main.rs`）：**

```rust
let state = AppState {
    db: DB::db_connection().await.clone(),
    enforcer: enforcer.clone(),
    http_client: reqwest::Client::new(),
    config: CONFIG.clone(),
};

let app = Router::new()
    .merge(api::public_routes())
    .merge(api::swagger_routes())
    .merge(api::protected_routes().layer(auth_layer))
    .with_state(state);
```

**Service 签名变更规则：**

- 所有 Service 关联函数的第一个参数变为 `db: &DatabaseConnection`
- 返回类型从 `Result<T>` (anyhow) 变为 `Result<T, ServiceError>`
- API handler 提取 `State<AppState>` 并传递 `&state.db` 给 Service

**示例：**

```rust
// Service 层
pub async fn login(db: &DatabaseConnection, data: LoginDTO) -> Result<sys_user::Model, ServiceError> { ... }

// API 层
pub async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginDTO>,
) -> Result<impl IntoResponse, AppError> {
    let user = SysUserService::login(&state.db, data).await?;
    let token = create_token(&user.username.unwrap_or_default())?;
    Ok(R::ok(LoginResp { token }))
}
```

**过渡策略：**
- `db_conn!()` 宏和 `service::enforcer` 保留但标记 `#[deprecated]`
- P1 阶段逐文件迁移，确保编译通过后再删除全局静态变量
- `CONFIG` 全局静态保留用于启动期，启动后通过 `AppState` 传递

**`db_conn!()` 宏迁移注意：** 全项目约 30+ 处调用 `db_conn!()`，需逐一替换为 `&state.db` 参数传递。建议按 Service 文件逐个迁移，每完成一个文件确认编译通过。

### 4.3 统一错误层级

**改造文件：** `utils/src/error.rs`

**新增 `ServiceError` 领域错误类型：**

```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    // ── 通用领域错误 ──
    #[error("资源不存在: {0}")]
    NotFound(String),
    #[error("认证失败: {0}")]
    Auth(String),
    #[error("权限不足: {0}")]
    Forbidden(String),
    #[error("参数错误: {0}")]
    BadRequest(String),

    // ── 用户领域特定 ──
    #[error("用户不存在")]
    UserNotFound,
    #[error("密码错误")]
    InvalidPassword,
    #[error("该微信号已绑定其他账号")]
    WechatAlreadyBound,
    #[error("微信登录失败: {0}")]
    WechatApi(String),

    // ── 基础设施错误 ──
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("JWT 错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    // P2 追加:
    // #[error("Redis 操作失败: {0}")]
    // Redis(#[from] redis::RedisError),
}
```

**`AppError` 改造：**

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error("认证失败: {0}")]
    AuthError(String),
    #[error("权限不足: {0}")]
    Forbidden(String),
    #[error("资源不存在: {0}")]
    NotFoundError(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

impl From<ServiceError> for AppError {
    fn from(e: ServiceError) -> Self {
        match &e {
            ServiceError::NotFound(_) | ServiceError::UserNotFound =>
                AppError::NotFoundError(e.to_string()),
            ServiceError::Auth(_) | ServiceError::InvalidPassword
            | ServiceError::WechatApi(_) | ServiceError::WechatAlreadyBound =>
                AppError::AuthError(e.to_string()),
            ServiceError::Forbidden(_) =>
                AppError::Forbidden(e.to_string()),
            ServiceError::BadRequest(_) =>
                AppError::AuthError(e.to_string()),
            _ => AppError::Internal(e.to_string()),
        }
    }
}
```

**迁移策略：** 优先迁移 `user_service`、`role_service`、`menu_service`（最复杂），其余 Service 用 `ServiceError::NotFound` / `ServiceError::Auth` 等通用变体过渡，后续逐步细化。不做一次性全量枚举所有错误。

## 5. P2 中间件链 + Redis 缓存层设计

### 5.1 中间件链重构

**改造文件：** `gateway/src/main.rs`

**中间件栈（从内到外）：**

```rust
let app = Router::new()
    .merge(api::public_routes())
    .merge(api::swagger_routes())
    .merge(api::protected_routes().layer(auth_layer))
    // ── 标准化中间件栈 ──
    .layer(CompressionLayer::new())               // 响应压缩 (gzip/br)
    .layer(TimeoutLayer::new(Duration::from_secs(30)))  // 请求超时
    .layer(
        TraceLayer::new_for_http()                 // 请求链路追踪
            .make_span_with(|req: &Request<_>| {
                tracing::info_span!("http",
                    method = %req.method(),
                    uri = %req.uri(),
                    request_id = %Uuid::new_v4(),
                )
            })
    )
    .layer(cors)                                   // CORS（最外层）
    .with_state(state);
```

**优雅关闭：**

```rust
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.unwrap(); };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate()
        ).unwrap().recv().await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {}, }
    tracing::info!("收到关闭信号，等待请求处理完成...");
}
```

**健康检查增强：**

```rust
async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = state.db.ping().await.is_ok();
    let redis_ok = redis::cmd("PING")
        .query_async::<String>(&mut state.redis.clone())
        .await.is_ok();
    let status = if db_ok && redis_ok { 200 } else { 503 };
    (
        StatusCode::from_u16(status).unwrap(),
        Json(json!({ "db": db_ok, "redis": redis_ok })),
    )
}
```

**新增依赖（`Cargo.toml` workspace dependencies）：**

```toml
tower-http = { version = "0.6", features = ["cors", "trace", "compression-full", "timeout"] }
uuid = { version = "1.8", features = ["v4"] }
redis = { version = "0.27", features = ["tokio-comp", "connection-manager"] }
```

### 5.2 Redis 缓存层

**新增文件：** `utils/src/redis.rs`

```rust
pub struct Redis;

impl Redis {
    pub async fn connection(config: &Cache) -> ConnectionManager {
        let url = format!("redis://:{}@{}:{}", config.password, config.host, config.port);
        let client = redis::Client::open(url).expect("无法连接 Redis");
        ConnectionManager::new(client).await.expect("Redis 连接管理器创建失败")
    }
}
```

**AppState 追加 Redis 字段：**

```rust
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,           // ← P2 新增
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub http_client: reqwest::Client,
    pub config: Config,
}
```

#### 5.2.1 JWT 黑名单 → Redis

**现状：** `middleware.rs:104-111` 每个受保护请求查 MySQL `jwt_blacklists` 表。

**改造：**

- 登出时写入 Redis（带 TTL，自动过期）：

```rust
let ttl = CONFIG.jwt.expire_hours * 3600;
redis::cmd("SET")
    .arg(format!("jwt_blacklist:{}", token))
    .arg("1").arg("EX").arg(ttl)
    .query_async::<()>(&mut redis_conn).await?;
```

- 中间件检查（O(1) Redis EXISTS）：

```rust
let blacklisted: bool = redis::cmd("EXISTS")
    .arg(format!("jwt_blacklist:{}", token))
    .query_async(&mut redis_conn).await?;
```

**`AuthLayer` 新增 Redis 字段：**

```rust
pub struct AuthLayer {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub redis: ConnectionManager,   // ← 新增
}
```

**MySQL `jwt_blacklists` 表保留作为持久化备份**，热查询走 Redis。启动时可选地从 DB 加载未过期的黑名单到 Redis（防止 Redis 重启丢失）。

#### 5.2.2 热点数据缓存

| 缓存对象 | Redis Key | TTL | 失效时机 |
|----------|-----------|-----|----------|
| 用户菜单 | `user_menus:{username}` | 1h | 菜单/角色变更时删除 |
| 字典数据 | `dict:{type}` | 2h | 字典修改时删除 |
| 用户权限策略 | `user_perms:{username}` | 1h | 角色权限变更时删除 |

**缓存模式：** Cache-Aside（先查 Redis，miss 则查 DB 并回填）。

```rust
// 示例：用户菜单缓存
pub async fn get_menus_cached(
    db: &DatabaseConnection,
    redis: &ConnectionManager,
    username: &str,
) -> Result<Vec<sys_menu::Model>, ServiceError> {
    let key = format!("user_menus:{}", username);
    // 1. 查 Redis
    if let Some(cached) = redis_get::<Vec<_>>(redis, &key).await? {
        return Ok(cached);
    }
    // 2. Miss → 查 DB
    let menus = SysMenuService::get_menus_by_username(db, username).await?;
    // 3. 回填 Redis
    redis_set(redis, &key, &menus, 3600).await?;
    Ok(menus)
}
```

#### 5.2.3 Casbin 缓存策略

**保持现状不变。** `CachedEnforcer` 已有内存缓存，enforce 结果在内存中命中。Redis 在此场景无额外收益（内存查找 < 网络往返），仅用于策略变更时的跨实例通知（如果未来多实例部署）。

### 5.3 其他性能优化

#### 5.3.1 reqwest Client 复用

**现状：** `sys_user_service.rs:241` 使用 `reqwest::get()` 每次创建新客户端。

**改造：** 在 `AppState` 中持有 `reqwest::Client` 单例，Service 函数通过 `&state.http_client` 调用。

#### 5.3.2 Dashboard 并行查询

**现状：** `sys_user_service.rs:257-268` 4 个 count 查询串行执行。

**改造：**

```rust
pub async fn dashboard_stats(db: &DatabaseConnection) -> Result<DashboardStats, ServiceError> {
    let (user_count, role_count, menu_count, api_count) = tokio::try_join!(
        SysUser::find().count(db),
        SysRole::find().count(db),
        SysMenu::find().count(db),
        SysApis::find().count(db),
    )?;
    Ok(DashboardStats { user_count, role_count, menu_count, api_count })
}
```

## 6. P3 Service Trait 抽象 + 测试套件设计

### 6.1 Service Trait 抽象

**新增文件：** `service/src/traits.rs`

**Trait 定义模式：**

```rust
#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn login(&self, db: &DatabaseConnection, data: LoginDTO)
        -> Result<sys_user::Model, ServiceError>;
    async fn user_info(&self, db: &DatabaseConnection, username: &str)
        -> Result<sys_user::Model, ServiceError>;
    async fn list(&self, db: &DatabaseConnection, query: PageRequest)
        -> Result<PageResponse<sys_user::Model>, ServiceError>;
    async fn insert(&self, db: &DatabaseConnection, data: SysUserInsertDTO)
        -> Result<(), ServiceError>;
    async fn update(&self, db: &DatabaseConnection, id: i32, data: SysUserUpdateDTO)
        -> Result<sys_user::Model, ServiceError>;
    async fn delete(&self, db: &DatabaseConnection, id: i32)
        -> Result<(), ServiceError>;
    async fn change_password(&self, db: &DatabaseConnection, username: &str,
        old: String, new: String) -> Result<(), ServiceError>;
    async fn dashboard_stats(&self, db: &DatabaseConnection)
        -> Result<DashboardStats, ServiceError>;
}
```

**AppState 注入 trait 对象：**

```rust
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub http_client: reqwest::Client,
    pub config: Config,
    // ── Service trait 对象 ──
    pub user_service: Arc<dyn UserService>,
    pub role_service: Arc<dyn RoleService>,
    pub menu_service: Arc<dyn MenuService>,
    pub casbin_service: Arc<dyn CasbinService>,
    pub dictionary_service: Arc<dyn DictionaryService>,
}
```

**Handler 使用：**

```rust
pub async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginDTO>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.login(&state.db, data).await?;
    let token = create_token(&user.username.unwrap_or_default())?;
    Ok(R::ok(LoginResp { token }))
}
```

**渐进式策略 — 首批 5 个核心 Service：**

| 优先级 | Service | 理由 |
|--------|---------|------|
| 1 | `UserService` | 最复杂，含微信登录/绑定 |
| 2 | `CasbinService` | 权限核心 |
| 3 | `MenuService` | 菜单树构建逻辑复杂 |
| 4 | `RoleService` | 角色管理核心 |
| 5 | `DictionaryService` | 代表性的简单 CRUD |

其余 15 个 Service 保持静态方法 + `ServiceError` 返回类型，后续按需添加 trait。遵循 YAGNI——不为暂不测试的 Service 提前定义 trait。

### 6.2 测试套件

**新增目录结构：**

```
tests/
├── common/
│   └── mod.rs                   # 测试辅助：创建 AppState、测试 DB 初始化
├── service/
│   ├── user_service_test.rs     # Service 层单元测试（用真实 DB）
│   └── role_service_test.rs
└── api/
    ├── user_api_test.rs          # API 集成测试
    └── role_api_test.rs
```

#### 6.2.1 测试辅助模块

```rust
// tests/common/mod.rs
pub async fn create_test_state() -> AppState {
    let config = Config::load();  // 使用测试配置（通过环境变量指定测试 DB）
    let db = setup_test_db(&config).await;
    Migrator::fresh(&db).await.unwrap();  // 重建表结构
    let redis = setup_test_redis(&config).await;
    AppState {
        db: db.clone(),
        redis,
        enforcer: setup_test_enforcer(db).await,
        http_client: reqwest::Client::new(),
        config,
        user_service: Arc::new(SysUserService),
        role_service: Arc::new(SysRoleService),
        // ...
    }
}
```

#### 6.2.2 API 集成测试

使用 `tower::ServiceExt::oneshot` 测试，无需绑定端口：

```rust
use tower::ServiceExt;

#[tokio::test]
async fn test_login_success() {
    let state = create_test_state().await;
    insert_test_user(&state.db).await;

    let app = api::protected_routes()
        .merge(api::public_routes())
        .with_state(state);

    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/user/login")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"admin","password":"123456"}"#))
            .unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), 200);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["code"], 200);
    assert!(json["data"]["token"].is_string());
}
```

#### 6.2.3 Service 单元测试（Mock trait）

```rust
#[cfg(test)]
struct MockUserService {
    pub login_result: Result<sys_user::Model, ServiceError>,
}

#[async_trait]
impl UserService for MockUserService {
    async fn login(&self, _: &DatabaseConnection, _: LoginDTO)
        -> Result<sys_user::Model, ServiceError> {
        self.login_result.clone()
    }
    // ... 其他方法返回默认值
}

#[tokio::test]
async fn test_login_handler_with_mock() {
    let mock = MockUserService {
        login_result: Ok(test_user()),
    };
    let state = AppState {
        user_service: Arc::new(mock),
        ..create_test_state().await
    };
    // 测试 handler 逻辑，不触碰真实 DB
}
```

#### 6.2.4 测试数据库策略

**选择方案：独立测试 DB（`scm_test`）**

- 每次测试前 `Migrator::fresh()` 重建表结构
- 测试配置通过 `ADMIN_DB_DATABASE=scm_test` 环境变量指定
- 需要 MySQL 运行（与开发环境共用）
- 真实 MySQL，结果可靠

## 7. 新增依赖汇总

```toml
# Cargo.toml [workspace.dependencies] 追加
tower-http = { version = "0.6", features = ["cors", "trace", "compression-full", "timeout"] }
uuid = { version = "1.8", features = ["v4"] }
redis = { version = "0.27", features = ["tokio-comp", "connection-manager"] }
async-trait = "0.1"  # 已存在
```

## 8. 文件变更清单

| 阶段 | 操作 | 文件 |
|------|------|------|
| P1 | 改造 | `utils/src/config.rs` — 环境变量覆盖 |
| P1 | 新增 | `utils/src/state.rs` — AppState 结构 |
| P1 | 改造 | `utils/src/error.rs` — ServiceError + AppError |
| P1 | 改造 | `utils/src/lib.rs` — 注册 state 模块 |
| P1 | 改造 | `utils/src/prelude.rs` — 导出 AppState、ServiceError |
| P1 | 改造 | `gateway/src/main.rs` — 初始化 AppState、with_state |
| P1 | 改造 | `api/src/lib.rs` — 路由 with_state、health 增强 |
| P1 | 改造 | `api/src/*.rs` — 所有 handler 提取 State、传 db 参数 |
| P1 | 改造 | `service/src/*.rs` — 所有 Service 函数签名变更 |
| P1 | 改造 | `service/src/enforcer.rs` — 移除 set_enforcer 全局注入 |
| P1 | 改造 | `auth-layer/src/middleware.rs` — 适配 State（或保持构造器注入） |
| P2 | 新增 | `utils/src/redis.rs` — Redis 连接管理 |
| P2 | 改造 | `gateway/src/main.rs` — 中间件栈、优雅关闭、Redis 初始化 |
| P2 | 改造 | `auth-layer/src/middleware.rs` — JWT 黑名单走 Redis |
| P2 | 改造 | `service/src/sys_user_service.rs` — reqwest 复用、Dashboard 并行 |
| P2 | 改造 | `service/src/jwt_blacklist_service.rs` — Redis 黑名单写入 |
| P2 | 改造 | `api/src/lib.rs` — 健康检查增强 |
| P2 | 改造 | `Cargo.toml` — 新增依赖 |
| P3 | 新增 | `service/src/traits.rs` — Service trait 定义 |
| P3 | 改造 | `service/src/lib.rs` — 注册 traits 模块 |
| P3 | 改造 | `service/src/sys_user_service.rs` — 实现 UserService trait |
| P3 | 改造 | `service/src/casbin_service.rs` — 实现 CasbinService trait |
| P3 | 改造 | `service/src/sys_menu_service.rs` — 实现 MenuService trait |
| P3 | 改造 | `service/src/sys_role_service.rs` — 实现 RoleService trait |
| P3 | 改造 | `service/src/sys_dictionary_service.rs` — 实现 DictionaryService trait |
| P3 | 改造 | `utils/src/state.rs` — AppState 追加 Service trait 对象 |
| P3 | 改造 | `api/src/*.rs` — handler 通过 state.xxx_service 调用 |
| P3 | 新增 | `tests/common/mod.rs` — 测试辅助 |
| P3 | 新增 | `tests/service/user_service_test.rs` |
| P3 | 新增 | `tests/service/role_service_test.rs` |
| P3 | 新增 | `tests/api/user_api_test.rs` |
| P3 | 新增 | `tests/api/role_api_test.rs` |

## 9. 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| P1 全量签名变更导致编译中断 | 逐 Service 文件迁移，每完成一个确认编译通过 |
| Redis 不可用时服务降级 | JWT 黑名单回退到 DB 查询，缓存 miss 回退到 DB 查询 |
| 测试 DB 与开发 DB 冲突 | 使用独立数据库名 `scm_test` |
| 迁移过程中 `db_conn!()` 和 State 并存 | 过渡期保留宏但标记 deprecated，全部迁移后删除 |
| Service trait 增加复杂度 | 仅对 5 个核心 Service 定义 trait，其余保持静态方法 |

## 10. 验收标准

### P1 验收
- [ ] `cargo build` 全量通过，无 `db_conn!()` 调用残留
- [ ] `ADMIN_DB_HOST=testhost cargo run` 能覆盖 config.yml 中的 host
- [ ] Service 层返回 `Result<T, ServiceError>`，无 `anyhow::Result` 残留
- [ ] API 功能正常（登录、CRUD、权限检查）

### P2 验收
- [ ] Redis 连接成功，健康检查返回 `{"db": true, "redis": true}`
- [ ] 登出后 Redis 中存在 `jwt_blacklist:{token}` 键
- [ ] 受保护请求不再查 MySQL `jwt_blacklists` 表
- [ ] Ctrl+C 触发优雅关闭，日志打印"收到关闭信号"
- [ ] 响应头包含 `Content-Encoding: gzip`（对大响应）
- [ ] 请求超时 30s 后返回 408
- [ ] Dashboard stats 响应时间缩短（4 个查询并行）

### P3 验收
- [ ] 5 个核心 Service trait 定义完成
- [ ] `cargo test` 通过，测试覆盖登录、CRUD、权限
- [ ] Mock 测试可独立运行，不依赖 DB
- [ ] 集成测试使用 `scm_test` 数据库
