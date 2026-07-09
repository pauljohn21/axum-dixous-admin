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
use crate::type_map::{get_type_mapping, backend_field_type, SearchType};

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
        let comment_str = if field.comment.is_empty() {
            String::new()
        } else {
            format!(".comment(\"{}\")", field.comment)
        };

        if field.field_type == "decimal" {
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).decimal_len(10, 2){null_str}{comment_str})\n",
                iden = iden,
                null_str = null_str,
                comment_str = comment_str,
            ));
        } else if field.field_type == "enum" {
            // 枚举类型使用 string 存储
            let enum_vals = if field.enum_values.is_empty() {
                "\"\"".to_string()
            } else {
                field.enum_values.split(',').map(|v| format!("\"{}\"", v.trim())).collect::<Vec<_>>().join(", ")
            };
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).string(){null_str}{comment_str})\n",
                iden = iden,
                null_str = null_str,
                comment_str = comment_str,
            ));
            // 注释枚举值
            let _ = enum_vals; // 枚举值在 comment 中体现
        } else {
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).{col}(){null_str}{comment_str})\n",
                iden = iden,
                col = mapping.sea_orm_col,
                null_str = null_str,
                comment_str = comment_str,
            ));
        }

        // 默认值
        if !field.default_value.is_empty() {
            let default_str = if mapping.is_string || mapping.is_enum {
                format!("\"{}\"", field.default_value)
            } else if mapping.is_bool {
                field.default_value.clone()
            } else {
                field.default_value.clone()
            };
            cols.push_str(&format!(
                "                    .col(ColumnDef::new(Idden::{iden}).{col}().default({default}))\n",
                iden = iden,
                col = mapping.sea_orm_col,
                default = default_str,
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
        let mapping = get_type_mapping(&field.field_type);
        let rust_type = backend_field_type(field);
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };

        // JSON/Array 类型需要特殊 serde 属性
        if mapping.is_json {
            fields.push_str("    #[serde(default)]\n");
        }

        fields.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    // 需要导入 serde_json 吗
    let needs_serde_json = config.fields.iter().any(|f| {
        let m = get_type_mapping(&f.field_type);
        m.is_json
    });

    let serde_json_import = if needs_serde_json {
        "use serde_json;\n"
    } else {
        ""
    };

    format!(
        r#"use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};
use time::OffsetDateTime;
use utoipa::ToSchema;
{serde_json_import}
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
        serde_json_import = serde_json_import,
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

        // 必填验证
        if field.require && field.nullable {
            insert_fields.push_str(&format!(
                "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n",
            ));
        } else if field.require && !field.nullable {
            // 非空必填字段不需要 skip
        } else {
            insert_fields.push_str("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
        }
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
        update_fields.push_str("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
        update_fields.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }

    // QueryDTO — 基础分页 + 字段级搜索条件
    let mut query_fields = String::new();
    query_fields.push_str("    pub page: Option<u64>,\n");
    query_fields.push_str("    pub page_size: Option<u64>,\n");
    query_fields.push_str("    pub keyword: Option<String>,\n");

    // 为有搜索类型的字段添加搜索参数
    for field in &config.fields {
        let search_type = SearchType::from_str(field.effective_search_type());
        if search_type == SearchType::None {
            continue;
        }

        let mapping = get_type_mapping(&field.field_type);
        let field_name = if is_rust_keyword(&field.name) {
            format!("r#{}", field.name)
        } else {
            field.name.clone()
        };

        if search_type.is_range() {
            // BETWEEN 类型需要 start/end 两个字段
            query_fields.push_str(&format!(
                "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub start_{}: Option<{}>,\n",
                field.name, mapping.backend_rust
            ));
            query_fields.push_str(&format!(
                "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub end_{}: Option<{}>,\n",
                field.name, mapping.backend_rust
            ));
        } else {
            // 普通搜索: 字符串类型用 String, 其他用 Option<T>
            if mapping.is_string {
                query_fields.push_str(&format!(
                    "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub {}: Option<String>,\n",
                    field_name
                ));
            } else {
                query_fields.push_str(&format!(
                    "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub {}: Option<{}>,\n",
                    field_name, mapping.backend_rust
                ));
            }
        }
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
{}}}
"#,
        insert_name,
        insert_fields.trim_end_matches('\n'),
        update_name,
        update_fields.trim_end_matches('\n'),
        query_name,
        query_fields.trim_end_matches('\n'),
    )
}

/// 生成 Service 文件
pub fn gen_service(config: &ModuleConfig, naming: &Naming) -> String {
    let struct_name = format!("{}Service", naming.pascal_singular);
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let query_dto = format!("{}QueryDTO", naming.pascal_singular);

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

    // list 方法中的搜索条件 — 支持字段级搜索类型
    let search_fields: Vec<&FieldConfig> = config.fields.iter().filter(|f| f.is_searchable()).collect();
    let search_condition = if search_fields.is_empty() {
        String::new()
    } else {
        let mut conditions = String::new();
        let has_keyword = search_fields.iter().any(|f| f.effective_search_type() == "like");

        // 关键字搜索 (所有 like 字段用 OR 连接)
        if has_keyword {
            let like_fields: Vec<&FieldConfig> = search_fields.iter().filter(|f| f.effective_search_type() == "like").copied().collect();
            conditions.push_str("        if let Some(keyword) = &query.keyword {\n");
            conditions.push_str("            let kw = format!(\"%{}%\", keyword);\n");
            conditions.push_str("            q = q.filter(\n");
            conditions.push_str("                sea_orm::Condition::any()\n");
            for field in &like_fields {
                let pascal = crate::naming::to_pascal(&field.name);
                conditions.push_str(&format!(
                    "                    .add({}::Column::{}.like(&kw))\n",
                    naming.dao_file, pascal
                ));
            }
            conditions.push_str("            );\n");
            conditions.push_str("        }\n");
        }

        // 字段级精确搜索
        for field in &search_fields {
            let search_type = SearchType::from_str(field.effective_search_type());
            if search_type == SearchType::Like {
                continue; // 已在 keyword 中处理
            }

            let pascal = crate::naming::to_pascal(&field.name);
            let field_name = if is_rust_keyword(&field.name) {
                format!("r#{}", field.name)
            } else {
                field.name.clone()
            };
            let col_expr = format!("{}::Column::{}", naming.dao_file, pascal);

            match search_type {
                SearchType::Eq => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.eq(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Ne => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.ne(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Gt => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.gt(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Lt => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.lt(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Gte => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.gte(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Lte => {
                    conditions.push_str(&format!(
                        "        if let Some(v) = &query.{} {{ q = q.filter({}.lte(v.clone())); }}\n",
                        field_name, col_expr
                    ));
                }
                SearchType::Between => {
                    conditions.push_str(&format!(
                        "        if let Some(start) = &query.start_{} {{\n            if let Some(end) = &query.end_{} {{\n                q = q.filter({}.between(start.clone(), end.clone()));\n            }}\n        }}\n",
                        field.name, field.name, col_expr
                    ));
                }
                _ => {}
            }
        }
        conditions
    };

    // 排序支持
    let sortable_fields: Vec<&FieldConfig> = config.fields.iter().filter(|f| f.sort).collect();
    let sort_logic = if !sortable_fields.is_empty() {
        let mut logic = String::new();
        logic.push_str("        // 排序\n");
        logic.push_str("        if let Some(sort_field) = &query.sort_field {\n");
        logic.push_str("            let desc = query.sort_order.as_deref() == Some(\"desc\");\n");
        logic.push_str("            match sort_field.as_str() {\n");
        for field in &sortable_fields {
            let pascal = crate::naming::to_pascal(&field.name);
            logic.push_str(&format!(
                "                \"{}\" => {{ q = q.order_by({}::Column::{}, if desc {{ Order::Desc }} else {{ Order::Asc }}); }}\n",
                field.name, naming.dao_file, pascal
            ));
        }
        logic.push_str("                _ => {}\n");
        logic.push_str("            }\n");
        logic.push_str("        }\n");
        logic
    } else {
        String::new()
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

    // 批量删除
    let batch_delete_method = if config.batch_delete {
        format!(
            r#"
    pub async fn delete_batch(ids: Vec<u64>) -> Result<()> {{
        {entity}::delete_many()
            .filter({entity}::Column::Id.is_in(ids))
            .exec(db_conn!())
            .await?;
        Ok(())
    }}
"#,
            entity = naming.entity_name,
        )
    } else {
        String::new()
    };

    // QueryDTO 引入
    let query_dto_import = if !search_fields.is_empty() || !sortable_fields.is_empty() {
        format!(", {}", query_dto)
    } else {
        String::new()
    };

    // list 方法签名
    let list_sig = if !sortable_fields.is_empty() {
        format!("pub async fn list(query: {}) -> Result<PageResponse<{}::Model>>", query_dto, naming.dao_file)
    } else {
        format!("pub async fn list(query: PageRequest) -> Result<PageResponse<{}::Model>>", naming.dao_file)
    };

    // list 方法中是否需要用 query_dto
    let list_body = if !sortable_fields.is_empty() {
        format!(
            r#"        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = {entity}::find();
{search_condition}{sort_logic}
        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse {{ list, total, page, page_size }})"#,
            entity = naming.entity_name,
            search_condition = search_condition,
            sort_logic = sort_logic,
        )
    } else if !search_fields.is_empty() {
        format!(
            r#"        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = {entity}::find();
{search_condition}
        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse {{ list, total, page, page_size }})"#,
            entity = naming.entity_name,
            search_condition = search_condition,
        )
    } else {
        format!(
            r#"        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = {entity}::find();
        if let Some(keyword) = &query.keyword {{
            q = q.filter(
                sea_orm::Condition::any()
                    .add({dao}::Column::Name.contains(keyword))
            );
        }}

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse {{ list, total, page, page_size }})"#,
            entity = naming.entity_name,
            dao = naming.dao_file,
        )
    };

    // Order 导入
    let order_import = if !sortable_fields.is_empty() {
        "use sea_orm::{{ActiveModelTrait, ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set}};"
    } else {
        "use sea_orm::{{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set}};"
    };

    format!(
        r#"use anyhow::{{anyhow, Result}};
{order_import}

use model::dao::{dao};
use model::dto::page_dto::{{PageRequest, PageResponse}};
use model::dto::{dto_file}::{{{insert_dto}, {update_dto}{query_dto_import}}};
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

    {list_sig} {{
{list_body}
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
{batch_delete_method}}}
"#,
        order_import = order_import,
        dao = naming.dao_file,
        dto_file = naming.dto_file,
        insert_dto = insert_dto,
        update_dto = update_dto,
        query_dto_import = query_dto_import,
        entity = naming.entity_name,
        struct_name = struct_name,
        insert_fields = insert_fields,
        list_sig = list_sig,
        list_body = list_body,
        update_fields = update_fields,
        error_msg = error_msg,
        batch_delete_method = batch_delete_method,
    )
}

/// 生成 API 文件
pub fn gen_api(config: &ModuleConfig, naming: &Naming) -> String {
    let service = format!("{}Service", naming.pascal_singular);
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let query_dto = format!("{}QueryDTO", naming.pascal_singular);
    let api_path = &naming.resource;
    let tag = &naming.module_cn;
    let dao = &naming.dao_file;
    let delete_fn = format!("delete_{}", naming.resource);

    // 是否有搜索字段
    let has_search = config.fields.iter().any(|f| f.is_searchable());
    let has_sort = config.fields.iter().any(|f| f.sort);
    let use_query_dto = has_search || has_sort;

    // Query 参数类型
    let query_type = if use_query_dto {
        &query_dto
    } else {
        "PageRequest"
    };
    let query_import = if use_query_dto {
        format!("use model::dto::{}::{{{}, {}}};\nuse model::dto::page_dto::PageResponse;", naming.dto_file, query_dto, insert_dto)
    } else {
        format!("use model::dto::page_dto::{{PageRequest, PageResponse}};\nuse model::dto::{}::{{{}, {}}};", naming.dto_file, insert_dto, update_dto)
    };

    // 批量删除 API
    let batch_delete_api = if config.batch_delete {
        format!(
            r#"
#[utoipa::path(
    delete,
    path = "/api/{api_path}/batch",
    request_body = Vec<u64>,
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "{tag}"
)]
pub async fn delete_batch(Json(ids): Json<Vec<u64>>) -> Result<impl IntoResponse, AppError> {{
    {service}::delete_batch(ids).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}}
"#,
            api_path = api_path,
            tag = tag,
            service = service,
        )
    } else {
        String::new()
    };

    // 批量删除路由
    let batch_delete_route = if config.batch_delete {
        format!("\n        .route(\"/api/{api_path}/batch\", delete(delete_batch))", api_path = api_path)
    } else {
        String::new()
    };

    format!(
        r#"use axum::extract::{{Path, Query}};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{{get, post, delete}};
use axum::Router;
use model::dao::{dao};
{query_import}
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
    responses((status = 200, description = "成功", body = R<PageResponse<{dao}::Model>>)),
    tag = "{tag}"
)]
pub async fn list(Query(query): Query<{query_type}>) -> Result<impl IntoResponse, AppError> {{
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
{batch_delete_api}
pub fn routes() -> Router {{
    Router::new()
        .route("/api/{api_path}", post(create))
        .route("/api/{api_path}/list", get(list))
        .route("/api/{api_path}/{{id}}", get(get_by_id).put(update).delete({delete_fn})){batch_delete_route}
}}
"#,
        query_import = query_import,
        dao = dao,
        insert_dto = insert_dto,
        update_dto = update_dto,
        query_type = query_type,
        service_file = naming.service_file,
        service = service,
        api_path = api_path,
        tag = tag,
        delete_fn = delete_fn,
        batch_delete_api = batch_delete_api,
        batch_delete_route = batch_delete_route,
    )
}

/// 检查是否为 Rust 关键字
pub fn is_rust_keyword(s: &str) -> bool {
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
