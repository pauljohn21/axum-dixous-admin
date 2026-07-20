use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::jwt_blacklists;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::jwt_blacklist_dto::{JwtBlacklistInsertDTO, JwtBlacklistUpdateDTO};
use service::jwt_blacklist_service::JwtBlacklistService;
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    post,
    path = "/api/jwtBlacklist",
    request_body = JwtBlacklistInsertDTO,
    responses((status = 200, description = "成功", body = R<jwt_blacklists::Model>)),
    tag = "JWT管理"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<JwtBlacklistInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::insert(&state.db, data).await?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/jwtBlacklist/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<jwt_blacklists::Model>>)),
    tag = "JWT管理"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = JwtBlacklistService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/jwtBlacklist/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<jwt_blacklists::Model>)),
    tag = "JWT管理"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::get_by_id(&state.db, id).await?;
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
pub async fn update(State(state): State<AppState>, Path(id): Path<u64>, Json(data): Json<JwtBlacklistUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let record = JwtBlacklistService::update(&state.db, id, data).await?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/jwtBlacklist/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "JWT管理"
)]
pub async fn delete_jwt(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    JwtBlacklistService::delete(&state.db, id).await?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/jwtBlacklist", post(create))
        .route("/api/jwtBlacklist/list", get(list))
        .route("/api/jwtBlacklist/{id}", get(get_by_id).put(update).delete(delete_jwt))
}
