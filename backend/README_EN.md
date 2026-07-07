# axum-dixous-admin Backend Service

> English | [简体中文](./README.md)

A Rust backend service for an admin management system built with Axum + SeaORM + Casbin + MySQL, providing user management, role management, menu management, access control, dictionary management, and more.

---

## Table of Contents

- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Key Conventions](#key-conventions)
- [API Modules](#api-modules)
- [Database Migration](#database-migration)
- [Adding a New Module](#adding-a-new-module)
- [Development Guide](#development-guide)
- [Common Commands](#common-commands)

---

## Tech Stack

| Layer | Technology | Version | Description |
|-------|------------|---------|-------------|
| Web Framework | Axum | 0.8 | Async HTTP framework based on Tower |
| ORM | SeaORM | 1 | Async Rust ORM |
| Database | MySQL | 8 | Relational database |
| Cache | Redis | 7 | In-memory cache (optional) |
| Access Control | Casbin | 2 | RBAC model with keyMatch2 path matching |
| JWT | jsonwebtoken | 9 | Authentication token |
| API Docs | utoipa + Swagger UI | 5 / 9 | OpenAPI 3.0 auto-generation |
| Password Hashing | Argon2 | 0.5 | Secure password hashing |
| Runtime | Tokio | 1 | Async runtime |

---

## Project Structure

```
backend/
├── Cargo.toml              # Workspace root config
├── compose.yml             # Docker Compose (MySQL 8 + Redis 7 + Adminer)
├── mysql.sql               # Initial SQL (reference data)
├── data/                   # Docker persistent data (MySQL / Redis)
└── server/                 # Backend service workspace
    ├── Cargo.toml          # Workspace config (workspace members)
    ├── config.yml          # Backend config (compiled via include_str!)
    ├── Dockerfile          # Docker build file
    ├── casbin/
    │   └── rbac_model.conf # Casbin RBAC model definition
    ├── gateway/            # Binary entry point (main.rs)
    ├── api/                # HTTP routes + OpenAPI annotations
    ├── service/            # Business logic layer
    ├── auth-layer/         # JWT + Casbin auth middleware
    ├── casbin-adapter/     # SeaORM-based Casbin Adapter
    ├── data/
    │   ├── migration/      # SeaORM migration scripts
    │   └── model/          # Entities (dao) + DTOs (dto)
    ├── utils/              # Config, DB, JWT, password, logging utils
    └── shell/              # sea-orm-cli helper scripts
```

### Workspace Members

| Crate | Path | Responsibility |
|-------|------|----------------|
| `gateway` | `server/gateway/` | Entry point, assembles routes & middleware, starts HTTP server |
| `api` | `server/api/` | HTTP route definitions, request handling, OpenAPI annotations |
| `service` | `server/service/` | Business logic layer, calls model + utils |
| `model` | `server/data/model/` | Data layer: `dao/` (entities) + `dto/` (DTOs) |
| `migration` | `server/data/migration/` | SeaORM database migration scripts |
| `auth-layer` | `server/auth-layer/` | JWT verification + Casbin authorization middleware |
| `casbin-adapter` | `server/casbin-adapter/` | SeaORM-based Casbin Adapter implementation |
| `utils` | `server/utils/` | Config, DB connection, JWT, password hashing, logging |

---

## Quick Start

### Prerequisites

- **Rust** >= 1.84
- **Docker** + **Docker Compose** (for MySQL & Redis)
- **sea-orm-cli** (optional, for generating entities and migrations)

```bash
# Install sea-orm-cli (optional)
cd backend/server/shell && sh install_sea_orm_cli.sh
```

### 1. Start Infrastructure

```bash
cd backend
docker compose up -d
```

Available services after startup:

| Service | Address | Credentials |
|---------|---------|-------------|
| MySQL | `localhost:3306` | root / root123456 / scm |
| Redis | `localhost:6379` | No password |
| Adminer | http://localhost:8090 | - |

### 2. Start Backend Service

```bash
cd backend
cargo run
```

The server listens on `0.0.0.0:8888` by default. Available endpoints:

- **Swagger UI**: http://localhost:8888/
- **OpenAPI JSON**: http://localhost:8888/openapi.json
- **Health Check**: http://localhost:8888/health

### 3. Docker Deployment

```bash
cd backend
docker build -t axum-admin-backend ./server
docker run -p 8888:8888 axum-admin-backend
```

> **Note**: The Docker container must be able to reach MySQL. Please modify the database address in `config.yml` according to your deployment environment.

---

## Configuration

The configuration file is located at `server/config.yml`. It is embedded into the binary at compile time via `include_str!` and held globally at runtime using `once_cell::Lazy`.

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

Access configuration in code via `utils::prelude::CONFIG`:

```rust
use utils::prelude::CONFIG;

let host = &CONFIG.datasource.host;
let port = CONFIG.server.port;
```

### Migration Modes

The `migration` field in `config.yml` controls migration behavior on startup:

| Value | Description |
|-------|-------------|
| `fresh` | Drop all tables and recreate (recommended for development) |
| `up` | Apply pending migrations |
| `down` | Roll back the most recent migration |
| `reset` | Roll back all migrations then re-apply |

---

## Architecture

### Layered Architecture

```
Request → AuthLayer Middleware → API Layer → Service Layer → Model Layer → Database
            │                        │              │                │
            ├─ JWT Verify            ├─ Routing     ├─ Business Logic ├─ DAO (SeaORM Entities)
            ├─ JWT Blacklist Check   ├─ Parse Req   └─ Txn Management └─ DTO (Data Transfer Objects)
            └─ Casbin Authz         └─ Wrap Response
```

### Startup Flow

Startup flow in `gateway/src/main.rs`:

1. **Initialize Logging** — `Level::init()` configures log level from `config.yml`
2. **Run Migrations** — `Migrator::migration_init()` executes migrations based on `migration` config
3. **Database Connection** — `DB::db_connection()` creates SeaORM connection pool
4. **Initialize Casbin** — Load RBAC model + SeaORM Adapter → create `CachedEnforcer`
5. **Inject Enforcer** — Inject Enforcer into service layer for cache invalidation after policy changes
6. **Create AuthLayer** — Wrap JWT verification + Casbin authorization middleware
7. **Assemble Routes** — Public routes + Swagger UI + protected routes (with AuthLayer) + CORS
8. **Start Server** — Bind address and start listening

### Authentication & Authorization Flow

```
Client Request (with Authorization: Bearer {token})
  │
  ▼
AuthLayer Middleware
  │
  ├─ 1. Extract JWT → verify_token()
  │     Failed → 401 Unauthorized
  │
  ├─ 2. JWT blacklist check (jwt_blacklists table)
  │     Blacklisted → 401 Unauthorized
  │
  ├─ 3. Inject Username into request extension
  │
  ├─ 4. Casbin enforce(sub=user, obj=path, act=method)
  │     Denied → 403 Forbidden
  │     Allowed → Forward to downstream handler
  │
  ▼
API Handler (get current user via Extension<Username>)
```

### Casbin RBAC Model

The model is defined in `server/casbin/rbac_model.conf`:

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

- **sub**: Username (subject)
- **obj**: Request path (supports `keyMatch2` pattern matching, e.g. `/api/user/{id}`)
- **act**: HTTP method (supports regex matching, e.g. `GET|POST`)

---

## Key Conventions

### Database Connection

Use the `db_conn!()` macro to get a database connection:

```rust
let db = db_conn!();
// Macro expands to: &utils::prelude::DB::db_connection().await
```

### Unified Response Format

All APIs return the `R<T>` structure:

```json
{
  "code": 200,
  "message": "success",
  "data": { ... }
}
```

```rust
// Success
R::ok(data)           // code = 200

// Failure
R::fail(400, "Bad request")
```

### Error Handling

The `AppError` enum unifies error types and auto-converts to HTTP status codes:

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

### Pagination

```rust
// Request parameters
pub struct PageRequest {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}

// Response structure
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
```

### Password Hashing

Password hashing using the Argon2 algorithm:

```rust
// Encrypt
let hash = PasswordUtils::encrypt("plaintext_password");
// hash.password_hash → Store in database
// hash.salt → Store in database

// Verify
PasswordUtils::verify("input_password", &stored_hash, &stored_salt)?;
```

### JWT Token

```rust
// Generate
let token = create_token("username")?;

// Verify
let claims = verify_token(&token)?;
// claims.sub → Username
// claims.exp → Expiration time
// claims.iat → Issued at time
```

---

## API Modules

The system includes the following API modules. All endpoints are documented via Swagger UI:

| Module | Route Prefix | Description |
|--------|--------------|-------------|
| User | `/api/user` | Login, register, CRUD, password change, user info |
| Role | `/api/role` | Role CRUD |
| Menu | `/api/menu` | Menu CRUD |
| API | `/api/api` | API endpoint management |
| Casbin Policy | `/api/casbin` | Policy management, role policy assignment |
| JWT | `/api/jwt` | JWT blacklist management |
| Menu Button | `/api/menu_btn` | Menu button management |
| Menu Param | `/api/menu_param` | Menu route parameters |
| Role Button | `/api/role_btn` | Role button permissions |
| Role Menu | `/api/role_menu` | Role menu permissions |
| Data Role | `/api/data_role` | Role data permissions |
| Dictionary | `/api/dictionary` | System dictionary CRUD |
| Dictionary Detail | `/api/dictionary_detail` | Dictionary item CRUD |
| Operation Record | `/api/operation_record` | Operation log management |
| Dashboard | `/api/dashboard` | Statistics data |

### Public Routes

The following routes do not require authentication:

- `POST /api/user/login` — User login
- `GET /health` — Health check

### Routing Conventions

- Public routes: `/api/user/login`, `/health`
- Protected routes: `/api/{resource}` (CRUD)
- Resource operations: `/api/{resource}/{id}` (GET / PUT / DELETE)

---

## Database Migration

### Migration Scripts

Migration scripts are located in `server/data/migration/src/`, named by timestamp:

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

### Helper Scripts

```bash
# Create a new migration
cd backend/server/shell && sh migrate_table.sh

# Generate entities (reverse-engineer from database)
cd backend/server/shell && sh gen_entity.sh

# Install sea-orm-cli
cd backend/server/shell && sh install_sea_orm_cli.sh
```

> **Note**: The database connection info in `gen_entity.sh` must match `config.yml`.

---

## Adding a New Module

Example: adding a `xxx` module:

### 1. Create Migration

```bash
cd backend/server/shell
# Edit table name in migrate_table.sh then run
sh migrate_table.sh
```

Define the table schema in the generated migration file and register it in `migration/src/lib.rs`:

```rust
mod mXXXXXX_create_xxx;
// ...
vec![Box::new(mXXXXXX_create_xxx::Migration)]
```

### 2. Generate Entity

```bash
cd backend/server/shell && sh gen_entity.sh
```

Or manually create entity files in `model/src/dao/`.

### 3. Create DTO

Add `xxx_dto.rs` in `model/src/dto/`:

```rust
#[derive(Deserialize, ToSchema)]
pub struct XxxInsertDTO { ... }

#[derive(Deserialize, ToSchema)]
pub struct XxxUpdateDTO { ... }

#[derive(Deserialize, ToSchema)]
pub struct XxxQueryDTO { ... }
```

Register in `model/src/dto/mod.rs`: `pub mod xxx_dto;`

### 4. Create Service

Add `xxx_service.rs` in `service/src/` and implement CRUD logic:

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

Register in `service/src/lib.rs`: `pub mod xxx_service;`

### 5. Create API

Add `xxx_api.rs` in `api/src/` with routes and OpenAPI annotations:

```rust
#[utoipa::path(post, path = "/api/xxx", ...)]
pub async fn create(...) -> Result<impl IntoResponse, AppError> { ... }

pub fn routes() -> Router {
    Router::new()
        .route("/api/xxx/list", get(list))
        .route("/api/xxx/{id}", get(get_by_id).put(update).delete(delete))
}
```

### 6. Register Module

In `api/src/lib.rs`:

- `pub mod xxx_api;` — Declare module
- `.merge(xxx_api::routes())` in `protected_routes()` — Merge routes
- Register endpoints in `ApiDoc` `paths()`
- Register DTOs in `components(schemas(...))`

---

## Development Guide

### Build Optimization

Release profile optimization in `Cargo.toml`:

```toml
[profile.release]
codegen-units = 1
debug = false
lto = true
opt-level = "z"       # Optimize for binary size
panic = 'abort'
```

### Coding Conventions

- Rust edition 2021
- Comments in Chinese, module-level `//!` doc comments
- Error handling: `anyhow::Result` + `AppError`
- Serialization: `serde` derive
- Naming: snake_case

### Logging

Logging via `tracing`, level controlled by `logger.level` in `config.yml`:

```rust
tracing::info!("Info log");
tracing::error!("Error log: {}", e);
```

---

## Common Commands

```bash
# Start development server
cd backend && cargo run

# Build release
cd backend && cargo build --release

# Check compilation
cd backend && cargo check

# Run tests
cd backend && cargo test

# Start infrastructure
cd backend && docker compose up -d

# View infrastructure logs
cd backend && docker compose logs -f mysql

# Stop infrastructure
cd backend && docker compose down
```
