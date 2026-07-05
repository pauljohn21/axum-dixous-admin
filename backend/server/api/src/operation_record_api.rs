use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_operation_records;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_operation_record_dto::{SysOperationRecordInsertDTO, SysOperationRecordUpdateDTO};
use service::sys_operation_record_service::SysOperationRecordService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/operationRecord",
    request_body = SysOperationRecordInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_operation_records::Model>)),
    tag = "操作记录"
)]
pub async fn create(Json(data): Json<SysOperationRecordInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = SysOperationRecordService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/operationRecord/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_operation_records::Model>>)),
    tag = "操作记录"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysOperationRecordService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/operationRecord/{id}",
    params(("id" = u64, Path, description = "操作记录ID")),
    responses((status = 200, description = "成功", body = R<sys_operation_records::Model>)),
    tag = "操作记录"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let record = SysOperationRecordService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(record))
}

#[utoipa::path(
    put,
    path = "/api/operationRecord/{id}",
    params(("id" = u64, Path, description = "操作记录ID")),
    request_body = SysOperationRecordUpdateDTO,
    responses((status = 200, description = "成功", body = R<sys_operation_records::Model>)),
    tag = "操作记录"
)]
pub async fn update(Path(id): Path<u64>, Json(data): Json<SysOperationRecordUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let record = SysOperationRecordService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/operationRecord/{id}",
    params(("id" = u64, Path, description = "操作记录ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "操作记录"
)]
pub async fn delete_record(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysOperationRecordService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/operationRecord", post(create))
        .route("/api/operationRecord/list", get(list))
        .route("/api/operationRecord/{id}", get(get_by_id).put(update).delete(delete_record))
}
