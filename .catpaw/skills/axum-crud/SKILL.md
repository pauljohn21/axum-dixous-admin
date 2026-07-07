---
name: axum-crud
description: Generate full-stack CRUD module for axum-dixous-admin project — backend (migration + dao + dto + service + api) and frontend (model + api + component + i18n + router). Use when adding a new resource/table, creating CRUD pages, or when the user mentions "新增模块", "CRUD", "增删改查", "新增页面".
---

# axum-dixous-admin CRUD Module Generator

Generate a complete full-stack CRUD module following the project's established conventions.

## Architecture Overview

```
Backend:  migration → dao → dto → service → api → lib.rs registration
Frontend: models → api → components → i18n → router → admin_layout menu
```

## Workflow

When asked to create a new CRUD module (e.g. "product"), follow these steps in order. Replace `xxx` with the actual resource name (snake_case), `Xxx` with PascalCase.

### Step 1: Backend Migration

Create `backend/server/data/migration/src/m{timestamp}_create_xxx.rs`:

- Run `cd backend/server/shell && sh migrate_table.sh` (edit the script first to use `create_xxx`)
- Or manually create file with timestamp format `mYYYYMMDD_HHMMSS_create_xxx`
- Use `sea_orm_migration::prelude::*` and `schema::*`
- Table name: `sys_xxx` (plural form, e.g. `sys_products`)
- Always include: `pk_auto(Id)`, `created_at`, `updated_at`, `deleted_at` columns
- Use `DeriveIden` enum for table/column identifiers
- Register in `migration/src/lib.rs`: add `mod m...;` and `Box::new(m...::Migration)` to `Migrator::migrations()`

### Step 2: Backend DAO (Entity)

- Run `cd backend/server/shell && sh gen_entity.sh` to auto-generate from DB
- Or manually create `model/src/dao/sys_xxx.rs`:

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_xxx")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: Option<String>,
    // ... business fields
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

- Register in `model/src/dao/mod.rs`: `pub mod sys_xxx;`
- Register in `model/src/dao/prelude.rs`: `pub use super::sys_xxx::Entity as SysXxx;`

### Step 3: Backend DTO

