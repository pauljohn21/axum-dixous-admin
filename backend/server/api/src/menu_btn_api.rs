use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use model::dao::sys_base_menu_btns;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_base_menu_btn_dto::{SysBaseMenuBtnInsertDTO, SysBaseMenuBtnUpdateDTO};
use service::sys_base_menu_btn_service::SysBaseMenuBtnService;
use utils::prelude::{AppError, R};

#[utoipa::path(
    post,
    path = "/api/menuBtn",
    request_body = SysBaseMenuBtnInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_base_menu_btns::Model>)),
    tag = "菜单按钮"
)]
pub async fn create(Json(data): Json<SysBaseMenuBtnInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let btn = SysBaseMenuBtnService::insert(data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(btn))
}

#[utoipa::path(
    get,
    path = "/api/menuBtn/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_base_menu_btns::Model>>)),
    tag = "菜单按钮"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysBaseMenuBtnService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/menuBtn/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_base_menu_btns::Model>)),
    tag = "菜单按钮"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let btn = SysBaseMenuBtnService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(btn))
}

#[utoipa::path(
    put,
    path = "/api/menuBtn/{id}",
    request_body = SysBaseMenuBtnUpdateDTO,
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_base_menu_btns::Model>)),
    tag = "菜单按钮"
)]
pub async fn update(Path(id): Path<u64>, Json(data): Json<SysBaseMenuBtnUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let btn = SysBaseMenuBtnService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(btn))
}

#[utoipa::path(
    delete,
    path = "/api/menuBtn/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "菜单按钮"
)]
pub async fn delete_btn(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    SysBaseMenuBtnService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/menuBtn", post(create))
        .route("/api/menuBtn/list", get(list))
        .route("/api/menuBtn/{id}", get(get_by_id).put(update).delete(delete_btn))
}
