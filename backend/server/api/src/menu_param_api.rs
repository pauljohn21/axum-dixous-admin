use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_base_menu_parameters;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_base_menu_param_dto::{SysBaseMenuParamInsertDTO, SysBaseMenuParamUpdateDTO};
use service::sys_base_menu_param_service::SysBaseMenuParamService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/menuParam",
    request_body = SysBaseMenuParamInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_base_menu_parameters::Model>)),
    tag = "菜单参数"
)]
pub async fn create(Json(data): Json<SysBaseMenuParamInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(param))
}

#[utoipa::path(
    get,
    path = "/api/menuParam/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_base_menu_parameters::Model>>)),
    tag = "菜单参数"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysBaseMenuParamService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/menuParam/{id}",
    params(("id" = u64, Path, description = "菜单参数ID")),
    responses((status = 200, description = "成功", body = R<sys_base_menu_parameters::Model>)),
    tag = "菜单参数"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
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
pub async fn update(Path(id): Path<u64>, Json(data): Json<SysBaseMenuParamUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let param = SysBaseMenuParamService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(param))
}

#[utoipa::path(
    delete,
    path = "/api/menuParam/{id}",
    params(("id" = u64, Path, description = "菜单参数ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "菜单参数"
)]
pub async fn delete_param(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysBaseMenuParamService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/menuParam", post(create))
        .route("/api/menuParam/list", get(list))
        .route("/api/menuParam/{id}", get(get_by_id).put(update).delete(delete_param))
}