Create `model/src/dto/sys_xxx_dto.rs`:

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysXxxInsertDTO {
    pub name: Option<String>,
    // ... fields for creation
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysXxxUpdateDTO {
    pub name: Option<String>,
    // ... fields for update (all Option)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysXxxQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
```

- Register in `model/src/dto/mod.rs`: `pub mod sys_xxx_dto;`
- Add to `components(schemas(...))` in `api/src/lib.rs`

### Step 4: Backend Service

Create `service/src/sys_xxx_service.rs`:

```rust
use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_xxx;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_xxx_dto::{SysXxxInsertDTO, SysXxxUpdateDTO};
use model::prelude::SysXxx;
use utils::db_conn;

pub struct SysXxxService;

impl SysXxxService {
    pub async fn insert(data: SysXxxInsertDTO) -> Result<sys_xxx::Model> {
        let db = db_conn!();
        let active = sys_xxx::ActiveModel {
            name: Set(data.name),
            ..Default::default()
        };
        let result = SysXxx::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_xxx::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        let mut q = SysXxx::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_xxx::Column::Name.contains(keyword))
            );
        }
        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<sys_xxx::Model> {
        SysXxx::find_by_id(id).one(db_conn!()).await?
            .ok_or_else(|| anyhow!("xxx不存在"))
    }

    pub async fn update(id: u64, data: SysXxxUpdateDTO) -> Result<sys_xxx::Model> {
        let db = db_conn!();
        let model: sys_xxx::ActiveModel = SysXxx::find_by_id(id).one(db).await?
            .ok_or_else(|| anyhow!("xxx不存在"))?.into();
        let mut updated = model;
        if let Some(v) = data.name { updated.name = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: u64) -> Result<()> {
        SysXxx::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
```

- Register in `service/src/lib.rs`: `pub mod sys_xxx_service;`

### Step 5: Backend API

Create `api/src/xxx_api.rs` with 5 endpoints + `routes()` function:

```rust
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_xxx;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_xxx_dto::{SysXxxInsertDTO, SysXxxUpdateDTO};
use service::sys_xxx_service::SysXxxService;
use utils::prelude::{AppError, R};

#[utoipa::path(post, path = "/api/xxx", request_body = SysXxxInsertDTO, responses((status = 200, body = R<sys_xxx::Model>)), tag = "xxx管理")]
pub async fn create(Json(data): Json<SysXxxInsertDTO>) -> Result<impl IntoResponse, AppError> {
    Ok(R::ok(SysXxxService::insert(data).await.map_err(AppError::Anyhow)?))
}

#[utoipa::path(get, path = "/api/xxx/list", params(("page" = Option<u64>, Query), ("page_size" = Option<u64>, Query)), responses((status = 200, body = R<PageResponse<sys_xxx::Model>>)), tag = "xxx管理")]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    Ok(R::ok(SysXxxService::list(query).await.map_err(AppError::Anyhow)?))
}

#[utoipa::path(get, path = "/api/xxx/{id}", params(("id" = u64, Path)), responses((status = 200, body = R<sys_xxx::Model>)), tag = "xxx管理")]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    Ok(R::ok(SysXxxService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?))
}

#[utoipa::path(put, path = "/api/xxx/{id}", params(("id" = u64, Path)), request_body = SysXxxUpdateDTO, responses((status = 200, body = R<sys_xxx::Model>)), tag = "xxx管理")]
pub async fn update(Path(id): Path<u64>, Json(data): Json<SysXxxUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    Ok(R::ok(SysXxxService::update(id, data).await.map_err(AppError::Anyhow)?))
}

#[utoipa::path(delete, path = "/api/xxx/{id}", params(("id" = u64, Path)), responses((status = 200, body = R<serde_json::Value>)), tag = "xxx管理")]
pub async fn delete_xxx(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysXxxService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/xxx", post(create))
        .route("/api/xxx/list", get(list))
        .route("/api/xxx/{id}", get(get_by_id).put(update).delete(delete_xxx))
}
```

**Register in `api/src/lib.rs`:**
1. `pub mod xxx_api;`
2. Add all 5 paths to `paths(...)` in `#[openapi(...)]`
3. Add `xxx_api::routes()` to `protected_routes()` via `.merge()`
4. Add tag `(name = "xxx管理", description = "xxx CRUD")`

### Step 6: Frontend Model

Create `web/src/models/xxx.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysXxx {
    pub id: i32,
    #[serde(default)]
    pub name: Option<String>,
    // ... fields matching backend Model
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysXxxInsertDTO {
    pub name: String,
    // ... required fields for create
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysXxxUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // ... all Option for partial update
}
```

- Register in `web/src/models/mod.rs`: `pub mod xxx;`
- **Frontend `id` is `i32`** (not `u64` like backend — intentional convention)

### Step 7: Frontend API

Create `web/src/api/xxx.rs`:

```rust
use crate::http::{build_page_query, delete_void, get_with_query, post_void, put};
use crate::models::common::PageResponse;
use crate::models::xxx::{SysXxx, SysXxxInsertDTO, SysXxxUpdateDTO};

pub async fn list(page: Option<u32>, page_size: Option<u32>, keyword: Option<&str>) -> Result<PageResponse<SysXxx>, String> {
    get_with_query("/api/xxx/list", &build_page_query(page, page_size, keyword)).await
}

pub async fn create(data: SysXxxInsertDTO) -> Result<(), String> {
    post_void("/api/xxx", &data).await
}

pub async fn update(id: i32, data: SysXxxUpdateDTO) -> Result<SysXxx, String> {
    put(&format!("/api/xxx/{}", id), &data).await
}

pub async fn delete_xxx(id: i32) -> Result<(), String> {
    delete_void(&format!("/api/xxx/{}", id)).await
}
```

- Register in `web/src/api/mod.rs`: `pub mod xxx;`
- Use `post_void` / `delete_void` when backend returns `R<()>` (null data)
- Use `post` / `delete` when backend returns `R<Model>`

### Step 8: Frontend Component

Create `web/src/components/xxx_manage.rs` — full CRUD page with:
- Page title + Add button
- Error alert (CSS variable colors)
- Search bar (keyword input + search button)
- Data table with `th_style` / `td_style` helper functions (all using CSS variables)
- Loading / empty state
- Pagination (prev/next + total text)
- Delete confirmation modal
- Add/Edit modal dialog
- All colors use `var(--el-xxx)` CSS variables (see theme module)

**Key RSX patterns:**

```rust
// Edit button — clone before closure
Button {
    on_click: { let item = item.clone(); move |_| on_edit(item.clone()) },
    "{t(TKey::Edit)}"
}

// Delete button — pass id directly
Button {
    variant: ButtonVariant::Danger,
    on_click: move |_| on_delete(item.id),
    "{t(TKey::Delete)}"
}

// Style helpers — always use CSS variables
fn th_style() -> String {
    "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);".into()
}
fn td_style() -> String {
    "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);".into()
}
fn label_style() -> String {
    "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;".into()
}
```

- Register in `web/src/components/mod.rs`: `pub mod xxx_manage;`

### Step 9: Frontend i18n

In `web/src/i18n/mod.rs`:

1. Add TKey variants:
```rust
// xxx管理
XxxManage, AddXxx, EditXxx, XxxName, XxxField2,
SearchXxxPlaceholder, XxxNamePlaceholder, XxxField2Placeholder,
```

2. Add Chinese translations in `t_zh()`:
```rust
TKey::XxxManage => "xxx管理", TKey::AddXxx => "+ 新增xxx", TKey::EditXxx => "编辑xxx",
TKey::XxxName => "名称", TKey::XxxField2 => "字段2",
TKey::SearchXxxPlaceholder => "搜索xxx名称", TKey::XxxNamePlaceholder => "请输入名称",
TKey::XxxField2Placeholder => "请输入字段2",
```

3. Add English translations in `t_en()`:
```rust
TKey::XxxManage => "Xxx Management", TKey::AddXxx => "+ Add Xxx", TKey::EditXxx => "Edit Xxx",
TKey::XxxName => "Name", TKey::XxxField2 => "Field2",
TKey::SearchXxxPlaceholder => "Search xxx name", TKey::XxxNamePlaceholder => "Enter name",
TKey::XxxField2Placeholder => "Enter field2",
```

### Step 10: Frontend Router

In `web/src/router/mod.rs`:

1. Add route variant inside `#[layout(AdminLayout)]`:
```rust
#[route("/xxxs")]
XxxManage {},
```

2. Add to `menu_path_to_route()` in `menu_item.rs`:
```rust
"xxx" | "xxxs" => Some(Route::XxxManage {}),
```

### Step 11: Frontend Menu (Database)

Insert a menu record via migration or SQL:
```sql
INSERT INTO sys_base_menus (menu_level, parent_id, path, name, hidden, sort, title, icon)
VALUES (0, 0, 'xxx', 'xxx', 0, 10, 'xxx管理', 'document');
```

Or add to the menu migration's `values_panic` section.

## Key Conventions

| Convention | Detail |
|---|---|
| Table naming | `sys_xxx` (snake_case, sys_ prefix) |
| DAO Entity alias | `SysXxx` (PascalCase in prelude.rs) |
| Service struct | `SysXxxService` with static methods |
| API tag | Chinese name + "管理" (e.g. "字典管理") |
| API path | `/api/xxx` (singular resource name) |
| Response wrapper | `R<T>` with `{ code: 200, message, data }` |
| Frontend id type | `i32` (not `u64`) |
| HTTP functions | `post_void`/`delete_void` for null data, `post`/`delete` for model data |
| CSS colors | Always `var(--el-xxx)` — never hardcoded hex |
| Comments | Chinese comments throughout |
| Error handling | Backend: `anyhow::Result` + `AppError`; Frontend: `Result<T, String>` |

## Verification

After generating all files:

1. Backend: `cd backend && cargo check`
2. Frontend: `cd web && cargo check`
3. Verify Swagger UI shows new endpoints at `http://localhost:8888/`
