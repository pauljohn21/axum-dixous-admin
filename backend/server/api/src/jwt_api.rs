use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::jwt_blacklists;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::jwt_blacklist_dto::{JwtBlacklistInsertDTO, JwtBlacklistUpdateDTO};
use service::jwt_blacklist_service::JwtBlacklistService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/jwtBlacklist",
    request_body = JwtBlacklistInsertDTO,
    responses((status = 200, description = "成功", body = R<jwt_blacklists::Model>)),
    tag = "JWT管理"
)]
pub async fn create(Json(data): Json<JwtBlacklistInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/jwtBlacklist/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<jwt_blacklists::Model>>)),
    tag = "JWT管理"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = JwtBlacklistService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/jwtBlacklist/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<jwt_blacklists::Model>)),
    tag = "JWT管理"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(record))
}

#[utoipa::path(
    put,
    path = "/api/jwtBlacklist/{id}",
    request_body = JwtBlacklistUpdateDTO,
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<jwt_blacklists::Model>)),
    tag = "JWT管理"
)]
pub async fn update(Path(id): Path<u64>, Json(data): Json<JwtBlacklistUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/jwtBlacklist/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "JWT管理"
)]
pub async fn delete_jwt(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    JwtBlacklistService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/jwtBlacklist", post(create))
        .route("/api/jwtBlacklist/list", get(list))
        .route("/api/jwtBlacklist/{id}", get(get_by_id).put(update).delete(delete_jwt))
}
