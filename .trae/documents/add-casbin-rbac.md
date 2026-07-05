# Plan: 为 Backend 添加 Casbin RBAC 权限控制

## Context

后端项目（Axum 0.8 + SeaORM 1）目前没有认证/授权机制。`gateway/main.rs` 是空壳，无路由、无中间件。已有的 `sys_user`、`sys_role`、`sys_user_role` 表提供了 RBAC 数据基础，但未与 HTTP 请求授权对接。`jsonwebtoken` 已声明但未使用。

参考 `apache/casbin-axum-casbin` 仓库，集成 `axum-casbin` 中间件 + `sea-orm-adapter` 数据库适配器，实现完整的 JWT 认证 + Casbin RBAC 授权。

**已有的迁移文件**（已编译通过）：

| 迁移文件 | 表名 |
|----------|------|
| `m20220101_000001_create_sys_user` | `sys_user` |
| `m20230705_052744_create_sys_role` | `sys_role` |
| `m20230705_053111_create_sys_user_role` | `sys_user_role` |
| `m20240422_075347_create_sys_menu` | `sys_menu` |
| `m20240424_074636_create_sys_menu_role` | `sys_menu_role` |
| `m20250211_071223_create_sys_menu_domain` | `sys_menu_domain` |
| `m20260701_000001_create_casbin_rule` | `casbin_rule` |
| `m20260701_000002_create_jwt_blacklists` | `jwt_blacklists` |
| `m20260701_000003_create_sys_apis` | `sys_apis` |
| `m20260701_000004_create_sys_role_btns` | `sys_role_btns` |
| `m20260701_000005_create_sys_role_menus` | `sys_role_menus` |
| `m20260701_000006_create_sys_base_menu_btns` | `sys_base_menu_btns` |
| `m20260701_000007_create_sys_base_menu_parameters` | `sys_base_menu_parameters` |
| `m20260701_000008_create_sys_data_role_id` | `sys_data_role_id` |
| `m20260701_000009_create_sys_dictionaries` | `sys_dictionaries` |
| `m20260701_000010_create_sys_dictionary_details` | `sys_dictionary_details` |
| `m20260701_000011_create_sys_operation_records` | `sys_operation_records` |

`casbin_rule` 和 `jwt_blacklists` 表的迁移已存在，实施时直接复用，无需 `SeaOrmAdapter` 自建表。

## 依赖版本（已验证兼容）

| Crate | Version | 说明 |
|-------|---------|------|
| `axum-casbin` | 1.3.0 | Apache 官方，axum 0.8，CachedEnforcer |
| `sea-orm-adapter` | 0.4.0 | sea-orm ^1, casbin ^2 |
| `casbin` | 2.10+ | 由 axum-casbin 传递引入 |
| `tower` | 0.5 | 从 0.4 升级（当前未使用，无破坏） |

## 架构设计

请求流经的中间件层（从外到内）：

```
Request → JwtAuthLayer → CasbinAxumLayer → Handler
              ↓                ↓
         提取JWT，插入      读取 CasbinVals，
         CasbinVals 到      enforce(sub,dom,obj,act)
         request.extensions()
```

- 公开路由（`/api/login`、`/health`）不经过任何中间件
- 受保护路由经过 JWT 认证 + Casbin 授权

### Casbin 模型：RBAC with Domains

```ini
r = sub, dom, obj, act      # sub=用户名, dom=域, obj=URL路径, act=HTTP方法
p = sub, dom, obj, act
g = _, _, _                  # 用户-角色-域 分组
m = g(r.sub, p.sub, r.dom) && r.dom == p.dom && keyMatch2(r.obj, p.obj) && regexMatch(r.act, p.act)
```

使用 `keyMatch2` 匹配 URL 参数（如 `/api/users/:id`），`regexMatch` 匹配 HTTP 方法。

## 实施步骤

### Step 1: 更新 Cargo.toml（workspace）

**文件**: `backend/server/Cargo.toml`

- `tower` 改为 `"0.5"`
- 添加：`casbin = "2"`, `axum-casbin = "1.3.0"`, `sea-orm-adapter = "0.4.0"`, `http = "1"`, `http-body-util = "1"`, `futures = "0.3"`

### Step 2: 更新 gateway/Cargo.toml

添加：`axum-casbin`, `sea-orm-adapter`, `jsonwebtoken`, `tower`, `http`, `http-body-util`, `futures`, `service`, `api`, `chrono`, `serde_json`, `serde`

### Step 3: 更新 utils/Cargo.toml

添加：`jsonwebtoken`, `chrono`

### Step 4: 更新 api/Cargo.toml

添加：`axum`, `model`, `service`, `utils`, `serde_json`, `anyhow`

### Step 5: 添加 JWT 配置

**文件**: `backend/server/config.yml` — 添加 `jwt` 节

```yaml
jwt:
  secret: "your-256-bit-secret-key-change-in-production"
  expire_hours: 24
```

**文件**: `backend/server/utils/src/config.rs` — 添加 `JwtConfig` 结构体和字段

### Step 6: 创建 utils/src/auth.rs

JWT 创建/验证工具：

```rust
pub struct Claims { pub sub: String, pub exp: i64, pub iat: i64 }
pub fn create_token(username: &str) -> Result<String>
pub fn verify_token(token: &str) -> Result<Claims>
```

### Step 7: 更新 utils/src/lib.rs + prelude.rs

注册 `auth` 模块，导出 `Claims`, `create_token`, `verify_token`

### Step 8: 更新 utils/src/error.rs

