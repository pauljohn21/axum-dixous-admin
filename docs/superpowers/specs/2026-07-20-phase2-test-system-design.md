# Phase 2 — 测试体系详细设计

> 日期：2026-07-20
> 状态：📋 设计完成，待实施
> 前置条件：Phase 1 已完成（代码整洁、全局状态清除、依赖注入就绪）

## 1. 现状分析

### 1.1 现有测试

| 文件 | 测试数 | 类型 | 说明 |
|------|--------|------|------|
| `backend/tests/api_tests.rs` | 7 | Mock + 单元 | MockUserService + ServiceError→AppError 转换 + 配置验证 |
| `backend/server/utils/tests/test_password_utils.rs` | 2 | 单元 | 密码加密/验证 |
| **合计** | **9** | — | 全部不碰真实 DB |

### 1.2 测试基础设施缺口

- **无测试辅助模块**：每个测试自行构造数据，无 fixtures
- **无集成测试**：Service 层函数已接受 `db: &DatabaseConnection` 参数（Phase 1 成果），但无真实 DB 测试
- **无 HTTP 路由测试**：`api::public_routes()` / `api::protected_routes()` 返回 `Router<AppState>`，可用 `tower::ServiceExt::oneshot` 测试，但无 AppState 构建辅助
- **无 Mock 扩展**：仅 UserService 有 Mock，Role/Menu/Api Service 无 Mock

### 1.3 Phase 1 成果使测试成为可能

| Phase 1 改造 | 对测试的影响 |
|--------------|-------------|
| Service 函数接收 `db: &DatabaseConnection` | 可注入测试 DB 连接 |
| AuthLayer 构造器注入 Redis | 可注入测试 Redis |
| Casbin enforcer 参数化 | 可构造测试 enforcer |
| 微信配置参数化 | 可注入 mock 配置 |
| `db_conn!()` 宏删除 | 无全局状态干扰 |

## 2. 测试分层

```
L1 单元测试     — 纯函数，无 IO 依赖
L2 Mock 测试    — Trait + Mock 实现，不碰 DB/Redis
L3 集成测试     — 真实 MySQL + Redis，Service 层调用
L4 HTTP 路由测试 — Router::oneshot(request)，完整中间件链
```

> **不做 E2E 测试。** L4 的 `tower::ServiceExt::oneshot` 在内存中模拟 HTTP 请求，不启动真实服务器，属于 HTTP 层集成测试。

## 3. 测试目录结构

```
backend/
├── tests/
│   ├── api_tests.rs              # 现有 Mock 测试（保留，扩展）
│   ├── common/
│   │   └── mod.rs                # 测试辅助：AppState 构建、DB 初始化、fixtures
│   ├── integration/
│   │   ├── mod.rs                # 模块声明
│   │   ├── user_service_test.rs  # 真实 DB Service 测试
│   │   ├── role_service_test.rs
│   │   └── casbin_test.rs        # 权限规则测试
│   └── api/
│       ├── mod.rs                # 模块声明
│       ├── auth_api_test.rs      # 登录/登出/JWT 黑名单路由测试
│       └── user_api_test.rs      # 用户 CRUD 路由测试
└── server/
    └── utils/
        └── tests/
            └── test_password_utils.rs  # 现有单元测试（保留）
```

## 4. 测试 DB 策略

### 4.1 独立测试数据库

- 数据库名：`scm_test`（与开发库 `scm` 隔离）
- 通过环境变量 `ADMIN_DB_DATABASE=scm_test` 覆盖
- 与开发环境共用同一 MySQL 实例（`localhost:3306`）

### 4.2 数据初始化

每个集成测试函数前：
1. 连接 `scm_test` 数据库
2. `Migrator::fresh()` — drop 所有表 + 重新执行全部迁移
3. 插入种子数据（测试用户、角色等）

### 4.3 Redis 策略

- 复用开发环境 Redis（`localhost:6379`）
- 测试前 `FLUSHDB` 清空（避免 JWT 黑名单等残留）
- 或使用独立 DB index（`REDIS_URL=redis://localhost:6379/1`）

