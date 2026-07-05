use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_apis;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_api_dto::{SysApiInsertDTO, SysApiUpdateDTO};
use service::sys_api_service::SysApiService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/apis",
    request_body = SysApiInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_apis::Model>)),
    tag = "API管理"
)]
pub async fn create(Json(data): Json<SysApiInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let api = SysApiService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(api))
}

#[utoipa::path(
    get,
    path = "/api/apis/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_apis::Model>>)),
    tag = "API管理"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysApiService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/apis/{id}",
    params(("id" = i64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_apis::Model>)),
    tag = "API管理"
)]
pub async fn get_by_id(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let api = SysApiService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(api))
}

#[utoipa::path(
    put,
    path = "/api/apis/{id}",
    request_body = SysApiUpdateDTO,
    params(("id" = i64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_apis::Model>)),
    tag = "API管理"
)]
pub async fn update(Path(id): Path<i64>, Json(data): Json<SysApiUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let api = SysApiService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(api))
}

#[utoipa::path(
    delete,
    path = "/api/apis/{id}",
    params(("id" = i64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "API管理"
)]
pub async fn delete_api(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    SysApiService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/apis", post(create))
        .route("/api/apis/list", get(list))
        .route("/api/apis/{id}", get(get_by_id).put(update).delete(delete_api))
}
