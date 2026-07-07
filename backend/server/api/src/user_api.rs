use axum::extract::{Extension, Path, Query, Request};
use axum::http::header::AUTHORIZATION;
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use auth_layer::Username;
use model::dao::sys_menu;
use model::dao::sys_user;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use service::jwt_blacklist_service::JwtBlacklistService;
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

/// 修改密码请求
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordDTO {
    pub old_password: String,
    pub new_password: String,
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

/// 退出登录 — 将当前 token 加入黑名单
#[utoipa::path(
    post,
    path = "/api/user/logout",
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "用户管理"
)]
pub async fn logout(req: Request) -> Result<impl IntoResponse, AppError> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // 将 token 加入 JWT 黑名单
            let dto = model::dto::jwt_blacklist_dto::JwtBlacklistInsertDTO {
                jwt: Some(token.to_string()),
            };
            let _ = JwtBlacklistService::insert(dto).await;
        }
    }
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

/// 修改密码
#[utoipa::path(
    put,
    path = "/api/user/change_password",
    request_body = ChangePasswordDTO,
    responses((status = 200, description = "成功", body = R<serde_json::Value>)),
    tag = "用户管理"
)]
pub async fn change_password(
    Extension(username): Extension<Username>,
    Json(data): Json<ChangePasswordDTO>,
) -> Result<impl IntoResponse, AppError> {
    SysUserService::change_password(&username.0, data.old_password, data.new_password)
        .await
        .map_err(|e| AppError::AuthError(e.to_string()))?;
    Ok(R::ok(()))
}

/// 仪表盘统计数据
#[utoipa::path(
    get,
    path = "/api/dashboard/stats",
    responses((status = 200, description = "成功", body = R<service::DashboardStats>)),
    tag = "用户管理"
)]
pub async fn dashboard_stats() -> Result<impl IntoResponse, AppError> {
    let stats = SysUserService::dashboard_stats().await.map_err(AppError::Anyhow)?;
    Ok(R::ok(stats))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/user/list", get(list))
        .route("/api/user/register", post(register))
        .route("/api/user/logout", post(logout))
        .route("/api/user/info", get(get_user_info))
        .route("/api/user/change_password", axum::routing::put(change_password))
        .route("/api/user/{id}", get(get_by_id).put(update).delete(delete_user))
        .route("/api/dashboard/stats", get(dashboard_stats))
}
