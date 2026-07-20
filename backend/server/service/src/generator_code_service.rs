//! 代码生成 Service
//!
//! 根据配置生成前后端代码，支持预览和实际写入文件。
//! 生成的代码遵循项目架构规范（Phase 1-3 优化后）。

use chrono::Local;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use utils::prelude::ServiceError;

/// 生成的代码文件
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GeneratedFile {
    pub file_name: String,
    pub file_path: String,
    pub content: String,
    pub file_type: String,
}

/// 代码预览响应
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PreviewCodeResponse {
    pub backend_files: Vec<GeneratedFile>,
    pub frontend_files: Vec<GeneratedFile>,
}

/// 生成器配置（与前端 GeneratorConfig 对应）
#[derive(Debug, Clone, Deserialize)]
struct GeneratorConfig {
    table_name: String,
    resource: String,
    #[serde(default)]
    module_cn: String,
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
    #[serde(default = "default_true")]
    generate_backend: bool,
    #[serde(default = "default_true")]
    generate_frontend: bool,
    #[serde(default)]
    fields: Vec<GeneratorField>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct GeneratorField {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
    #[serde(default = "default_true")]
    nullable: bool,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    search: bool,
    #[serde(default)]
    search_type: String,
    #[serde(default)]
    require: bool,
    #[serde(default)]
    default_value: String,
    #[serde(default = "default_true")]
    form: bool,
    #[serde(default = "default_true")]
    table: bool,
    #[serde(default)]
    primary_key: bool,
}

fn default_true() -> bool {
    true
}

pub struct GeneratorCodeService;

impl GeneratorCodeService {
    /// 预览代码 - 根据 JSON 配置生成所有代码文件内容
    pub async fn preview_code(
        data: model::dto::sys_generator_history_dto::PreviewCodeDTO,
    ) -> Result<PreviewCodeResponse, ServiceError> {
        let config: GeneratorConfig = serde_json::from_str(&data.config_json)
            .map_err(|e| ServiceError::BadRequest(format!("配置解析失败: {}", e)))?;

        if config.table_name.is_empty() || config.resource.is_empty() {
            return Err(ServiceError::BadRequest(
                "table_name 和 resource 不能为空".into(),
            ));
        }

        let mut backend_files = Vec::new();
        let mut frontend_files = Vec::new();

        if config.generate_backend {
            backend_files.push(Self::gen_migration(&config));
            backend_files.push(Self::gen_dao(&config));
            backend_files.push(Self::gen_dto(&config));
            backend_files.push(Self::gen_service(&config));
            backend_files.push(Self::gen_api(&config));
        }

        if config.generate_frontend {
            frontend_files.push(Self::gen_model(&config));
            frontend_files.push(Self::gen_api_call(&config));
            frontend_files.push(Self::gen_component(&config));
        }

        Ok(PreviewCodeResponse {
            backend_files,
            frontend_files,
        })
    }

    fn timestamp() -> String {
        Local::now().format("%Y%m%d_%H%M%S").to_string()
    }

    /// PascalCase 转换: sys_category → SysCategory
    fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect()
    }

    /// Rust 类型映射
    fn rust_type(field_type: &str, nullable: bool) -> String {
        let base = match field_type {
            "string" | "text" | "date" | "datetime" | "json" | "decimal" | "enum" => "String",
            "i8" => "i8",
            "i16" => "i16",
            "i32" => "i32",
            "i64" => "i64",
            "u64" => "u64",
            "f32" => "f32",
            "f64" => "f64",
            "bool" => "bool",
            _ => "String",
        };
        if nullable {
            format!("Option<{}>", base)
        } else {
            base.to_string()
        }
    }

    // ===== 后端代码生成 =====

