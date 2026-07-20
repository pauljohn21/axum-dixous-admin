use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_dictionaries;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_dictionary_dto::{SysDictionaryInsertDTO, SysDictionaryUpdateDTO};
use service::sys_dictionary_service::SysDictionaryService;
use utils::cache::keys;
use utils::prelude::{AppError, Cache, R, AppState};

#[utoipa::path(
    post,
    path = "/api/dictionary",
    request_body = SysDictionaryInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_dictionaries::Model>)),
    tag = "字典管理"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysDictionaryInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let dict = SysDictionaryService::insert(&state.db, data).await?;
    Cache::del(&mut state.redis.clone(), keys::DASHBOARD_STATS).await;
    Ok(R::ok(dict))
}

#[utoipa::path(
    get,
    path = "/api/dictionary/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_dictionaries::Model>>)),
    tag = "字典管理"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysDictionaryService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/dictionary/{id}",
    params(("id" = u64, Path, description = "字典ID")),
    responses((status = 200, description = "成功", body = R<sys_dictionaries::Model>)),
    tag = "字典管理"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let dict = SysDictionaryService::get_by_id(&state.db, id).await?;
    Ok(R::ok(dict))
}

#[utoipa::path(
    put,
    path = "/api/dictionary/{id}",
    params(("id" = u64, Path, description = "字典ID")),
    request_body = SysDictionaryUpdateDTO,
    responses((status = 200, description = "成功", body = R<sys_dictionaries::Model>)),
    tag = "字典管理"
)]
pub async fn update(State(state): State<AppState>, Path(id): Path<u64>, Json(data): Json<SysDictionaryUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let dict = SysDictionaryService::update(&state.db, id, data).await?;
    Cache::del(&mut state.redis.clone(), keys::DASHBOARD_STATS).await;
    Ok(R::ok(dict))
}

#[utoipa::path(
    delete,
    path = "/api/dictionary/{id}",
    params(("id" = u64, Path, description = "字典ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "字典管理"
)]
pub async fn delete_dict(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysDictionaryService::delete(&state.db, id).await?;
    Cache::del(&mut state.redis.clone(), keys::DASHBOARD_STATS).await;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/dictionary", post(create))
        .route("/api/dictionary/list", get(list))
        .route("/api/dictionary/{id}", get(get_by_id).put(update).delete(delete_dict))
}
