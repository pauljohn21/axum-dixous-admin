# Casbin 域 (Domain) 扩展设计

> 日期：2026-07-20
> 状态：已批准，待实现

## 1. 背景与目标

### 1.1 问题

当前 Casbin 使用基础 RBAC 模型（`r = sub, obj, act`），所有 API 路由共享同一套权限策略，无法按模块/子系统隔离权限。随着微信小程序端 API 和未来开放 API 的加入，不同入口需要独立的权限策略。

### 1.2 目标

为 Casbin 增加"域"（domain）维度，实现多模块权限隔离：

- **admin** — Web 管理后台（当前所有 API）
- **wechat** — 微信小程序端 API
- **openapi** — 对外开放 API（预留，暂不实现路由）

### 1.3 域判定方式

路由配置中标注——每个 Router 绑定一个域，`AuthLayer` 持有 `domain` 字段，编译期确定，无运行时检测开销。

## 2. Casbin 模型变更

### 2.1 当前模型 (`rbac_model.conf`)

```ini
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && keyMatch2(r.obj, p.obj) && regexMatch(r.act, p.act)
```

### 2.2 变更后模型

```ini
[request_definition]
r = sub, obj, act, dom

[policy_definition]
p = sub, obj, act, dom

[role_definition]
g = _, _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub, r.dom) && keyMatch2(r.obj, p.obj) && regexMatch(r.act, p.act) && r.dom == p.dom
```

### 2.3 关键变化

| 项目 | 当前 | 变更后 |
|------|------|--------|
| `r` (request) | `sub, obj, act` | `sub, obj, act, dom` |
| `p` (policy) | `sub, obj, act` | `sub, obj, act, dom` |
| `g` (role) | `_, _` (2 元) | `_, _, _` (3 元) |
| matcher | `g(r.sub, p.sub)` | `g(r.sub, p.sub, r.dom) && r.dom == p.dom` |

`dom` 放在最后，对现有 `sub, obj, act` 顺序零破坏，仅在末尾追加。

## 3. AuthLayer 中间件变更

### 3.1 结构体变更

```rust
// 当前
pub struct AuthLayer {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub redis: redis::aio::ConnectionManager,
}

// 变更后
pub struct AuthLayer {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub redis: redis::aio::ConnectionManager,
    pub domain: String,  // 新增：域标识
}
```

### 3.2 构造函数

```rust
impl AuthLayer {
    pub fn new(
        enforcer: Arc<RwLock<CachedEnforcer>>,
        redis: redis::aio::ConnectionManager,
        domain: String,
    ) -> Self {
        Self { enforcer, redis, domain }
    }
}
```

### 3.3 enforce 调用

```rust
// 当前：三参数
let args = vec![subject, path, action];
guard.enforce(args)

// 变更后：四参数
let args = vec![subject, path, action, domain];
guard.enforce(args)
```

`domain` 从 `AuthLayer` 持有的字段获取，`AuthMiddleware` 同步增加 `domain` 字段。

## 4. 路由分组与 main.rs 变更

### 4.1 API 层路由拆分

```rust
// 当前
pub fn public_routes() -> Router<AppState> { ... }    // 登录、健康检查
pub fn protected_routes() -> Router<AppState> { ... }  // 所有受保护 API

// 变更后
pub fn public_routes() -> Router<AppState> { ... }     // 不变
pub fn admin_routes() -> Router<AppState> { ... }      // admin 域（原 protected_routes）
pub fn wechat_routes() -> Router<AppState> { ... }     // wechat 域（微信小程序 API）
```

当前所有 API 路由（user/role/menu/api/casbin/dictionary/generator 等）全部归入 `admin_routes()`。

`wechat_routes()` 初始内容：

```rust
pub fn wechat_routes() -> Router<AppState> {
    Router::new()
        .route("/api/wechat/login", post(wx_login))
        .route("/api/wechat/user/info", get(wx_user_info))
        .route("/api/wechat/user/change_password", put(wx_change_password))
}
```

`openapi` 域暂不实现路由，仅预留模型支持。

### 4.2 main.rs 路由组装

```rust
let admin_auth = AuthLayer::new(enforcer.clone(), redis.clone(), "admin".into());
let wechat_auth = AuthLayer::new(enforcer.clone(), redis.clone(), "wechat".into());

let app = Router::new()
    .merge(api::public_routes().with_state(state.clone()))
    .merge(api::swagger_routes())
    .merge(
        api::admin_routes()
            .with_state(state.clone())
            .layer(admin_auth),
    )
    .merge(
        api::wechat_routes()
            .with_state(state.clone())
            .layer(wechat_auth),
    );
```

## 5. 策略数据迁移

### 5.1 迁移方式

开发环境使用 `fresh` 迁移模式（每次重启重建数据库），直接修改初始数据插入语句。

### 5.2 p 策略变更