### 4.4 测试隔离

- 每个集成测试使用 `#[tokio::test]` 独立运行
- `Migrator::fresh()` 确保每个测试看到干净的数据库状态
- 测试间无数据依赖

## 5. 测试辅助模块 (`tests/common/mod.rs`)

```rust
//! 测试辅助模块

use std::sync::Arc;
use casbin::{CachedEnforcer, CoreApi};
use migration::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::sync::RwLock;
use utils::prelude::{AppState, CONFIG};

/// 创建测试数据库连接（scm_test）
pub async fn setup_test_db() -> DatabaseConnection {
    // 环境变量覆盖 → scm_test
    std::env::set_var("ADMIN_DB_DATABASE", "scm_test");
    let db_url = format!(
        "mysql://{}:{}@{}:{}/scm_test",
        CONFIG.datasource.username,
        CONFIG.datasource.password,
        CONFIG.datasource.host,
        CONFIG.datasource.port,
    );
    let db = Database::connect(db_url).await.expect("连接测试 DB 失败");
    Migrator::fresh(&db).await.expect("迁移失败");
    db
}

/// 创建测试 Redis 连接
pub async fn setup_test_redis() -> redis::aio::ConnectionManager {
    let url = format!("redis://{}:{}/1", CONFIG.cache.host, CONFIG.cache.port);
    let client = redis::Client::open(url).expect("创建 Redis 客户端失败");
    let mut conn = client.get_connection_manager().await.expect("连接 Redis 失败");
    redis::cmd("FLUSHDB").query_async(&mut conn).await.ok();
    conn
}

/// 创建测试 Casbin Enforcer
pub async fn setup_test_enforcer(db: &DatabaseConnection) -> Arc<RwLock<CachedEnforcer>> {
    let model_str = include_str!("../../../server/casbin/rbac_model.conf");
    let model = casbin::DefaultModel::from_str(model_str).await.unwrap();
    let adapter = casbin_adapter::SeaOrmAdapter::new(db.clone()).await.unwrap();
    Arc::new(RwLock::new(CachedEnforcer::new(model, adapter).await.unwrap()))
}

/// 构建完整测试 AppState
pub async fn setup_test_state() -> AppState {
    let db = setup_test_db().await;
    let enforcer = setup_test_enforcer(&db).await;
    let redis = setup_test_redis().await;
    let http_client = reqwest::Client::new();
    AppState {
        db,
        enforcer,
        http_client,
        config: CONFIG.clone(),
        redis,
    }
}

/// 插入测试用户并返回 (id, username)
pub async fn insert_test_user(db: &DatabaseConnection) -> (i32, String) {
    use model::dao::sys_user;
    use sea_orm::Set;
    use utils::prelude::PasswordUtils;

    let hash = PasswordUtils::encrypt("123456");
    let active = sys_user::ActiveModel {
        username: Set(Some("testuser".into())),
        password: Set(Some(hash.password_hash)),
        salt: Set(Some(hash.salt)),
        nick_name: Set(Some("测试用户".into())),
        enable: Set(Some(1)),
        ..Default::default()
    };
    let result = sys_user::Entity::insert(active).exec(db).await.unwrap();
    (result.last_insert_id as i32, "testuser".to_string())
}
```

## 6. 测试用例清单

### 6.1 L2 Mock 测试扩展（`api_tests.rs`）

| 测试名 | 验证内容 |
|--------|----------|
| `test_mock_role_service_crud` | RoleService Trait Mock 的增删改查 |
| `test_mock_menu_service_crud` | MenuService Trait Mock 的增删改查 |
| `test_mock_api_service_crud` | ApiService Trait Mock 的增删改查 |

### 6.2 L3 集成测试

**`integration/user_service_test.rs`：**

