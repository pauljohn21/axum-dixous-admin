use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_role_menus;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_role_menu_dto::SysRoleMenuInsertDTO;
use service::sys_role_menu_service::SysRoleMenuService;
use utils::prelude::{AppError, R, AppState};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct CompositeIdPath {
    pub sys_base_menu_id: u64,
    pub sys_role_role_id: u64,
}

#[utoipa::path(
    post,
    path = "/api/roleMenu",
    request_body = SysRoleMenuInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_role_menus::Model>)),
    tag = "角色菜单"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysRoleMenuInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = SysRoleMenuService::insert(&state.db, data).await?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/roleMenu/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_role_menus::Model>>)),
    tag = "角色菜单"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysRoleMenuService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/roleMenu/{sys_base_menu_id}/{sys_role_role_id}",
    params(
        ("sys_base_menu_id" = u64, Path, description = "菜单ID"),
        ("sys_role_role_id" = u64, Path, description = "角色ID")
    ),
    responses((status = 200, description = "成功", body = R<sys_role_menus::Model>)),
    tag = "角色菜单"
)]
pub async fn get_by_composite_id(State(state): State<AppState>, Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    let record = SysRoleMenuService::get_by_composite_id(&state.db, path.sys_base_menu_id, path.sys_role_role_id)
        .await
        ?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/roleMenu/{sys_base_menu_id}/{sys_role_role_id}",
    params(
        ("sys_base_menu_id" = u64, Path, description = "菜单ID"),
        ("sys_role_role_id" = u64, Path, description = "角色ID")
    ),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "角色菜单"
)]
pub async fn delete_role_menu(State(state): State<AppState>, Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    SysRoleMenuService::delete(&state.db, path.sys_base_menu_id, path.sys_role_role_id)
        .await
        ?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/roleMenu", post(create))
        .route("/api/roleMenu/list", get(list))
        .route("/api/roleMenu/{sys_base_menu_id}/{sys_role_role_id}", get(get_by_composite_id).delete(delete_role_menu))
}
