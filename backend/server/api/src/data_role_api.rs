use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_data_role_id;
use model::dto::page_dto::PageRequest;
use model::dto::page_dto::PageResponse;
use model::dto::sys_data_role_dto::SysDataRoleInsertDTO;
use service::sys_data_role_service::SysDataRoleService;
use utils::prelude::{AppError, R, AppState};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct CompositeIdPath {
    pub sys_role_role_id: u64,
    pub data_role_id_role_id: u64,
}

#[utoipa::path(
    post,
    path = "/api/dataRole",
    request_body = SysDataRoleInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_data_role_id::Model>)),
    tag = "数据权限"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysDataRoleInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let record = SysDataRoleService::insert(&state.db, data).await?;
    Ok(R::ok(record))
}

#[utoipa::path(
    get,
    path = "/api/dataRole/list",
    params(("page" = Option<u64>, Query, description = "页码"), ("page_size" = Option<u64>, Query, description = "每页条数")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_data_role_id::Model>>)),
    tag = "数据权限"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysDataRoleService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/dataRole/{sys_role_role_id}/{data_role_id_role_id}",
    params(
        ("sys_role_role_id" = u64, Path, description = "角色ID"),
        ("data_role_id_role_id" = u64, Path, description = "数据权限角色ID")
    ),
    responses((status = 200, description = "成功", body = R<sys_data_role_id::Model>)),
    tag = "数据权限"
)]
pub async fn get_by_composite_id(State(state): State<AppState>, Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    let record = SysDataRoleService::get_by_composite_id(&state.db, path.sys_role_role_id, path.data_role_id_role_id)
        .await
        ?;
    Ok(R::ok(record))
}

#[utoipa::path(
    delete,
    path = "/api/dataRole/{sys_role_role_id}/{data_role_id_role_id}",
    params(
        ("sys_role_role_id" = u64, Path, description = "角色ID"),
        ("data_role_id_role_id" = u64, Path, description = "数据权限角色ID")
    ),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "数据权限"
)]
pub async fn delete_data_role(State(state): State<AppState>, Path(path): Path<CompositeIdPath>) -> Result<impl IntoResponse, AppError> {
    SysDataRoleService::delete(&state.db, path.sys_role_role_id, path.data_role_id_role_id)
        .await
        ?;
    Ok(R::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/dataRole", post(create))
        .route("/api/dataRole/list", get(list))
        .route("/api/dataRole/{sys_role_role_id}/{data_role_id_role_id}", get(get_by_composite_id).delete(delete_data_role))
}