| 测试名 | 验证内容 |
|--------|----------|
| `test_user_login_success` | 正确密码登录返回用户 |
| `test_user_login_wrong_password` | 错误密码返回 InvalidPassword |
| `test_user_login_disabled` | 禁用用户登录 |
| `test_user_crud_lifecycle` | insert → get_by_id → update → delete 完整生命周期 |
| `test_user_list_with_keyword` | 关键词搜索分页 |
| `test_user_change_password` | 修改密码后可用新密码登录 |
| `test_user_dashboard_stats` | 仪表盘统计数据正确 |

**`integration/role_service_test.rs`：**

| 测试名 | 验证内容 |
|--------|----------|
| `test_role_crud_lifecycle` | insert → get_by_id → update → delete |
| `test_role_delete_cleans_associations` | 删除角色时清理 user_role + role_menus + role_btns + casbin 策略 |

**`integration/casbin_test.rs`：**

| 测试名 | 验证内容 |
|--------|----------|
| `test_casbin_enforce_allow` | 有权限的路由通过 |
| `test_casbin_enforce_deny` | 无权限的路由拒绝 |
| `test_casbin_reload_policy` | 策略修改后 reload 生效 |

### 6.3 L4 HTTP 路由测试

**`api/auth_api_test.rs`：**

| 测试名 | 验证内容 |
|--------|----------|
| `test_login_route_success` | POST /api/user/login 返回 200 + token |
| `test_login_route_wrong_password` | 错误密码返回 401 |
| `test_health_route` | GET /health 返回 200 |

**`api/user_api_test.rs`：**

| 测试名 | 验证内容 |
|--------|----------|
| `test_get_user_info_route` | GET /api/user/info 带 JWT 返回用户信息 |
| `test_get_user_info_no_token` | 无 token 返回 401 |
| `test_user_list_route` | GET /api/user 返回分页列表 |

### 6.4 前端纯函数测试

在 `web/src/http/mod.rs` 和 `web/src/models/` 中添加 `#[cfg(test)]` 模块：

| 测试名 | 文件 | 验证内容 |
|--------|------|----------|
| `test_build_page_query` | `http/mod.rs` | 分页参数构建 URL query |
| `test_dto_serde` | `models/generator.rs` | GeneratorConfig 序列化/反序列化 |

## 7. HTTP 路由测试模式

使用 `tower::ServiceExt::oneshot` 对 `Router` 发送请求，不启动真实 HTTP 服务器：

```rust
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_login_route_success() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    let app = api::public_routes().with_state(state);

    let body = serde_json::json!({
        "username": "testuser",
        "password": "123456"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/user/login")
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

**受保护路由测试**：需要先登录获取 token，或者直接在 `Request` 的 `Authorization` 头中放入 JWT：

```rust
let token = create_token("testuser"); // 工具函数
let request = Request::builder()
    .method("GET")
    .uri("/api/user/info")
    .header("Authorization", format!("Bearer {}", token))
    .body(Body::empty())
    .unwrap();
```

## 8. 依赖变更

### 8.1 后端 dev-dependencies

```toml
[dev-dependencies]
async-trait = { workspace = true }
tower = { workspace = true, features = ["util"] }  # 追加 util feature
service = { path = "server/service" }
# 新增
http-body-util = { workspace = true }
```

### 8.2 前端

无新增依赖。`#[cfg(test)]` 模块使用 `serde_json`（已有）。

## 9. 运行方式

```bash
# 全量测试（需要 MySQL + Redis 运行）
cd backend && cargo test

# 仅 Mock 测试（不需要 DB）
cargo test --test api_tests

# 仅集成测试
cargo test --test integration

# 仅 HTTP 路由测试
cargo test --test api

# 前端测试
cd web && cargo test
```

## 10. 验收标准

- [ ] 后端测试数 > 20（现有 9 + 新增 12+）
- [ ] 集成测试覆盖 User/Role CRUD 核心路径
- [ ] Casbin 权限规则测试通过
- [ ] HTTP 路由测试覆盖登录 + 用户信息获取
- [ ] Mock 测试扩展到 4 个 Service
- [ ] 前端纯函数测试覆盖 HTTP 层和 Model 序列化
- [ ] `cargo test` 全量通过
- [ ] `cargo clippy` 零 warning