    fn gen_migration(config: &GeneratorConfig) -> GeneratedFile {
        let ts = Self::timestamp();
        let table_ident = Self::to_pascal_case(&config.table_name);

        let mut cols = String::new();
        for field in &config.fields {
            let field_name = Self::to_pascal_case(&field.name);
            if field.primary_key {
                cols.push_str(&format!(".col(pk_auto({}::{}))\n", table_ident, field_name));
            } else {
                cols.push_str(&format!(
                    ".col(string_null({}::{}).comment(\"{}\"))\n",
                    table_ident, field_name, field.comment
                ));
            }
        }

        let iden_variants: Vec<String> = config
            .fields
            .iter()
            .map(|f| format!("    {},", Self::to_pascal_case(&f.name)))
            .collect();

        let content = format!(
            "use sea_orm_migration::prelude::*;\n\n\
             #[derive(DeriveMigrationName)]\n\
             pub struct Migration;\n\n\
             #[async_trait::async_trait]\n\
             impl MigrationTrait for Migration {{\n\
             \x20   async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{\n\
             \x20       manager\n\
             \x20           .create_table(\n\
             \x20               Table::create()\n\
             \x20                   .table({}::Table)\n\
             \x20                   .if_not_exists()\n\
             \x20                   {}\n\
             \x20                   .to_owned(),\n\
             \x20           )\n\
             \x20           .await\n\
             \x20   }}\n\n\
             \x20   async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{\n\
             \x20       manager\n\
             \x20           .drop_table(Table::drop().table({}::Table).to_owned())\n\
             \x20           .await\n\
             \x20   }}\n\
             }}\n\n\
             #[derive(Iden)]\n\
             enum {} {{\n\
             \x20   Table,\n\
             {}\n\
             }}\n",
            table_ident,
            cols.trim_end(),
            table_ident,
            table_ident,
            iden_variants.join("\n"),
        );

        GeneratedFile {
            file_name: format!("m{}_create_{}.rs", ts, config.table_name),
            file_path: format!("data/migration/src/m{}_create_{}.rs", ts, config.table_name),
            content,
            file_type: "rust".into(),
        }
    }

