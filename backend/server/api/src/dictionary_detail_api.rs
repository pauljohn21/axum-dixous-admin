use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_dictionary_details;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_dictionary_detail_dto::{SysDictionaryDetailInsertDTO, SysDictionaryDetailUpdateDTO};
use service::sys_dictionary_detail_service::SysDictionaryDetailService;
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    post,
    path = "/api/dictionaryDetail",
    request_body = SysDictionaryDetailInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_dictionary_details::Model>)),
    tag = "字典详情"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysDictionaryDetailInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let detail = SysDictionaryDetailService::insert(&state.db, data).await?;
    Ok(R::ok(detail))
}

#[utoipa::path(
    get,
    path = "/api/dictionaryDetail/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_dictionary_details::Model>>)),
    tag = "字典详情"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysDictionaryDetailService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/dictionaryDetail/{id}",
    params(("id" = u64, Path, description = "字典详情ID")),
    responses((status = 200, description = "成功", body = R<sys_dictionary_details::Model>)),
    tag = "字典详情"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let detail = SysDictionaryDetailService::get_by_id(&state.db, id).await?;
    Ok(R::ok(detail))
}

#[utoipa::path(
    put,
    path = "/api/dictionaryDetail/{id}",
    params(("id" = u64, Path, description = "字典详情ID")),
    request_body = SysDictionaryDetailUpdateDTO,
    responses((status = 200, description = "成功", body = R<sys_dictionary_details::Model>)),
    tag = "字典详情"
)]
pub async fn update(State(state): State<AppState>, Path(id): Path<u64>, Json(data): Json<SysDictionaryDetailUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let detail = SysDictionaryDetailService::update(&state.db, id, data).await?;
    Ok(R::ok(detail))
}

#[utoipa::path(
    delete,
    path = "/api/dictionaryDetail/{id}",
    params(("id" = u64, Path, description = "字典详情ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "字典详情"
)]
pub async fn delete_detail(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysDictionaryDetailService::delete(&state.db, id).await?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/dictionaryDetail", post(create))
        .route("/api/dictionaryDetail/list", get(list))
        .route("/api/dictionaryDetail/{id}", get(get_by_id).put(update).delete(delete_detail))
}
