use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_role_btns;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_role_btn_dto::SysRoleBtnInsertDTO;
use service::sys_role_btn_service::SysRoleBtnService;
use utils::prelude::{AppError, R};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct CompositeIdPath {
    pub role_id: u64,
    pub sys_menu_id: u64,
    pub sys_base_menu_btn_id: u64,
}

#[utoipa::path(
    post,
    path = "/api/roleBtn",
    request_body = SysRoleBtnInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_role_btns::Model>)),
    tag = "角色按钮"
)]
pub async fn create(Json(data): Json<SysRoleBtnInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = SysRoleBtnService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/roleBtn/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_role_btns::Model>>)),
    tag = "角色按钮"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysRoleBtnService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}",
    params(
        ("role_id" = u64, Path, description = "角色ID"),
        ("sys_menu_id" = u64, Path, description = "菜单ID"),
        ("sys_base_menu_btn_id" = u64, Path, description = "按钮ID")
    ),
    responses((status = 200, description = "成功", body = R<sys_role_btns::Model>)),
    tag = "角色按钮"
)]
pub async fn get_by_composite_id(Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    let record = SysRoleBtnService::get_by_composite_id(path.role_id, path.sys_menu_id, path.sys_base_menu_btn_id)
        .await
        .map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}",
    params(
        ("role_id" = u64, Path, description = "角色ID"),
        ("sys_menu_id" = u64, Path, description = "菜单ID"),
        ("sys_base_menu_btn_id" = u64, Path, description = "按钮ID")
    ),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "角色按钮"
)]
pub async fn delete_role_btn(Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    SysRoleBtnService::delete(path.role_id, path.sys_menu_id, path.sys_base_menu_btn_id)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/roleBtn", post(create))
        .route("/api/roleBtn/list", get(list))
        .route("/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}", get(get_by_composite_id).delete(delete_role_btn))
}
