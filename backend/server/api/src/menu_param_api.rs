use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_base_menu_parameters;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_base_menu_param_dto::{SysBaseMenuParamInsertDTO, SysBaseMenuParamUpdateDTO};
use service::sys_base_menu_param_service::SysBaseMenuParamService;
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    post,
    path = "/api/menuParam",
    request_body = SysBaseMenuParamInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_base_menu_parameters::Model>)),
    tag = "菜单参数"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysBaseMenuParamInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::insert(&state.db, data).await?;
    Ok(R::ok(param))
}

#[utoipa::path(
    get,
    path = "/api/menuParam/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_base_menu_parameters::Model>>)),
    tag = "菜单参数"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysBaseMenuParamService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/menuParam/{id}",
    params(("id" = u64, Path, description = "菜单参数ID")),
    responses((status = 200, description = "成功", body = R<sys_base_menu_parameters::Model>)),
    tag = "菜单参数"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::get_by_id(&state.db, id).await?;
    Ok(R::ok(param))
}

#[utoipa::path(
    put,
    path = "/api/menuParam/{id}",
    params(("id" = u64, Path, description = "菜单参数ID")),
    request_body = SysBaseMenuParamUpdateDTO,
    responses((status = 200, description = "成功", body = R<sys_base_menu_parameters::Model>)),
    tag = "菜单参数"
)]
pub async fn update(State(state): State<AppState>, Path(id): Path<u64>, Json(data): Json<SysBaseMenuParamUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::update(&state.db, id, data).await?;
    Ok(R::ok(param))
}

#[utoipa::path(
    delete,
    path = "/api/menuParam/{id}",
    params(("id" = u64, Path, description = "菜单参数ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "菜单参数"
)]
pub async fn delete_param(State(state): State<AppState>, Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysBaseMenuParamService::delete(&state.db, id).await?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/menuParam", post(create))
        .route("/api/menuParam/list", get(list))
        .route("/api/menuParam/{id}", get(get_by_id).put(update).delete(delete_param))
}
