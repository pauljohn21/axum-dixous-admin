//! 代码生成器 API
//!
//! 提供历史记录 CRUD、回滚、从数据库创建等功能。
//! 配置以 JSON 格式存储在 sys_generator_history.request 字段。

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;

use model::dao::sys_generator_history;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_generator_history_dto::{
    ColumnInfo, DatabaseInfo, GenerateFromTableDTO, GeneratedFile, GeneratorRollbackDTO,
    PreviewCodeDTO, PreviewCodeResponse, SysGeneratorHistoryInsertDTO, SysGeneratorHistoryUpdateDTO, TableInfo,
};
use service::generator_code_service::GeneratorCodeService;
use service::generator_history_service::GeneratorHistoryService;
use utils::prelude::{AppError, R};

// ===== 历史 CRUD =====

#[utoipa::path(
    post,
    path = "/api/generator/history",
    request_body = SysGeneratorHistoryInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_generator_history::Model>)),
    tag = "代码生成器"
)]
pub async fn create_history(
    Json(data): Json<SysGeneratorHistoryInsertDTO>,
) -> Result<impl IntoResponse, AppError> {
    let record = GeneratorHistoryService::insert(data)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/generator/history/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_generator_history::Model>>)),
    tag = "代码生成器"
)]
pub async fn list_history(
    Query(query): Query<PageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = GeneratorHistoryService::list(query)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/generator/history/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_generator_history::Model>)),
    tag = "代码生成器"
)]
pub async fn get_history_by_id(
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let record = GeneratorHistoryService::get_by_id(id)
        .await
        .map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/generator/history/{id}/meta",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<String>)),
    tag = "代码生成器"
)]
pub async fn get_history_meta(
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let meta = GeneratorHistoryService::get_meta(id)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(meta))
}

#[utoipa::path(
    put,
    path = "/api/generator/history/{id}",
    request_body = SysGeneratorHistoryUpdateDTO,
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_generator_history::Model>)),
    tag = "代码生成器"
)]
pub async fn update_history(
    Path(id): Path<u64>,
    Json(data): Json<SysGeneratorHistoryUpdateDTO>,
) -> Result<impl IntoResponse, AppError> {
    let record = GeneratorHistoryService::update(id, data)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/generator/history/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "代码生成器"
)]
pub async fn delete_history(
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    GeneratorHistoryService::delete(id)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

// ===== 回滚 =====

#[utoipa::path(
    post,
    path = "/api/generator/rollback",
    request_body = GeneratorRollbackDTO,
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "代码生成器"
)]
pub async fn rollback(
    Json(data): Json<GeneratorRollbackDTO>,
) -> Result<impl IntoResponse, AppError> {
    GeneratorHistoryService::rollback(data)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

// ===== 从数据库创建 =====

#[utoipa::path(
    get,
    path = "/api/generator/databases",
    responses((status = 200, description = "成功", body = R<Vec<DatabaseInfo>>)),
    tag = "代码生成器"
)]
pub async fn get_databases() -> Result<impl IntoResponse, AppError> {
    let databases = GeneratorHistoryService::get_databases()
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(databases))
}

#[utoipa::path(
    get,
    path = "/api/generator/tables",
    params(("db_name" = String, Query, description = "数据库名")),
    responses((status = 200, description = "成功", body = R<Vec<TableInfo>>)),
    tag = "代码生成器"
)]
pub async fn get_tables(
    Query(params): Query<GenerateFromTableDTO>,
) -> Result<impl IntoResponse, AppError> {
    let tables = GeneratorHistoryService::get_tables(&params.db_name)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(tables))
}

#[utoipa::path(
    get,
    path = "/api/generator/columns",
    params(
        ("db_name" = String, Query, description = "数据库名"),
        ("table_name" = String, Query, description = "表名"),
    ),
    responses((status = 200, description = "成功", body = R<Vec<ColumnInfo>>)),
    tag = "代码生成器"
)]
pub async fn get_columns(
    Query(params): Query<GenerateFromTableDTO>,
) -> Result<impl IntoResponse, AppError> {
    let columns = GeneratorHistoryService::get_columns(&params.db_name, &params.table_name)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(columns))
}

#[utoipa::path(
    post,
    path = "/api/generator/generate-from-table",
    request_body = GenerateFromTableDTO,
    responses((status = 200, description = "成功", body = R<String>)),
    tag = "代码生成器"
)]
pub async fn generate_from_table(
    Json(data): Json<GenerateFromTableDTO>,
) -> Result<impl IntoResponse, AppError> {
    let yaml = GeneratorHistoryService::generate_from_table(data)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(yaml))
}

// ===== 代码预览 =====

#[utoipa::path(
    post,
    path = "/api/generator/preview",
    request_body = PreviewCodeDTO,
    responses((status = 200, description = "成功", body = R<PreviewCodeResponse>)),
    tag = "代码生成器"
)]
pub async fn preview_code(
    Json(data): Json<PreviewCodeDTO>,
) -> Result<impl IntoResponse, AppError> {
    let result = GeneratorCodeService::preview_code(data)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

// ===== 路由 =====

pub fn routes() -> Router {
    Router::new()
        // 历史 CRUD
        .route("/api/generator/history", post(create_history))
        .route("/api/generator/history/list", get(list_history))
        .route(
            "/api/generator/history/{id}",
            get(get_history_by_id).put(update_history).delete(delete_history),
        )
        .route("/api/generator/history/{id}/meta", get(get_history_meta))
        // 回滚
        .route("/api/generator/rollback", post(rollback))
        // 从数据库创建
        .route("/api/generator/databases", get(get_databases))
        .route("/api/generator/tables", get(get_tables))
        .route("/api/generator/columns", get(get_columns))
        .route("/api/generator/generate-from-table", post(generate_from_table))
        // 代码预览
        .route("/api/generator/preview", post(preview_code))
}
