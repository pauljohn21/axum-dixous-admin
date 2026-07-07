//! 后端代码生成模块
//!
//! 生成 5 个后端文件:
//! 1. Migration 迁移文件
//! 2. DAO 实体文件
//! 3. DTO 数据传输对象
//! 4. Service 业务逻辑
//! 5. API HTTP 路由

use crate::config::{FieldConfig, ModuleConfig};
use crate::naming::Naming;
use crate::type_map::{get_type_mapping, backend_field_type};

/// 生成迁移文件 — 返回 (文件名, 模块名, 文件内容)
pub fn gen_migration(config: &ModuleConfig, naming: &Naming) -> (String, String, String) {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("m{}_create_{}.rs", timestamp, naming.table_name);
    let module_name = format!("m{}_create_{}", timestamp, naming.table_name);

    // DeriveIden 枚举变体
    let iden_variants: Vec<String> = std::iter::once("Table".to_string())
        .chain(std::iter::once("Id".to_string()))
        .chain(std::iter::once("CreatedAt".to_string()))
        .chain(std::iter::once("UpdatedAt".to_string()))
        .chain(std::iter::once("DeletedAt".to_string()))
        .chain(config.fields.iter().map(|f| {
            crate::naming::to_pascal(&f.name)
        }))
        .collect();

    // 列定义
    let mut cols = String::new();
    cols.push_str("                    .col(\n");
    cols.push_str("                        ColumnDef::new(Idden::Id)\n");
    cols.push_str("                            .big_integer()\n");
    cols.push_str("                            .not_null()\n");
    cols.push_str("                            .auto_increment()\n");
    cols.push_str("                            .primary_key(),\n");
    cols.push_str("                    )\n");
    cols.push_str("                    .col(ColumnDef::new(Idden::CreatedAt).date_time().null())\n");
    cols.push_str("                    .col(ColumnDef::new(Idden::UpdatedAt).date_time().null())\n");
    cols.push_str("                    .col(ColumnDef::new(Idden::DeletedAt).date_time().null())\n");

    for field in &config.fields {
        let mapping = get_type_mapping(&field.field_type);
        let iden = crate::naming::to_pascal(&field.name);
        let null_str = if field.nullable { ".null()" } else { ".not_null()" };

        if field.field_type == "decimal" {
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).decimal_len(10, 2){null_str})\n",
                iden = iden,
                null_str = null_str,
            ));
        } else {
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).{col}(){null_str}.comment(\"{comment}\"))\n",
                iden = iden,
                col = mapping.sea_orm_col,
                null_str = null_str,
                comment = field.comment,
            ));
        }
    }

    let content = format!(
        r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table(Idden::Table)
                    .if_not_exists()
{cols}                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table(Idden::Table).to_owned())
            .await
    }}
}}

#[derive(Iden)]
enum Idden {{
    {variants},
}}
"#,
        cols = cols,
        variants = iden_variants.join(",\n    "),
    );

    (file_name, module_name, content)
}

/// 生成 DAO 实体文件
pub fn gen_dao(config: &ModuleConfig, naming: &Naming) -> String {
    let mut fields = String::new();
    fields.push_str("    #[sea_orm(primary_key)]\n");
    fields.push_str("    pub id: u64,\n");

    // 时间戳字段
    fields.push_str("    #[serde(with = \"time::serde::rfc3339::option\")]\n");
    fields.push_str("    pub created_at: Option<OffsetDateTime>,\n");
    fields.push_str("    #[serde(with = \"time::serde::rfc3339::option\")]\n");
    fields.push_str("    pub updated_at: Option<OffsetDateTime>,\n");
    fields.push_str("    #[serde(with = \"time::serde::rfc3339::option\")]\n");
    fields.push_str("    pub deleted_at: Option<OffsetDateTime>,\n");

    for field in &config.fields {
        let rust_type = backend_field_type(field);
        // 处理 Rust 关键字
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };
        fields.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    format!(
        r#"use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
{}}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
        naming.table_name,
        fields.trim_end_matches('\n'),
    )
}

/// 生成 DTO 文件
pub fn gen_dto(config: &ModuleConfig, naming: &Naming) -> String {
    let insert_name = format!("{}InsertDTO", naming.pascal_singular);
    let update_name = format!("{}UpdateDTO", naming.pascal_singular);
    let query_name = format!("{}QueryDTO", naming.pascal_singular);

    // InsertDTO 字段
    let mut insert_fields = String::new();
    for field in &config.fields {
        let rust_type = backend_field_type(field);
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };
        insert_fields.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    // UpdateDTO 字段 (同 InsertDTO，全部 Option)
    let mut update_fields = String::new();
    for field in &config.fields {
        let rust_type = backend_field_type(field);
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };
        update_fields.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    format!(
        r#"use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct {} {{
{}}}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct {} {{
{}}}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct {} {{
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}}
"#,
        insert_name,
        insert_fields.trim_end_matches('\n'),
        update_name,
        update_fields.trim_end_matches('\n'),
        query_name,
    )
}

