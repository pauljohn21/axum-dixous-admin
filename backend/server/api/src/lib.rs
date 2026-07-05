pub mod api_api;
pub mod casbin_api;
pub mod data_role_api;
pub mod dictionary_api;
pub mod dictionary_detail_api;
pub mod jwt_api;
pub mod menu_api;
pub mod menu_btn_api;
pub mod menu_param_api;
pub mod operation_record_api;
pub mod role_api;
pub mod role_btn_api;
pub mod role_menu_api;
pub mod user_api;

use axum::Router;
use axum::routing::post;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::Modify;
use utoipa::OpenApi;

use model::dao::*;
use model::dto::*;
use utils::prelude::R;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SCM Admin API",
        version = "1.0.0",
        description = "后台管理系统 API"
    ),
    paths(
        user_api::login, user_api::register, user_api::list, user_api::get_by_id, user_api::update, user_api::delete_user,
        role_api::create, role_api::list, role_api::get_by_id, role_api::update, role_api::delete_role,
        menu_api::create, menu_api::list, menu_api::get_by_id, menu_api::update, menu_api::delete_menu,
        api_api::create, api_api::list, api_api::get_by_id, api_api::update, api_api::delete_api,
        jwt_api::create, jwt_api::list, jwt_api::get_by_id, jwt_api::update, jwt_api::delete_jwt,
        menu_btn_api::create, menu_btn_api::list, menu_btn_api::get_by_id, menu_btn_api::update, menu_btn_api::delete_btn,
        menu_param_api::create, menu_param_api::list, menu_param_api::get_by_id, menu_param_api::update, menu_param_api::delete_param,
        role_btn_api::create, role_btn_api::list, role_btn_api::get_by_composite_id, role_btn_api::delete_role_btn,
        role_menu_api::create, role_menu_api::list, role_menu_api::get_by_composite_id, role_menu_api::delete_role_menu,
        data_role_api::create, data_role_api::list, data_role_api::get_by_composite_id, data_role_api::delete_data_role,
        dictionary_api::create, dictionary_api::list, dictionary_api::get_by_id, dictionary_api::update, dictionary_api::delete_dict,
        dictionary_detail_api::create, dictionary_detail_api::list, dictionary_detail_api::get_by_id, dictionary_detail_api::update, dictionary_detail_api::delete_detail,
        operation_record_api::create, operation_record_api::list, operation_record_api::get_by_id, operation_record_api::update, operation_record_api::delete_record
    ),
    components(schemas(
        R<serde_json::Value>,
        page_dto::PageRequest,
        page_dto::PageResponse<serde_json::Value>,
        sys_user_dto::LoginDTO, sys_user_dto::SysUserInsertDTO, sys_user_dto::SysUserUpdateDTO, sys_user_dto::SysUserQueryDTO,
        sys_role_dto::SysRoleInsertDTO, sys_role_dto::SysRoleUpdateDTO, sys_role_dto::SysRoleQueryDTO,
        sys_menu_dto::SysMenuInsertDTO, sys_menu_dto::SysMenuUpdateDTO, sys_menu_dto::SysMenuQueryDTO,
        sys_api_dto::SysApiInsertDTO, sys_api_dto::SysApiUpdateDTO, sys_api_dto::SysApiQueryDTO,
        jwt_blacklist_dto::JwtBlacklistInsertDTO, jwt_blacklist_dto::JwtBlacklistUpdateDTO, jwt_blacklist_dto::JwtBlacklistQueryDTO,
        casbin_dto::CasbinRuleQueryDTO,
        sys_base_menu_btn_dto::SysBaseMenuBtnInsertDTO, sys_base_menu_btn_dto::SysBaseMenuBtnUpdateDTO, sys_base_menu_btn_dto::SysBaseMenuBtnQueryDTO,
        sys_base_menu_param_dto::SysBaseMenuParamInsertDTO, sys_base_menu_param_dto::SysBaseMenuParamUpdateDTO, sys_base_menu_param_dto::SysBaseMenuParamQueryDTO,
        sys_role_btn_dto::SysRoleBtnInsertDTO, sys_role_btn_dto::SysRoleBtnQueryDTO,
        sys_role_menu_dto::SysRoleMenuInsertDTO, sys_role_menu_dto::SysRoleMenuQueryDTO,
        sys_data_role_dto::SysDataRoleInsertDTO, sys_data_role_dto::SysDataRoleQueryDTO,
        sys_dictionary_dto::SysDictionaryInsertDTO, sys_dictionary_dto::SysDictionaryUpdateDTO, sys_dictionary_dto::SysDictionaryQueryDTO,
        sys_dictionary_detail_dto::SysDictionaryDetailInsertDTO, sys_dictionary_detail_dto::SysDictionaryDetailUpdateDTO, sys_dictionary_detail_dto::SysDictionaryDetailQueryDTO,
        sys_operation_record_dto::SysOperationRecordInsertDTO, sys_operation_record_dto::SysOperationRecordUpdateDTO, sys_operation_record_dto::SysOperationRecordQueryDTO,
        sys_user::Model, sys_role::Model, sys_menu::Model, sys_apis::Model,
        jwt_blacklists::Model, casbin_rule::Model,
        sys_base_menu_btns::Model, sys_base_menu_parameters::Model,
        sys_role_btns::Model, sys_role_menus::Model, sys_data_role_id::Model,
        sys_dictionaries::Model, sys_dictionary_details::Model,
        sys_operation_records::Model, sys_menu_role::Model, sys_menu_domain::Model,
        user_api::LoginResp
    )),
    tags(
        (name = "用户管理", description = "用户 CRUD"),
        (name = "角色管理", description = "角色 CRUD"),
        (name = "菜单管理", description = "菜单 CRUD"),
        (name = "API管理", description = "API 接口管理"),
        (name = "Casbin策略", description = "权限策略管理"),
        (name = "JWT管理", description = "JWT 黑名单管理"),
        (name = "菜单按钮", description = "菜单按钮管理"),
        (name = "菜单参数", description = "菜单路由参数"),
        (name = "角色按钮", description = "角色按钮权限"),
        (name = "角色菜单", description = "角色菜单权限"),
        (name = "数据权限", description = "角色数据权限"),
        (name = "字典管理", description = "系统字典 CRUD"),
        (name = "字典详情", description = "字典项 CRUD"),
        (name = "操作记录", description = "操作日志查询")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

pub fn public_routes() -> Router {
    Router::new()
        .route("/api/user/login", post(user_api::login))
        .route("/health", axum::routing::get(health))
}

pub fn protected_routes() -> Router {
    Router::new()
        .merge(user_api::routes())
        .merge(role_api::routes())
        .merge(menu_api::routes())
        .merge(api_api::routes())
        .merge(jwt_api::routes())
        .merge(casbin_api::routes())
        .merge(role_btn_api::routes())
        .merge(role_menu_api::routes())
        .merge(menu_btn_api::routes())
        .merge(menu_param_api::routes())
        .merge(data_role_api::routes())
        .merge(dictionary_api::routes())
        .merge(dictionary_detail_api::routes())
        .merge(operation_record_api::routes())
}

pub fn swagger_routes() -> Router {
    utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
        .url("/openapi.json", ApiDoc::openapi())
        .into()
}

async fn health() -> impl axum::response::IntoResponse {
    axum::Json(serde_json::json!({ "status": "ok" }))
}
