//! Service 层 Trait 定义
//!
//! 将 Service 层抽象为 Trait，支持依赖注入和测试 Mock。
//! Trait 定义放在 `utils` 中以避免 `service` → `utils` 的循环依赖。
//! Service 实现持有 `DatabaseConnection`，Trait 方法不再需要传入 db 参数。

use async_trait::async_trait;

use model::dao::{self, sys_user, sys_role, sys_menu, sys_apis};
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use model::dto::sys_api_dto::{SysApiInsertDTO, SysApiUpdateDTO};
use crate::error::ServiceError;

/// 仪表盘统计数据
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct DashboardStats {
    pub user_count: u64,
    pub role_count: u64,
    pub menu_count: u64,
    pub api_count: u64,
}

// ===== 用户服务 =====

#[async_trait]
pub trait UserService: Send + Sync {
    async fn login(&self, data: LoginDTO) -> Result<sys_user::Model, ServiceError>;
    async fn register(&self, data: SysUserInsertDTO) -> Result<(), ServiceError>;
    async fn user_info(&self, username: String) -> Result<sys_user::Model, ServiceError>;
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_user::Model>, ServiceError>;
    async fn get_by_id(&self, id: i32) -> Result<sys_user::Model, ServiceError>;
    async fn update(&self, id: i32, data: SysUserUpdateDTO) -> Result<sys_user::Model, ServiceError>;
    async fn delete(&self, id: i32) -> Result<(), ServiceError>;
    async fn change_password(&self, username: &str, old: String, new: String) -> Result<(), ServiceError>;
    async fn wx_login(&self, code: &str) -> Result<sys_user::Model, ServiceError>;
    async fn wx_bind(&self, username: &str, code: &str) -> Result<(), ServiceError>;
    async fn dashboard_stats(&self) -> Result<DashboardStats, ServiceError>;
}

// ===== 角色服务 =====

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn insert(&self, data: SysRoleInsertDTO) -> Result<sys_role::Model, ServiceError>;
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_role::Model>, ServiceError>;
    async fn get_by_id(&self, id: i32) -> Result<sys_role::Model, ServiceError>;
    async fn update(&self, id: i32, data: SysRoleUpdateDTO) -> Result<sys_role::Model, ServiceError>;
    async fn delete(&self, id: i32) -> Result<(), ServiceError>;
}

// ===== 菜单服务 =====

#[async_trait]
pub trait MenuService: Send + Sync {
    async fn insert(&self, data: SysMenuInsertDTO) -> Result<sys_menu::Model, ServiceError>;
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_menu::Model>, ServiceError>;
    async fn get_by_id(&self, id: i32) -> Result<sys_menu::Model, ServiceError>;
    async fn update(&self, id: i32, data: SysMenuUpdateDTO) -> Result<sys_menu::Model, ServiceError>;
    async fn delete(&self, id: i32) -> Result<(), ServiceError>;
    async fn get_menus_by_username(&self, username: &str) -> Result<Vec<sys_menu::Model>, ServiceError>;
}

// ===== API 服务 =====

#[async_trait]
pub trait ApiService: Send + Sync {
    async fn insert(&self, data: SysApiInsertDTO) -> Result<sys_apis::Model, ServiceError>;
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_apis::Model>, ServiceError>;
    async fn get_by_id(&self, id: i64) -> Result<sys_apis::Model, ServiceError>;
    async fn update(&self, id: i64, data: SysApiUpdateDTO) -> Result<sys_apis::Model, ServiceError>;
    async fn delete(&self, id: i64) -> Result<(), ServiceError>;
}
