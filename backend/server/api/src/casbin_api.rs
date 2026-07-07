use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::routing::delete as delete_method;
use axum::Router;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use model::dao::casbin_rule;
use model::dto::page_dto::{PageRequest, PageResponse};
use service::casbin_service::{CasbinService, CreateCasbinRuleRequest, UpdateCasbinRuleRequest};
use utils::prelude::{AppError, R};

#[utoipa::path(
    get,
    path = "/api/casbin/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = CasbinService::list(query).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/casbin/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn get_by_id(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::get_by_id(id).await.map_err(|e| AppError::NotFoundError(e.to_string()))?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    post,
    path = "/api/casbin",
    request_body = CreateCasbinRuleRequest,
    responses((status = 200, description = "创建成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn create(Json(request): Json<CreateCasbinRuleRequest>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::create(request).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    put,
    path = "/api/casbin/{id}",
    request_body = UpdateCasbinRuleRequest,
    responses((status = 200, description = "更新成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn update(Path(id): Path<u64>, Json(request): Json<UpdateCasbinRuleRequest>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::update(id, request).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    delete,
    path = "/api/casbin/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "删除成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn delete(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    CasbinService::delete(id).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(()))
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct BatchDeleteRequest {
    pub ids: Vec<u64>,
}

#[utoipa::path(
    delete,
    path = "/api/casbin/batch",
    request_body = BatchDeleteRequest,
    responses((status = 200, description = "批量删除成功", body = R<u64>)),
    tag = "Casbin策略"
)]
pub async fn delete_batch(Json(request): Json<BatchDeleteRequest>) -> Result<impl IntoResponse, AppError> {
    let deleted_count = CasbinService::delete_batch(request.ids).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(deleted_count))
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CasbinInfo {
    pub path: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateRolePoliciesRequest {
    pub casbin_infos: Vec<CasbinInfo>,
}

#[utoipa::path(
    put,
    path = "/api/casbin/role/{role}",
    request_body = UpdateRolePoliciesRequest,
    params(("role" = String, Path, description = "角色名称")),
    responses((status = 200, description = "权限策略更新成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn update_role_policies(Path(role): Path<String>, Json(req): Json<UpdateRolePoliciesRequest>) -> Result<impl IntoResponse, AppError> {
    let policies: Vec<(String, String)> = req.casbin_infos
        .into_iter()
        .map(|info| (info.path, info.method))
        .collect();

    CasbinService::update_role_policies(&role, policies)
        .await
        .map_err(AppError::Anyhow)?;

    Ok(R::ok(()))
}

#[utoipa::path(
    get,
    path = "/api/casbin/role/{role}",
    params(("role" = String, Path, description = "角色名称")),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_policies_by_role(Path(role): Path<String>) -> Result<impl IntoResponse, AppError> {
    let policies = CasbinService::get_policy_by_role(&role).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(policies))
}

#[utoipa::path(
    get,
    path = "/api/casbin/user/{user}",
    params(("user" = String, Path, description = "用户名")),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_roles_for_user(Path(user): Path<String>) -> Result<impl IntoResponse, AppError> {
    let roles = CasbinService::get_roles_for_user(&user).await.map_err(AppError::Anyhow)?;
    Ok(R::ok(roles))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/casbin/list", get(list))
        .route("/api/casbin/{id}", get(get_by_id))
        .route("/api/casbin", post(create))
        .route("/api/casbin/{id}", put(update))
        .route("/api/casbin/{id}", delete_method(delete))
        .route("/api/casbin/batch", delete_method(delete_batch))
        .route("/api/casbin/role/{role}", get(get_policies_by_role))
        .route("/api/casbin/role/{role}", put(update_role_policies))
        .route("/api/casbin/user/{user}", get(get_roles_for_user))
}
