use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_role;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use service::sys_role_service::SysRoleService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/role",
    request_body = SysRoleInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_role::Model>)),
    tag = "角色管理"
)]
pub async fn create(Json(data): Json<SysRoleInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(role))
}

#[utoipa::path(
    get,
    path = "/api/role/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_role::Model>>)),
    tag = "角色管理"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysRoleService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/role/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_role::Model>)),
    tag = "角色管理"
)]
pub async fn get_by_id(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(role))
}

#[utoipa::path(
    put,
    path = "/api/role/{id}",
    request_body = SysRoleUpdateDTO,
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_role::Model>)),
    tag = "角色管理"
)]
pub async fn update(Path(id): Path<i32>, Json(data): Json<SysRoleUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(role))
}

#[utoipa::path(
    delete,
    path = "/api/role/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "角色管理"
)]
pub async fn delete_role(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    SysRoleService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/role", post(create))
        .route("/api/role/list", get(list))
        .route("/api/role/{id}", get(get_by_id).put(update).delete(delete_role))
}
