use axum::extract::{Path, Query, State};
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
use utils::prelude::{AppError, R, AppState};

#[utoipa::path(
    get,
    path = "/api/casbin/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses((status = 200, description = "成功", body = R<PageResponse<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn list(State(state): State<AppState>, Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> {
    let result = CasbinService::list(&state.db, query).await?;
    Ok(R::ok(result))
}

#[utoipa::path(
    get,
    path = "/api/casbin/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::get_by_id(&state.db, id).await?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    post,
    path = "/api/casbin",
    request_body = CreateCasbinRuleRequest,
    responses((status = 200, description = "创建成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn create(State(state): State<AppState>, Json(request): Json<CreateCasbinRuleRequest>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::create(&state.db, &state.enforcer, request).await?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    put,
    path = "/api/casbin/{id}",
    request_body = UpdateCasbinRuleRequest,
    responses((status = 200, description = "更新成功", body = R<casbin_rule::Model>)),
    tag = "Casbin策略"
)]
pub async fn update(State(state): State<AppState>, Path(id): Path<i64>, Json(request): Json<UpdateCasbinRuleRequest>) -> Result<impl IntoResponse, AppError> {
    let rule = CasbinService::update(&state.db, &state.enforcer, id, request).await?;
    Ok(R::ok(rule))
}

#[utoipa::path(
    delete,
    path = "/api/casbin/{id}",
    params(("id" = u64, Path, description = "ID")),
    responses((status = 200, description = "删除成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn delete(State(state): State<AppState>, Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    CasbinService::delete(&state.db, &state.enforcer, id).await?;
    Ok(R::ok(()))
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct BatchDeleteRequest {
    pub ids: Vec<i64>,
}

#[utoipa::path(
    delete,
    path = "/api/casbin/batch",
    request_body = BatchDeleteRequest,
    responses((status = 200, description = "批量删除成功", body = R<u64>)),
    tag = "Casbin策略"
)]
pub async fn delete_batch(State(state): State<AppState>, Json(request): Json<BatchDeleteRequest>) -> Result<impl IntoResponse, AppError> {
    let deleted_count = CasbinService::delete_batch(&state.db, request.ids).await?;
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
    path = "/api/casbin/role/{role}/domain/{domain}",
    request_body = UpdateRolePoliciesRequest,
    params(
        ("role" = String, Path, description = "角色名称"),
        ("domain" = String, Path, description = "域名称"),
    ),
    responses((status = 200, description = "权限策略更新成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn update_role_policies(State(state): State<AppState>, Path((role, domain)): Path<(String, String)>, Json(req): Json<UpdateRolePoliciesRequest>) -> Result<impl IntoResponse, AppError> {
    let policies: Vec<(String, String)> = req.casbin_infos
        .into_iter()
        .map(|info| (info.path, info.method))
        .collect();

    CasbinService::update_role_policies(&state.db, &state.enforcer, &role, &domain, policies)
        .await?;

    Ok(R::ok(()))
}

#[utoipa::path(
    get,
    path = "/api/casbin/role/{role}/domain/{domain}",
    params(
        ("role" = String, Path, description = "角色名称"),
        ("domain" = String, Path, description = "域名称"),
    ),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_policies_by_role(State(state): State<AppState>, Path((role, domain)): Path<(String, String)>) -> Result<impl IntoResponse, AppError> {
    let policies = CasbinService::get_policy_by_role(&state.db, &role, &domain).await?;
    Ok(R::ok(policies))
}

#[utoipa::path(
    get,
    path = "/api/casbin/user/{user}",
    params(("user" = String, Path, description = "用户名")),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_roles_for_user(State(state): State<AppState>, Path(user): Path<String>) -> Result<impl IntoResponse, AppError> {
    let roles = CasbinService::get_roles_for_user(&state.db, &user).await?;
    Ok(R::ok(roles))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/casbin/list", get(list))
        .route("/api/casbin/{id}", get(get_by_id))
        .route("/api/casbin", post(create))
        .route("/api/casbin/{id}", put(update))
        .route("/api/casbin/{id}", delete_method(delete))
        .route("/api/casbin/batch", delete_method(delete_batch))
        .route("/api/casbin/role/{role}/domain/{domain}", get(get_policies_by_role))
        .route("/api/casbin/role/{role}/domain/{domain}", put(update_role_policies))
        .route("/api/casbin/user/{user}", get(get_roles_for_user))
}