/// 生成 Service 文件
pub fn gen_service(config: &ModuleConfig, naming: &Naming) -> String {
    let struct_name = format!("{}Service", naming.pascal_singular);
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);

    // insert 方法中的字段映射
    let mut insert_fields = String::new();
    for field in &config.fields {
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };
        insert_fields.push_str(&format!("            {}: Set(data.{}),\n", field_name, field_name));
    }

    // list 方法中的搜索条件
    let search_fields: Vec<&FieldConfig> = config.fields.iter().filter(|f| f.search).collect();
    let search_condition = if search_fields.is_empty() {
        String::new()
    } else {
        let mut conditions = String::new();
        for (i, field) in search_fields.iter().enumerate() {
            let _col_name = if is_rust_keyword(&field.name) {
                format!("r#{}", field.name)
            } else {
                field.name.clone()
            };
            let pascal = crate::naming::to_pascal(&field.name);
            if i == 0 {
                conditions.push_str(&format!(
                    "                q = q.filter(\n                    sea_orm::Condition::any()\n                        .add({}::Column::{}.contains(keyword))\n",
                    naming.dao_file, pascal
                ));
            } else {
                conditions.push_str(&format!(
                    "                        .add({}::Column::{}.contains(keyword))\n",
                    naming.dao_file, pascal
                ));
            }
        }
        conditions.push_str("                );\n");
        conditions
    };

    // update 方法中的字段更新
    let mut update_fields = String::new();
    for field in &config.fields {
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };
        update_fields.push_str(&format!(
            "        if let Some(v) = data.{} {{ updated.{} = Set(Some(v)); }}\n",
            field_name, field_name
        ));
    }

    // 错误消息
    let error_msg = format!("{}不存在", naming.module_cn.trim_end_matches("管理"));

    format!(
        r#"use anyhow::{{anyhow, Result}};
use sea_orm::{{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set}};

use model::dao::{dao};
use model::dto::page_dto::{{PageRequest, PageResponse}};
use model::dto::{dto_file}::{{{insert_dto}, {update_dto}}};
use model::prelude::{entity};
use utils::db_conn;

pub struct {struct_name};

impl {struct_name} {{
    pub async fn insert(data: {insert_dto}) -> Result<{dao}::Model> {{
        let db = db_conn!();
        let active = {dao}::ActiveModel {{
{insert_fields}            ..Default::default()
        }};
        let result = {entity}::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }}

    pub async fn list(query: PageRequest) -> Result<PageResponse<{dao}::Model>> {{
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = {entity}::find();
        if let Some(keyword) = &query.keyword {{
{search_condition}        }}

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse {{ list, total, page, page_size }})
    }}

    pub async fn get_by_id(id: u64) -> Result<{dao}::Model> {{
        {entity}::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("{error_msg}"))
    }}

    pub async fn update(id: u64, data: {update_dto}) -> Result<{dao}::Model> {{
        let db = db_conn!();
        let model: {dao}::ActiveModel = {entity}::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("{error_msg}"))?
            .into();
        let mut updated = model;
{update_fields}        Ok(updated.update(db).await?)
    }}

    pub async fn delete(id: u64) -> Result<()> {{
        {entity}::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }}
}}
"#,
        dao = naming.dao_file,
        dto_file = naming.dto_file,
        insert_dto = insert_dto,
        update_dto = update_dto,
        entity = naming.entity_name,
        struct_name = struct_name,
        insert_fields = insert_fields,
        search_condition = search_condition,
        update_fields = update_fields,
        error_msg = error_msg,
    )
}

/// 生成 API 文件
pub fn gen_api(_config: &ModuleConfig, naming: &Naming) -> String {
    let service = format!("{}Service", naming.pascal_singular);
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let api_path = &naming.resource;
    let tag = &naming.module_cn;
    let dao = &naming.dao_file;
    let delete_fn = format!("delete_{}", naming.resource);

    format!(
        r#"use axum::extract::{{Path, Query}};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{{get, post}};
use axum::Router;
use model::dao::{dao};
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::{dto_file}::{{{insert_dto}, {update_dto}}};
use service::{service_file}::{service};
use utils::prelude::{{AppError, R}};

#[utoipa::path(
    post,
    path = "/api/{api_path}",
    request_body = {insert_dto},
    responses((status = 200, description = "成功", body = R<{dao}::Model>)),
    tag = "{tag}"
)]
pub async fn create(Json(data): Json<{insert_dto}>) -> Result<impl IntoResponse, AppError> {{
    let result = {service}::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}}

#[utoipa::path(
    get,
    path = "/api/{api_path}/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<{dao}::Model>>)),
    tag = "{tag}"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {{
    let result = {service}::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}}

#[utoipa::path(
    get,
    path = "/api/{api_path}/{{id}}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<{dao}::Model>)),
    tag = "{tag}"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {{
    let result = {service}::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(result))
}}

#[utoipa::path(
    put,
    path = "/api/{api_path}/{{id}}",
    params(("id" = u64, Path, description = "ID")),
    request_body = {update_dto},
    responses((status = 200, description = "成功", body = R<{dao}::Model>)),
    tag = "{tag}"
)]
pub async fn update(Path(id): Path<u64>, Json(data): Json<{update_dto}>) -> Result<impl IntoResponse, AppError> {{
    let result = {service}::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}}

#[utoipa::path(
    delete,
    path = "/api/{api_path}/{{id}}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "{tag}"
)]
pub async fn {delete_fn}(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {{
    {service}::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}}

pub fn routes() -> Router {{
    Router::new()
        .route("/api/{api_path}", post(create))
        .route("/api/{api_path}/list", get(list))
        .route("/api/{api_path}/{{id}}", get(get_by_id).put(update).delete({delete_fn}))
}}
"#,
        dao = dao,
        dto_file = naming.dto_file,
        insert_dto = insert_dto,
        update_dto = update_dto,
        service_file = naming.service_file,
        service = service,
        api_path = api_path,
        tag = tag,
        delete_fn = delete_fn,
    )
}

/// 检查是否为 Rust 关键字
fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
            | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match"
            | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
            | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe"
            | "use" | "where" | "while" | "async" | "await" | "dyn" | "abstract"
            | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
            | "typeof" | "unsized" | "virtual" | "yield" | "try" | "union"
    )
}
