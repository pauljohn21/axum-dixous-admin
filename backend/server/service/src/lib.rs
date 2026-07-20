pub mod casbin_service;
pub mod enforcer;
pub mod impls;
pub mod jwt_blacklist_service;
pub mod sys_api_service;
pub mod sys_base_menu_btn_service;
pub mod sys_base_menu_param_service;
pub mod sys_data_role_service;
pub mod sys_dictionary_detail_service;
pub mod sys_dictionary_service;
pub mod sys_menu_domain_service;
pub mod sys_menu_role_service;
pub mod sys_menu_service;
pub mod sys_operation_record_service;
pub mod sys_role_btn_service;
pub mod sys_role_menu_service;
pub mod sys_role_service;
pub mod sys_user_role_service;
pub mod sys_user_service;
pub mod generator_history_service;
pub mod generator_code_service;

/// 仪表盘统计数据
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct DashboardStats {
    pub user_count: u64,
    pub role_count: u64,
    pub menu_count: u64,
    pub api_count: u64,
}
