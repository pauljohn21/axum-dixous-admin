# axum-dixous-admin 后端服务

> [English](./README_EN.md) | 简体中文

基于 Axum + SeaORM + Casbin + MySQL 的 Rust 后台管理系统后端，提供用户管理、角色管理、菜单管理、权限控制、字典管理等完整功能。

---

## 目录

- [技术栈](#技术栈)
- [项目结构](#项目结构)
- [快速开始](#快速开始)
- [配置说明](#配置说明)
- [架构设计](#架构设计)
- [关键约定](#关键约定)
- [API 模块](#api-模块)
- [数据库迁移](#数据库迁移)
- [添加新模块](#添加新模块)
- [开发指南](#开发指南)
- [常用命令](#常用命令)

---

## 技术栈

| 层级 | 技术 | 版本 | 说明 |
|------|------|------|------|
| Web 框架 | Axum | 0.8 | 异步 HTTP 框架，基于 Tower |
| ORM | SeaORM | 1 | 异步 Rust ORM |
| 数据库 | MySQL | 8 | 关系型数据库 |
| 缓存 | Redis | 7 | 内存缓存（可选） |
| 权限控制 | Casbin | 2 | RBAC 模型，支持 keyMatch2 路径匹配 |
| JWT | jsonwebtoken | 9 | 身份认证令牌 |
| API 文档 | utoipa + Swagger UI | 5 / 9 | OpenAPI 3.0 自动生成 |
| 密码加密 | Argon2 | 0.5 | 安全密码哈希 |
| 运行时 | Tokio | 1 | 异步运行时 |

---

## 项目结构

```
backend/
├── Cargo.toml              # 工作空间根配置
├── compose.yml             # Docker Compose (MySQL 8 + Redis 7 + Adminer)
├── mysql.sql               # 数据库初始 SQL (参考数据)
├── data/                   # Docker 持久化数据 (MySQL / Redis)
└── server/                 # 后端服务工作空间
    ├── Cargo.toml          # 工作空间配置 (workspace members)
    ├── config.yml          # 后端配置文件 (编译时 include_str! 内嵌)
    ├── Dockerfile          # Docker 构建文件
    ├── casbin/
    │   └── rbac_model.conf # Casbin RBAC 模型定义
    ├── gateway/            # 二进制入口 (main.rs)
    ├── api/                # HTTP 路由 + OpenAPI 注解
    ├── service/            # 业务逻辑层
    ├── auth-layer/         # JWT 验证 + Casbin 鉴权中间件
    ├── casbin-adapter/     # SeaORM 实现的 Casbin Adapter
    ├── data/
    │   ├── migration/      # SeaORM 数据迁移脚本
    │   └── model/          # 实体 (dao) + DTO (dto)
    ├── utils/              # 配置、DB 连接、JWT、密码、日志等工具
    └── shell/              # sea-orm-cli 辅助脚本
```

### 工作空间成员

| Crate | 路径 | 职责 |
|-------|------|------|
| `gateway` | `server/gateway/` | 程序入口，组装路由、中间件，启动 HTTP 服务 |
| `api` | `server/api/` | HTTP 路由定义、请求处理、OpenAPI 注解 |
| `service` | `server/service/` | 业务逻辑层，调用 model + utils |
| `model` | `server/data/model/` | 数据层：`dao/` (SeaORM 实体) + `dto/` (数据传输对象) |
| `migration` | `server/data/migration/` | SeaORM 数据库迁移脚本 |
| `auth-layer` | `server/auth-layer/` | JWT 验证 + Casbin 权限检查中间件 |
| `casbin-adapter` | `server/casbin-adapter/` | 基于 SeaORM 的 Casbin Adapter 实现 |
| `utils` | `server/utils/` | 配置读取、数据库连接、JWT 生成/验证、密码加密、日志初始化 |

---

## 快速开始

### 环境要求

- **Rust** >= 1.84
- **Docker** + **Docker Compose**（用于 MySQL / Redis）
- **sea-orm-cli**（可选，用于生成实体和迁移）

```bash
# 安装 sea-orm-cli（可选）
cd backend/server/shell && sh install_sea_orm_cli.sh
```

### 1. 启动基础设施

```bash
cd backend
docker compose up -d
```

启动后可访问：

| 服务 | 地址 | 凭据 |
|------|------|------|
| MySQL | `localhost:3306` | root / root123456 / scm |
| Redis | `localhost:6379` | 无密码 |
| Adminer | http://localhost:8090 | - |

### 2. 启动后端服务

```bash
cd backend
cargo run
```

服务默认监听 `0.0.0.0:8888`，启动后可访问：

- **Swagger UI**: http://localhost:8888/
- **OpenAPI JSON**: http://localhost:8888/openapi.json
- **健康检查**: http://localhost:8888/health

### 3. Docker 部署

```bash
cd backend
docker build -t axum-admin-backend ./server
docker run -p 8888:8888 axum-admin-backend
```

> **注意**：Docker 容器内需能访问 MySQL，请根据部署环境修改 `config.yml` 中的数据库地址。

---

## 配置说明

配置文件位于 `server/config.yml`，在编译时通过 `include_str!` 内嵌到二进制中，运行时使用 `once_cell::Lazy` 全局持有。

```yaml
server:
  host: 0.0.0.0
  port: 8888

datasource:
  driver: mysql
  host: localhost
  port: 3306
  database: scm
  username: root
  password: root123456
  config:
    max_conn: 100
    min_conn: 5
    connect_timeout: 5
    acquire_timeout: 5
    max_lifetime: 100
    idle_timeout: 100
    sqlx_level: info
  migration: fresh          # fresh / up / down / reset

cache:
  driver: redis
  host: 0.0.0.0
  port: 6379
  password:

logger:
  level: info               # debug / info / warn / error / trace

jwt:
  secret: "your-256-bit-secret-key-change-in-production"
  expire_hours: 24
```

在代码中通过 `utils::prelude::CONFIG` 访问配置：

```rust
use utils::prelude::CONFIG;

let host = &CONFIG.datasource.host;
let port = CONFIG.server.port;
```

### 数据迁移模式

`config.yml` 中的 `migration` 字段控制启动时的迁移行为：

| 值 | 说明 |
|----|------|
| `fresh` | 删除所有表并重新创建（开发环境推荐） |
| `up` | 执行未应用的迁移 |
| `down` | 回滚最近一次迁移 |
| `reset` | 回滚所有迁移再重新执行 |

---

## 架构设计

### 分层架构

```
请求 → AuthLayer 中间件 → API 层 → Service 层 → Model 层 → 数据库
         │                    │          │           │
         ├─ JWT 验证           ├─ 路由     ├─ 业务逻辑  ├─ DAO (SeaORM 实体)
         ├─ JWT 黑名单检查      ├─ 请求解析  └─ 事务管理  └─ DTO (数据传输对象)
         └─ Casbin 权限检查    └─ 响应封装
```

### 启动流程

`gateway/src/main.rs` 中的启动流程：

1. **初始化日志** — `Level::init()` 根据 `config.yml` 配置日志级别
2. **执行数据迁移** — `Migrator::migration_init()` 根据 `migration` 配置执行迁移
3. **建立数据库连接** — `DB::db_connection()` 创建 SeaORM 连接池
4. **初始化 Casbin** — 加载 RBAC 模型 + SeaORM Adapter → 创建 `CachedEnforcer`
5. **注入 Enforcer** — 将 Enforcer 注入 service 层，用于策略修改后刷新缓存
6. **创建 AuthLayer** — 封装 JWT 验证 + Casbin 权限检查中间件
7. **组装路由** — 公开路由 + Swagger UI + 受保护路由（含 AuthLayer）+ CORS
8. **启动服务** — 绑定地址并开始监听

### 鉴权流程

```
客户端请求 (带 Authorization: Bearer {token})
  │
  ▼
AuthLayer 中间件
  │
  ├─ 1. 提取 JWT Token → verify_token() 验证
  │     失败 → 401 Unauthorized
  │
  ├─ 2. JWT 黑名单检查 (jwt_blacklists 表)
  │     命中黑名单 → 401 Unauthorized
  │
  ├─ 3. 注入 Username 到 request extension
  │
  ├─ 4. Casbin enforce(sub=user, obj=path, act=method)
  │     拒绝 → 403 Forbidden
  │     通过 → 放行到下游 handler
  │
  ▼
API Handler (可通过 Extension<Username> 获取当前用户)
```

### Casbin RBAC 模型

模型定义在 `server/casbin/rbac_model.conf`：

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

- **sub**：用户名
- **obj**：请求路径（支持 `keyMatch2` 模式匹配，如 `/api/user/{id}`）
- **act**：HTTP 方法（支持正则匹配，如 `GET|POST`）

---

## 关键约定

### 数据库连接

使用 `db_conn!()` 宏获取数据库连接：

```rust
let db = db_conn!();
// 宏展开为: &utils::prelude::DB::db_connection().await
```

### 统一响应格式

所有 API 返回 `R<T>` 结构：

```json
{
  "code": 200,
  "message": "success",
  "data": { ... }
}
```

```rust
// 成功
R::ok(data)           // code = 200

// 失败
R::fail(400, "参数错误")
```

### 错误处理

使用 `AppError` 枚举统一错误类型，自动转换为 HTTP 状态码：

```rust
pub enum AppError {
    Anyhow(anyhow::Error),       // → 500
    DbErr(sea_orm::DbErr),       // → 500
    AppError(axum::Error),       // → 500
    AuthError(String),           // → 401
    Forbidden(String),           // → 403
    NotFoundError(String),       // → 404
}
```

### 分页

```rust
// 请求参数
pub struct PageRequest {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}

// 响应结构
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
```

### 密码加密

使用 Argon2 算法进行密码哈希：

```rust
// 加密
let hash = PasswordUtils::encrypt("plaintext_password");
// hash.password_hash → 存储到数据库
// hash.salt → 存储到数据库

// 验证
PasswordUtils::verify("input_password", &stored_hash, &stored_salt)?;
```

### JWT 令牌

```rust
// 生成
let token = create_token("username")?;

// 验证
let claims = verify_token(&token)?;
// claims.sub → 用户名
// claims.exp → 过期时间
// claims.iat → 签发时间
```

---

## API 模块

系统内置以下 API 模块，所有接口均通过 Swagger UI 文档查看：

| 模块 | 路由前缀 | 说明 |
|------|----------|------|
| 用户管理 | `/api/user` | 登录、注册、用户 CRUD、修改密码、用户信息 |
| 角色管理 | `/api/role` | 角色 CRUD |
| 菜单管理 | `/api/menu` | 菜单 CRUD |
| API 管理 | `/api/api` | API 接口管理 |
| Casbin 策略 | `/api/casbin` | 权限策略管理、角色策略分配 |
| JWT 管理 | `/api/jwt` | JWT 黑名单管理 |
| 菜单按钮 | `/api/menu_btn` | 菜单按钮管理 |
| 菜单参数 | `/api/menu_param` | 菜单路由参数 |
| 角色按钮 | `/api/role_btn` | 角色按钮权限 |
| 角色菜单 | `/api/role_menu` | 角色菜单权限 |
| 数据权限 | `/api/data_role` | 角色数据权限 |
| 字典管理 | `/api/dictionary` | 系统字典 CRUD |
| 字典详情 | `/api/dictionary_detail` | 字典项 CRUD |
| 操作记录 | `/api/operation_record` | 操作日志管理 |
| 仪表盘 | `/api/dashboard` | 统计数据 |

### 公开路由

以下路由不需要鉴权：

- `POST /api/user/login` — 用户登录
- `GET /health` — 健康检查

### 路由规范

- 公开路由：`/api/user/login`, `/health`
- 受保护路由：`/api/{resource}` (CRUD)
- 资源操作：`/api/{resource}/{id}` (GET / PUT / DELETE)

---

## 数据库迁移

### 迁移脚本

迁移脚本位于 `server/data/migration/src/`，按时间戳命名：

```
m20220101_000001_create_sys_user.rs
m20230705_052744_create_sys_role.rs
m20230705_053111_create_sys_user_role.rs
m20240422_075347_create_sys_menu.rs
m20240423_112033_create_sys_apis.rs
m20240424_074636_create_sys_menu_role.rs
m20250211_071223_create_sys_menu_domain.rs
m20260701_000001_create_casbin_rule.rs
m20260701_000002_create_jwt_blacklists.rs
```

### 辅助脚本

```bash
# 新建迁移
cd backend/server/shell && sh migrate_table.sh

# 生成实体 (从数据库反向生成 SeaORM 实体)
cd backend/server/shell && sh gen_entity.sh

# 安装 sea-orm-cli
cd backend/server/shell && sh install_sea_orm_cli.sh
```

> **注意**：`gen_entity.sh` 中的数据库连接信息需与 `config.yml` 一致。

---

## 添加新模块

以添加 `xxx` 模块为例：

### 1. 创建迁移

```bash
cd backend/server/shell
# 修改 migrate_table.sh 中的表名后执行
sh migrate_table.sh
```

在生成的迁移文件中定义表结构，并在 `migration/src/lib.rs` 中注册：

```rust
mod mXXXXXX_create_xxx;
// ...
vec![Box::new(mXXXXXX_create_xxx::Migration)]
```

### 2. 生成实体

```bash
cd backend/server/shell && sh gen_entity.sh
```

或手动在 `model/src/dao/` 创建实体文件。

### 3. 创建 DTO

在 `model/src/dto/` 添加 `xxx_dto.rs`：

```rust
#[derive(Deserialize, ToSchema)]
pub struct XxxInsertDTO { ... }

#[derive(Deserialize, ToSchema)]
pub struct XxxUpdateDTO { ... }

#[derive(Deserialize, ToSchema)]
pub struct XxxQueryDTO { ... }
```

在 `model/src/dto/mod.rs` 中注册：`pub mod xxx_dto;`

### 4. 创建 Service

在 `service/src/` 添加 `xxx_service.rs`，实现 CRUD 业务逻辑：

```rust
pub struct XxxService;

impl XxxService {
    pub async fn insert(data: XxxInsertDTO) -> Result<()> { ... }
    pub async fn list(query: PageRequest) -> Result<PageResponse<xxx::Model>> { ... }
    pub async fn get_by_id(id: i32) -> Result<xxx::Model> { ... }
    pub async fn update(id: i32, data: XxxUpdateDTO) -> Result<xxx::Model> { ... }
    pub async fn delete(id: i32) -> Result<()> { ... }
}
```

在 `service/src/lib.rs` 中注册：`pub mod xxx_service;`

### 5. 创建 API

在 `api/src/` 添加 `xxx_api.rs`，定义路由和 OpenAPI 注解：

```rust
#[utoipa::path(post, path = "/api/xxx", ...)]
pub async fn create(...) -> Result<impl IntoResponse, AppError> { ... }

pub fn routes() -> Router {
    Router::new()
        .route("/api/xxx/list", get(list))
        .route("/api/xxx/{id}", get(get_by_id).put(update).delete(delete))
}
```

### 6. 注册模块

在 `api/src/lib.rs` 中：

- `pub mod xxx_api;` — 声明模块
- 在 `protected_routes()` 中 `.merge(xxx_api::routes())` — 合并路由
- 在 `ApiDoc` 的 `paths()` 中注册接口
- 在 `components(schemas(...))` 中注册 DTO

---

## 开发指南

### 编译优化

`Cargo.toml` 中配置了 release 优化：

```toml
[profile.release]
codegen-units = 1
debug = false
lto = true
opt-level = "z"       # 优化二进制体积
panic = 'abort'
```

### 编码规范

- Rust edition 2021
- 中文注释，模块级使用 `//!` 文档注释
- 错误处理：`anyhow::Result` + `AppError`
- 序列化：`serde` derive
- 命名：snake_case

### 日志

使用 `tracing` 进行日志记录，级别由 `config.yml` 的 `logger.level` 控制：

```rust
tracing::info!("信息日志");
tracing::error!("错误日志: {}", e);
```

---

## 常用命令

```bash
# 启动开发服务
cd backend && cargo run

# 构建 release
cd backend && cargo build --release

# 检查编译
cd backend && cargo check

# 运行测试
cd backend && cargo test

# 启动基础设施
cd backend && docker compose up -d

# 查看基础设施日志
cd backend && docker compose logs -f mysql

# 停止基础设施
cd backend && docker compose down
```