```rust
// 当前：p, sub, obj, act（4 字段，V3 为空）
.values_panic(["p".into(), "888".into(), "/api/user/list".into(), "GET".into()])

// 变更后：p, sub, obj, act, dom（5 字段，V3 填 "admin"）
.columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2, CasbinRule::V3])
.values_panic(["p".into(), "888".into(), "/api/user/list".into(), "GET".into(), "admin".into()])
```

现有 108 条 `p` 策略全部在 V3 列填 `"admin"`。

### 5.3 g 策略变更

```rust
// 当前：g, user, role（3 字段，V2 为空）
.values_panic(["g".into(), "admin".into(), "888".into()])

// 变更后：g, user, role, dom（4 字段，V2 填 "admin"）
.columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2])
.values_panic(["g".into(), "admin".into(), "888".into(), "admin".into()])
```

现有 4 条 `g` 策略全部在 V2 列填 `"admin"`。

## 6. CasbinService 与 API 变更

### 6.1 DTO 变更

```rust
pub struct CreateCasbinRuleRequest {
    pub ptype: Option<String>,
    pub v0: Option<String>,   // sub
    pub v1: Option<String>,   // obj
    pub v2: Option<String>,   // act
    pub v3: Option<String>,   // dom（已有字段，现正式用于域）
    pub v4: Option<String>,
    pub v5: Option<String>,
}
// UpdateCasbinRuleRequest 同理
```

### 6.2 Service 方法变更

```rust
// update_role_policies 增加 domain 参数
pub async fn update_role_policies(
    db: &DatabaseConnection,
    enforcer: &Arc<RwLock<CachedEnforcer>>,
    role: &str,
    domain: &str,           // 新增
    policies: Vec<(String, String)>,
) -> Result<(), ServiceError>

// get_policy_by_role 增加 domain 过滤
pub async fn get_policy_by_role(
    db: &DatabaseConnection,
    role: &str,
    domain: &str,           // 新增
) -> Result<Vec<casbin_rule::Model>, ServiceError>
```

### 6.3 API 端点变更

```rust
// 按角色+域获取策略
GET /api/casbin/role/{role}/domain/{domain}

// 更新角色+域的策略
PUT /api/casbin/role/{role}/domain/{domain}
```

## 7. 前端变更

### 7.1 权限管理页面 (`casbin_manage.rs`)

- 表格新增"域"列，显示策略的 domain 字段
- 新增/编辑对话框增加"域"输入框，默认值 `admin`
- 搜索支持按域过滤

### 7.2 前端模型 (`casbin_rule.rs`)

已有 `v3` 字段，前端展示时映射为"域"。

### 7.3 前端 API (`casbin.rs`)

`update_role_policies` 和 `get_policies_by_role` 调用增加 domain 参数。

### 7.4 i18n

新增翻译 key：`RuleDomain`、`RuleDomainPlaceholder` 等。

## 8. 影响范围

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `casbin/rbac_model.conf` | 修改 | r/p 增加 dom，g 变 3 元 |
| `auth-layer/src/middleware.rs` | 修改 | AuthLayer 增加 domain 字段，enforce 四参数 |
| `api/src/lib.rs` | 修改 | `protected_routes()` 拆分为 `admin_routes()` + `wechat_routes()` |
| `gateway/src/main.rs` | 修改 | 两个 AuthLayer 实例，分别绑定 admin/wechat 域 |
| `data/migration/.../casbin_rule.rs` | 修改 | p 策略 V3 填 admin，g 策略 V2 填 admin |
| `service/src/casbin_service.rs` | 修改 | DTO 增加 dom，方法增加 domain 参数 |
| `api/src/casbin_api.rs` | 修改 | role 策略接口增加 domain 路径参数 |
| `web/src/components/casbin_manage.rs` | 修改 | 表格增加域列，对话框增加域输入 |
| `web/src/models/casbin_rule.rs` | 微调 | 注释标注 v3 为域字段 |
| `web/src/api/casbin.rs` | 修改 | API 调用适配 domain 参数 |
| `web/src/i18n/mod.rs` | 修改 | 新增域相关翻译 key |

## 9. 不变的部分

- JWT 机制（创建/验证/黑名单）
- Redis 缓存逻辑
- CorsLayer / CompressionLayer / TimeoutLayer / TraceLayer / GovernorLayer
- 前端路由守卫
- 数据库表结构（casbin_rule 表已有 V0-V5 列，无需 DDL 变更）
- 其他 Service（UserService / RoleService / MenuService 等）

## 10. 测试策略

- 后端编译通过 (`cargo check`)
- 后端 clippy 零 warning (`cargo clippy --all-targets -- -D warnings`)
- 前端编译通过 (`cargo check`)
- 手动验证：admin 用户登录后侧边栏显示"权限管理"，页面正常加载 112 条策略（域列显示 admin）
- 手动验证：enforce 正常放行 admin 域请求