        fn gen_dao(config: &GeneratorConfig) -> GeneratedFile {
        let mut fields = String::new();
        for f in &config.fields {
            let rust_type = Self::rust_type(&f.field_type, f.nullable);
            if f.primary_key {
                fields.push_str(&format!("    #[sea_orm(primary_key)]\n    pub {}: {},\n", f.name, rust_type));
            } else {
                fields.push_str(&format!("    pub {}: {},\n", f.name, rust_type));
            }
        }

        let content = format!(
            "use sea_orm::entity::prelude::*;\n\
             use serde::{{Deserialize, Serialize}};\n\
             use utoipa::ToSchema;\n\n\
             #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]\n\
             #[sea_orm(table_name = \"{}\")]\n\
             pub struct Model {{\n\
             {}\n\
             }}\n\n\
             #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]\n\
             pub enum Relation {{}}\n\n\
             impl ActiveModelBehavior for ActiveModel {{}}\n",
            config.table_name,
            fields.trim_end(),
        );

        GeneratedFile {
            file_name: format!("{}.rs", config.resource),
            file_path: format!("data/model/src/dao/{}.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    fn gen_dto(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);

        let form_fields: Vec<String> = config.fields.iter()
            .filter(|f| f.form && !f.primary_key)
            .map(|f| {
                let rust_type = Self::rust_type(&f.field_type, true);
                format!("    pub {}: {},", f.name, rust_type)
            })
            .collect();

        let search_fields: Vec<String> = config.fields.iter()
            .filter(|f| f.search)
            .map(|f| format!("    pub {}: Option<String>,", f.name))
            .collect();

        let content = format!(
            "use utoipa::ToSchema;\n\n\
             /// {} 插入 DTO\n\
             #[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]\n\
             pub struct {}InsertDTO {{\n\
             {}\n\
             }}\n\n\
             /// {} 更新 DTO\n\
             #[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]\n\
             pub struct {}UpdateDTO {{\n\
             {}\n\
             }}\n\n\
             /// {} 查询 DTO\n\
             #[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]\n\
             pub struct {}QueryDTO {{\n\
             {}\n\
             }}\n",
            config.module_cn, entity, form_fields.join("\n"),
            config.module_cn, entity, form_fields.join("\n"),
            config.module_cn, entity, search_fields.join("\n"),
        );

        GeneratedFile {
            file_name: format!("{}_dto.rs", config.resource),
            file_path: format!("data/model/src/dto/{}_dto.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    fn gen_service(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);
        let resource = &config.resource;
        let module_cn = &config.module_cn;

        let content = format!(
            "use std::sync::Arc;\n\n\
             use casbin::{{CachedEnforcer, CoreApi}};\n\
             use sea_orm::{{DatabaseConnection, EntityTrait, QueryFilter, Set}};\n\
             use tokio::sync::RwLock;\n\
             use model::dao::{entity};\n\
             use model::dto::page_dto::{{PageRequest, PageResponse}};\n\
             use model::dto::{resource}_dto::{{{entity}InsertDTO, {entity}UpdateDTO}};\n\
             use utils::prelude::ServiceError;\n\
             use crate::enforcer::reload_policy_with;\n\n\
             pub struct {entity}Service;\n\n\
             impl {entity}Service {{\n\
             \x20   pub async fn insert(db: &DatabaseConnection, data: {entity}InsertDTO) -> Result<{entity}::Model, ServiceError> {{\n\
             \x20       let active = {entity}::ActiveModel {{\n\
             \x20           ..Default::default()\n\
             \x20       }};\n\
             \x20       let result = {entity}::Entity::insert(active).exec(db).await?;\n\
             \x20       Self::get_by_id(db, result.last_insert_id).await\n\
             \x20   }}\n\n\
             \x20   pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<{entity}::Model>, ServiceError> {{\n\
             \x20       let page = query.page.unwrap_or(1);\n\
             \x20       let page_size = query.page_size.unwrap_or(10);\n\
             \x20       let mut q = {entity}::Entity::find();\n\
             \x20       if let Some(keyword) = &query.keyword {{\n\
             \x20           q = q.filter({entity}::Column::Name.contains(keyword));\n\
             \x20       }}\n\
             \x20       let total = q.clone().count(db).await?;\n\
             \x20       let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;\n\
             \x20       Ok(PageResponse {{ list, total, page, page_size }})\n\
             \x20   }}\n\n\
             \x20   pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<{entity}::Model, ServiceError> {{\n\
             \x20       {entity}::Entity::find_by_id(id)\n\
             \x20           .one(db)\n\
             \x20           .await?\n\
             \x20           .ok_or_else(|| ServiceError::NotFound(\"{module_cn}不存在\".into()))\n\
             \x20   }}\n\n\
             \x20   pub async fn update(db: &DatabaseConnection, id: i32, data: {entity}UpdateDTO) -> Result<{entity}::Model, ServiceError> {{\n\
             \x20       let model: {entity}::ActiveModel = {entity}::Entity::find_by_id(id)\n\
             \x20           .one(db)\n\
             \x20           .await?\n\
             \x20           .ok_or_else(|| ServiceError::NotFound(\"{module_cn}不存在\".into()))?\n\
             \x20           .into();\n\
             \x20       let result = model.update(db).await?;\n\
             \x20       Ok(result)\n\
             \x20   }}\n\n\
             \x20   pub async fn delete(db: &DatabaseConnection, enforcer: &Arc<RwLock<CachedEnforcer>>, id: i32) -> Result<(), ServiceError> {{\n\
             \x20       {entity}::Entity::delete_by_id(id).exec(db).await?;\n\
             \x20       reload_policy_with(enforcer).await;\n\
             \x20       Ok(())\n\
             \x20   }}\n\
             }}\n",
            entity = entity,
            resource = resource,
            module_cn = module_cn,
        );

        GeneratedFile {
            file_name: format!("{}_service.rs", config.resource),
            file_path: format!("service/src/{}_service.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    fn gen_api(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);
        let resource = &config.resource;
        let module_cn = &config.module_cn;

        let content = format!(
            "use axum::{{extract::{{Path, State, Query}}, Json, routing::{{get, post, put, delete}}, Router}};\n\
             use model::dto::page_dto::PageRequest;\n\
             use model::dto::{resource}_dto::{{{entity}InsertDTO, {entity}UpdateDTO}};\n\
             use service::{resource}_service::{entity}Service;\n\
             use utils::prelude::{{AppError, AppState, R}};\n\n\
             pub fn routes() -> Router<AppState> {{\n\
             \x20   Router::new()\n\
             \x20       .route(\"/api/{resource}\", post(create))\n\
             \x20       .route(\"/api/{resource}/list\", get(list))\n\
             \x20       .route(\"/api/{resource}/{{id}}\", get(get_by_id).put(update).delete(delete))\n\
             }}\n\n\
             /// 创建{module_cn}\n\
             #[utoipa::path(\n\
             \x20   post, path = \"/api/{resource}\",\n\
             \x20   request_body = {entity}InsertDTO,\n\
             \x20   responses((status = 200, body = R<serde_json::Value>)),\n\
             \x20   tag = \"{module_cn}管理\"\n\
             )]\n\
             pub async fn create(\n\
             \x20   State(state): State<AppState>,\n\
             \x20   Json(data): Json<{entity}InsertDTO>,\n\
             ) -> Result<impl axum::response::IntoResponse, AppError> {{\n\
             \x20   let result = {entity}Service::insert(&state.db, data).await?;\n\
             \x20   Ok(R::ok(result))\n\
             }}\n\n\
             /// {module_cn}列表\n\
             #[utoipa::path(\n\
             \x20   get, path = \"/api/{resource}/list\",\n\
             \x20   params(PageRequest),\n\
             \x20   responses((status = 200, body = R<serde_json::Value>)),\n\
             \x20   tag = \"{module_cn}管理\"\n\
             )]\n\
             pub async fn list(\n\
             \x20   State(state): State<AppState>,\n\
             \x20   Query(query): Query<PageRequest>,\n\
             ) -> Result<impl axum::response::IntoResponse, AppError> {{\n\
             \x20   let result = {entity}Service::list(&state.db, query).await?;\n\
             \x20   Ok(R::ok(result))\n\
             }}\n\n\
             /// 获取{module_cn}详情\n\
             #[utoipa::path(\n\
             \x20   get, path = \"/api/{resource}/{{id}}\",\n\
             \x20   responses((status = 200, body = R<serde_json::Value>)),\n\
             \x20   tag = \"{module_cn}管理\"\n\
             )]\n\
             pub async fn get_by_id(\n\
             \x20   State(state): State<AppState>,\n\
             \x20   Path(id): Path<i32>,\n\
             ) -> Result<impl axum::response::IntoResponse, AppError> {{\n\
             \x20   let result = {entity}Service::get_by_id(&state.db, id).await?;\n\
             \x20   Ok(R::ok(result))\n\
             }}\n\n\
             /// 更新{module_cn}\n\
             #[utoipa::path(\n\
             \x20   put, path = \"/api/{resource}/{{id}}\",\n\
             \x20   request_body = {entity}UpdateDTO,\n\
             \x20   responses((status = 200, body = R<serde_json::Value>)),\n\
             \x20   tag = \"{module_cn}管理\"\n\
             )]\n\
             pub async fn update(\n\
             \x20   State(state): State<AppState>,\n\
             \x20   Path(id): Path<i32>,\n\
             \x20   Json(data): Json<{entity}UpdateDTO>,\n\
             ) -> Result<impl axum::response::IntoResponse, AppError> {{\n\
             \x20   let result = {entity}Service::update(&state.db, id, data).await?;\n\
             \x20   Ok(R::ok(result))\n\
             }}\n\n\
             /// 删除{module_cn}\n\
             #[utoipa::path(\n\
             \x20   delete, path = \"/api/{resource}/{{id}}\",\n\
             \x20   responses((status = 200, body = R<serde_json::Value>)),\n\
             \x20   tag = \"{module_cn}管理\"\n\
             )]\n\
             pub async fn delete(\n\
             \x20   State(state): State<AppState>,\n\
             \x20   Path(id): Path<i32>,\n\
             ) -> Result<impl axum::response::IntoResponse, AppError> {{\n\
             \x20   {entity}Service::delete(&state.db, &state.enforcer, id).await?;\n\
             \x20   Ok(R::ok(()))\n\
             }}\n",
            entity = entity,
            resource = resource,
            module_cn = module_cn,
        );

        GeneratedFile {
            file_name: format!("{}_api.rs", config.resource),
            file_path: format!("api/src/{}_api.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    // ===== 前端代码生成 =====

    fn gen_model(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);
        let module_cn = &config.module_cn;

        let mut fields = String::new();
        for f in &config.fields {
            let base_type = Self::rust_type(&f.field_type, false);
            fields.push_str(&format!("    #[serde(default)]\n    pub {}: Option<{}>,\n", f.name, base_type));
        }

        let content = format!(
            "use serde::{{Deserialize, Serialize}};\n\n\
             /// {module_cn} — 对应后端 {table}::Model\n\
             #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]\n\
             pub struct {entity} {{\n\
             {fields}\n\
             }}\n\n\
             /// {module_cn} 插入 DTO\n\
             #[derive(Debug, Clone, Serialize, Deserialize, Default)]\n\
             pub struct {entity}InsertDTO {{\n\
             {fields}\n\
             }}\n\n\
             /// {module_cn} 更新 DTO\n\
             #[derive(Debug, Clone, Serialize, Deserialize, Default)]\n\
             pub struct {entity}UpdateDTO {{\n\
             {fields}\n\
             }}\n",
            module_cn = module_cn,
            table = config.table_name,
            entity = entity,
            fields = fields.trim_end(),
        );

        GeneratedFile {
            file_name: format!("{}.rs", config.resource),
            file_path: format!("src/models/{}.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    fn gen_api_call(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);
        let resource = &config.resource;
        let module_cn = &config.module_cn;

        let content = format!(
            "use crate::http;\n\
             use crate::models::{resource}::{{{entity}, {entity}InsertDTO, {entity}UpdateDTO}};\n\
             use crate::models::common::PageResponse;\n\n\
             /// 获取{module_cn}列表\n\
             pub async fn list(page: Option<u32>, page_size: Option<u32>, keyword: Option<&str>) -> Result<PageResponse<{entity}>, String> {{\n\
             \x20   let query = http::build_page_query(page, page_size, keyword);\n\
             \x20   let path = format!(\"/api/{resource}/list?{{}}\", query);\n\
             \x20   http::get(&path).await\n\
             }}\n\n\
             /// 获取{module_cn}详情\n\
             pub async fn get_by_id(id: i32) -> Result<{entity}, String> {{\n\
             \x20   http::get(&format!(\"/api/{resource}/{{}}\", id)).await\n\
             }}\n\n\
             /// 创建{module_cn}\n\
             pub async fn create(data: &{entity}InsertDTO) -> Result<{entity}, String> {{\n\
             \x20   http::post(\"/api/{resource}\", data).await\n\
             }}\n\n\
             /// 更新{module_cn}\n\
             pub async fn update(id: i32, data: &{entity}UpdateDTO) -> Result<{entity}, String> {{\n\
             \x20   http::put(&format!(\"/api/{resource}/{{}}\", id), data).await\n\
             }}\n\n\
             /// 删除{module_cn}\n\
             pub async fn delete(id: i32) -> Result<(), String> {{\n\
             \x20   http::delete_void(&format!(\"/api/{resource}/{{}}\", id)).await\n\
             }}\n",
            entity = entity,
            resource = resource,
            module_cn = module_cn,
        );

        GeneratedFile {
            file_name: format!("{}.rs", config.resource),
            file_path: format!("src/api/{}.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }

    fn gen_component(config: &GeneratorConfig) -> GeneratedFile {
        let entity = Self::to_pascal_case(&config.resource);
        let resource = &config.resource;
        let module_cn = &config.module_cn;

        let table_columns: Vec<String> = config.fields.iter()
            .filter(|f| f.table)
            .map(|f| format!("                    el_table_column {{ label: \"{}\", prop: \"{}\" }}", f.comment, f.name))
            .collect();

        let content = format!(
            "use dioxus::prelude::*;\n\
             use dioxus_element_plug::prelude::*;\n\
             use crate::api::{resource} as api;\n\
             use crate::models::{resource}::{entity};\n\n\
             /// {module_cn} 管理页面\n\
             #[component]\n\
             pub fn {entity}Manage() -> Element {{\n\
             \x20   let mut list = use_signal(Vec::<{entity}>::new);\n\
             \x20   let mut total = use_signal(|| 0u64);\n\
             \x20   let mut current_page = use_signal(|| 1u32);\n\
             \x20   let mut loading = use_signal(|| false);\n\n\
             \x20   use_effect(use_reactive!(|current_page| async move {{\n\
             \x20       loading.set(true);\n\
             \x20       match api::list(Some(current_page), Some(10), None).await {{\n\
             \x20           Ok(resp) => {{ list.set(resp.list); total.set(resp.total); }}\n\
             \x20           Err(e) => tracing::error!(\"加载失败: {{}}\", e),\n\
             \x20       }}\n\
             \x20       loading.set(false);\n\
             \x20   }}));\n\n\
             \x20   rsx! {{\n\
             \x20       div {{ class: \"page-container\",\n\
             \x20           el_table {{\n\
             \x20               data: list(),\n\
             {table_columns}\n\
             \x20           }}\n\
             \x20           el_pagination {{\n\
             \x20               total: total(),\n\
             \x20               current_page: current_page(),\n\
             \x20               page_size: 10,\n\
             \x20               on_change: move |page| current_page.set(page),\n\
             \x20           }}\n\
             \x20       }}\n\
             \x20   }}\n\
             }}\n",
            entity = entity,
            resource = resource,
            module_cn = module_cn,
            table_columns = table_columns.join("\n"),
        );

        GeneratedFile {
            file_name: format!("{}_manage.rs", config.resource),
            file_path: format!("src/components/{}_manage.rs", config.resource),
            content,
            file_type: "rust".into(),
        }
    }
}
