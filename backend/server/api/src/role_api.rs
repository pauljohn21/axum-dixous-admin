use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_role;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use service::sys_role_service::SysRoleService;
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    post,
    path = "/api/role",
    request_body = SysRoleInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_role::Model>)),
    tag = "角色管理"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysRoleInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::insert(&state.db, data).await?;
    Ok(R::ok(role))
}

#[utoipa::path(
    get,
    path = "/api/role/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_role::Model>>)),
    tag = "角色管理"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysRoleService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/role/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_role::Model>)),
    tag = "角色管理"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::get_by_id(&state.db, id).await?;
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
pub async fn update(State(state): State<AppState>, Path(id): Path<i32>, Json(data): Json<SysRoleUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let role = SysRoleService::update(&state.db, id, data).await?;
    Ok(R::ok(role))
}

#[utoipa::path(
    delete,
    path = "/api/role/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "角色管理"
)]
pub async fn delete_role(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    SysRoleService::delete(&state.db, id).await?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/role", post(create))
        .route("/api/role/list", get(list))
        .route("/api/role/{id}", get(get_by_id).put(update).delete(delete_role))
}
