use axum::extract::{Extension, Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use auth_layer::Username;
use model::dao::sys_menu;
use model::dao::sys_user;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use serde::Serialize;
use utoipa::ToSchema;
use service::sys_menu_service::SysMenuService;
use service::sys_user_service::SysUserService;
use utils::prelude::{AppError, R, create_token};

#[derive(Serialize, ToSchema)]
pub struct LoginResp { pub token: String }

#[derive(Serialize, ToSchema)]
pub struct UserInfoResp {
    pub username: String,
    pub menus: Vec<sys_menu::Model>,
}

#[utoipa::path(
    post,
    path = "/api/user/login",
    request_body = LoginDTO,
    responses((status = 200, description = "成功", body = R<LoginResp>)),
    tag = "用户管理"
)]
pub async fn login(Json(data): Json<LoginDTO>) -> Result<impl IntoResponse, AppError> {
    let user = SysUserService::login(data).await.map_err(|e| AppError::AuthError(e.to_string()))?;
    let token = create_token(&user.username.clone().unwrap_or_default()).map_err(|e| AppError::AuthError(e.to_string()))?;
    Ok(R::ok(LoginResp { token }))
}

#[utoipa::path(
    post,
    path = "/api/user/register",
    request_body = SysUserInsertDTO,
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "用户管理"
)]
pub async fn register(Json(data): Json<SysUserInsertDTO>) -> Result<impl IntoResponse, AppError> {
    SysUserService::insert(data).await.map_err(|e| AppError::AuthError(e.to_string()))?;
    Ok(R::ok(()))
}

#[utoipa::path(
    get,
    path = "/api/user/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<sys_user::Model>>)),
    tag = "用户管理"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = SysUserService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/user/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_user::Model>)),
    tag = "用户管理"
)]
pub async fn get_by_id(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let user = SysUserService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(user))
}

#[utoipa::path(
    put,
    path = "/api/user/{id}",
    request_body = SysUserUpdateDTO,
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<sys_user::Model>)),
    tag = "用户管理"
)]
pub async fn update(Path(id): Path<i32>, Json(data): Json<SysUserUpdateDTO>) -> Result<impl IntoResponse, AppError> {
    let user = SysUserService::update(id, data).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(user))
}

#[utoipa::path(
    delete,
    path = "/api/user/{id}",
    params(("id" = i32, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "用户管理"
)]
pub async fn delete_user(Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    SysUserService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

#[utoipa::path(
    get,
    path = "/api/user/info",
    responses((status = 200, description = "成功", body = R<UserInfoResp>)),
    tag = "用户管理"
)]
pub async fn get_user_info(Extension(username): Extension<Username>) -> Result<impl IntoResponse, AppError> {
    let menus = SysMenuService::get_menus_by_username(&username.0)
        .await
        .map_err(AppError::Anyhow)?;
    Ok(R::ok(UserInfoResp {
        username: username.0,
        menus,
    }))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/user/list", get(list))
        .route("/api/user/register", post(register))
        .route("/api/user/info", get(get_user_info))
        .route("/api/user/{id}", get(get_by_id).put(update).delete(delete_user))
}