- 添加 `AuthError(String)` 和 `Forbidden(String)` 变体
- 为 `AppError` 实现 `IntoResponse`（目前缺失）

### Step 9: 添加统一响应类型

**文件**: `backend/server/utils/src/res.rs` — 添加 `R<T>` 响应结构体

### Step 10: 创建 Casbin 模型配置

**文件**: `backend/server/casbin/rbac_model.conf` — RBAC with domains 模型

使用 `include_str!` 在编译时嵌入，避免运行时路径问题。

### Step 11: 创建 JWT 认证中间件

**文件**: `backend/server/gateway/src/auth_middleware.rs`

实现 tower `Layer` + `Service`：
1. 从 `Authorization: Bearer <token>` 提取 JWT
2. 验证 JWT，提取 `Claims.sub`
3. 从 URL 路径推导 domain（`/api/users` → `system`）
4. 插入 `CasbinVals { subject, domain }` 到 request extensions

### Step 12: 修复 SysUserService::login

**文件**: `backend/server/service/src/sys_user_service.rs`

当前 `login()` 不验证密码，添加 `PasswordUtils::verify()` 调用。

### Step 13: 修复 LoginDTO 反序列化

**文件**: `backend/server/data/model/src/dto/sys_user_dto.rs`

为 `LoginDTO` 和 `SysUserInsertDTO` 添加 `Deserialize` derive。

### Step 14: 创建 API 路由处理器

**文件**: `backend/server/api/src/lib.rs`

- `POST /api/login` — 登录返回 JWT（公开）
- `GET /api/users` — 用户列表（受保护）
- `GET /api/roles` — 角色列表（受保护）
- `GET /health` — 健康检查（公开）
- `public_routes()` / `protected_routes()` 分开构建

### Step 15: 重写 gateway/src/main.rs

核心编排：
1. 初始化日志 + 迁移（含 `casbin_rule` 表，已有迁移文件）
2. 创建共享 DB 连接
3. 初始化 `SeaOrmAdapter`（表已由迁移创建，adapter 直接读取数据）
4. 加载 Casbin 模型（`include_str!` + `from_str`）
5. 创建 `CasbinAxumLayer`
6. 配置 `keyMatch2` 匹配函数
7. 首次运行时 seed 策略数据
8. 构建路由：public 无中间件，protected 套 `casbin_layer` + `JwtAuthLayer`
9. 启动 Axum 服务器

**注意**：`casbin_rule` 表已由迁移文件 `m20260701_000001_create_casbin_rule.rs` 创建，`SeaOrmAdapter::new()` 内部的 `migration::up()` 因 `if_not_exists` 不会冲突。

### Step 16: Seed 策略数据

在 `main.rs` 中，如果 enforcer 无策略则插入：

```
p, admin, system, /api/users, GET
p, admin, system, /api/users, POST
p, admin, system, /api/users/:id, GET|PUT|DELETE
p, admin, system, /api/roles, GET
p, test, system, /api/users, GET
p, test, system, /api/users/:id, GET
g, admin, admin, system
g, test, test, system
```

## 变更文件清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `server/Cargo.toml` | 修改 | 添加 casbin 等依赖，tower 升级 0.5 |
| `server/gateway/Cargo.toml` | 修改 | 添加 axum-casbin, sea-orm-adapter 等 |
| `server/utils/Cargo.toml` | 修改 | 添加 jsonwebtoken, chrono |
| `server/api/Cargo.toml` | 修改 | 添加 axum, model, service, utils |
| `server/config.yml` | 修改 | 添加 jwt 配置节 |
| `server/utils/src/config.rs` | 修改 | 添加 JwtConfig |
| `server/utils/src/auth.rs` | **新建** | JWT 创建/验证 |
| `server/utils/src/lib.rs` | 修改 | 注册 auth 模块 |
| `server/utils/src/prelude.rs` | 修改 | 导出 auth 工具 |
| `server/utils/src/error.rs` | 修改 | 添加 AuthError/Forbidden + IntoResponse |
| `server/utils/src/res.rs` | 修改 | 添加 R\<T\> 响应结构体 |
| `server/casbin/rbac_model.conf` | **新建** | Casbin RBAC 模型 |
| `server/gateway/src/auth_middleware.rs` | **新建** | JWT 认证 tower layer |
| `server/gateway/src/main.rs` | 修改 | 编排 casbin + auth + routes |
| `server/api/src/lib.rs` | 修改 | 添加 login/protected 路由 |
| `server/data/model/src/dto/sys_user_dto.rs` | 修改 | 添加 Deserialize |
| `server/service/src/sys_user_service.rs` | 修改 | login 添加密码验证 |

**已有的迁移文件**（无需新建）：

| 文件 | 已存在 |
|------|--------|
| `m20260701_000001_create_casbin_rule.rs` | ✅ 已创建并注册 |
| `m20260701_000002_create_jwt_blacklists.rs` | ✅ 已创建并注册 |
| `m20260701_000003_create_sys_apis.rs` | ✅ 已创建并注册 |
| `m20260701_000004-000011` 等其余迁移 | ✅ 已创建并注册 |

## 验证步骤

1. `cd backend/server && cargo build` — 编译通过
2. 启动服务，确认迁移正常运行（`casbin_rule` 等表由 SeaORM 迁移创建）
3. `curl -X POST /api/login -d '{"username":"admin","password":"123456"}'` — 返回 JWT
4. `curl -H "Authorization: Bearer <token>" /api/users` — 200 OK
5. 用 test 用户 token 请求 `/api/roles` — 403 Forbidden
6. 无 token 请求受保护路由 — 401/403
