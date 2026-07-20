use axum::extract::{Extension, Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use auth_layer::Username;
use model::dao::sys_menu;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use service::sys_menu_service::SysMenuService;
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    post,
    path = "/api/menu",
    request_body = SysMenuInsertDTO,
    responses((status = 200, description = "成功", body = R<sys_menu::Model>)),
    tag = "菜单管理"
)]
pub async fn create(State(state): State<AppState>, Json(data): Json<SysMenuInsertDTO>) -> Result<impl IntoResponse, AppError> {
    let menu = SysMenuService::insert(&state.db, data).await?;
    Ok(R::ok(menu))
}

#[utoipa::path(
    get,
    path = "/api/menu/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_menu::Model>>)),
    tag = "菜单管理"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysMenuService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/menu/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_menu::Model>)),
    tag = "菜单管理"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let menu = SysMenuService::get_by_id(&state.db, id).await?;
    Ok(R::ok(menu))
}

#[utoipa::path(
    put,
    path = "/api/menu/{id}",
    request_body = SysMenuUpdateDTO,
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_menu::Model>)),
    tag = "菜单管理"
)]
pub async fn update(State(state): State<AppState>, Path(id): Path<i32>, Json(data): Json<SysMenuUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let menu = SysMenuService::update(&state.db, id, data).await?;
    Ok(R::ok(menu))
}

#[utoipa::path(
    delete,
    path = "/api/menu/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "菜单管理"
)]
pub async fn delete_menu(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    SysMenuService::delete(&state.db, id).await?;
    Ok(R::ok(()))
}

#[utoipa::path(
    get,
    path = "/api/menu/user",
    responses((status = 200, description = "成功", body = R<Vec<sys_menu::Model>>)),
    tag = "菜单管理"
)]
pub async fn get_user_menus(State(state): State<AppState>, Extension(username): Extension<Username>) -> Result<impl IntoResponse, AppError> {
    let menus = SysMenuService::get_menus_by_username(&state.db, &username.0)
        .await?;
    Ok(R::ok(menus))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/menu", post(create))
        .route("/api/menu/list", get(list))
        .route("/api/menu/user", get(get_user_menus))
        .route("/api/menu/{id}", get(get_by_id).put(update).delete(delete_menu))
}
