//! Service Trait 实现
//!
//! 为每个 Service 结构体实现对应的 Trait，支持通过 trait object 注入。

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;
use casbin::CachedEnforcer;

use model::dao::{sys_user, sys_role, sys_menu, sys_apis};
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use model::dto::sys_api_dto::{SysApiInsertDTO, SysApiUpdateDTO};
use utils::prelude::{ServiceError, DashboardStats, UserService, RoleService, MenuService, ApiService};

use crate::sys_user_service::SysUserService;
use crate::sys_role_service::SysRoleService;
use crate::sys_menu_service::SysMenuService;
use crate::sys_api_service::SysApiService;

// ===== 用户服务实现 =====

pub struct UserServiceImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn login(&self, data: LoginDTO) -> Result<sys_user::Model, ServiceError> {
        SysUserService::login(&self.db, data).await
    }
    async fn register(&self, data: SysUserInsertDTO) -> Result<(), ServiceError> {
        SysUserService::insert(&self.db, data).await
    }
    async fn user_info(&self, username: String) -> Result<sys_user::Model, ServiceError> {
        SysUserService::user_info(&self.db, username).await
    }
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_user::Model>, ServiceError> {
        SysUserService::list(&self.db, query).await
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_user::Model, ServiceError> {
        SysUserService::get_by_id(&self.db, id).await
    }
    async fn update(&self, id: i32, data: SysUserUpdateDTO) -> Result<sys_user::Model, ServiceError> {
        SysUserService::update(&self.db, id, data).await
    }
    async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        SysUserService::delete(&self.db, id).await
    }
    async fn change_password(&self, username: &str, old: String, new: String) -> Result<(), ServiceError> {
        SysUserService::change_password(&self.db, username, old, new).await
    }
    async fn wx_login(&self, code: &str) -> Result<sys_user::Model, ServiceError> {
        SysUserService::wx_login(&self.db, code).await
    }
    async fn wx_bind(&self, username: &str, code: &str) -> Result<(), ServiceError> {
        SysUserService::wx_bind(&self.db, username, code).await
    }
    async fn dashboard_stats(&self) -> Result<DashboardStats, ServiceError> {
        let stats = SysUserService::dashboard_stats(&self.db).await?;
        Ok(DashboardStats {
            user_count: stats.user_count,
            role_count: stats.role_count,
            menu_count: stats.menu_count,
            api_count: stats.api_count,
        })
    }
}

// ===== 角色服务实现 =====

pub struct RoleServiceImpl {
    pub db: DatabaseConnection,
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
}

#[async_trait]
impl RoleService for RoleServiceImpl {
    async fn insert(&self, data: SysRoleInsertDTO) -> Result<sys_role::Model, ServiceError> {
        SysRoleService::insert(&self.db, data).await
    }
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_role::Model>, ServiceError> {
        SysRoleService::list(&self.db, query).await
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_role::Model, ServiceError> {
        SysRoleService::get_by_id(&self.db, id).await
    }
    async fn update(&self, id: i32, data: SysRoleUpdateDTO) -> Result<sys_role::Model, ServiceError> {
        SysRoleService::update(&self.db, id, data).await
    }
    async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        SysRoleService::delete(&self.db, &self.enforcer, id).await
    }
}

// ===== 菜单服务实现 =====

pub struct MenuServiceImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl MenuService for MenuServiceImpl {
    async fn insert(&self, data: SysMenuInsertDTO) -> Result<sys_menu::Model, ServiceError> {
        SysMenuService::insert(&self.db, data).await
    }
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_menu::Model>, ServiceError> {
        SysMenuService::list(&self.db, query).await
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_menu::Model, ServiceError> {
        SysMenuService::get_by_id(&self.db, id).await
    }
    async fn update(&self, id: i32, data: SysMenuUpdateDTO) -> Result<sys_menu::Model, ServiceError> {
        SysMenuService::update(&self.db, id, data).await
    }
    async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        SysMenuService::delete(&self.db, id).await
    }
    async fn get_menus_by_username(&self, username: &str) -> Result<Vec<sys_menu::Model>, ServiceError> {
        SysMenuService::get_menus_by_username(&self.db, username).await
    }
}

// ===== API 服务实现 =====

pub struct ApiServiceImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ApiService for ApiServiceImpl {
    async fn insert(&self, data: SysApiInsertDTO) -> Result<sys_apis::Model, ServiceError> {
        SysApiService::insert(&self.db, data).await
    }
    async fn list(&self, query: PageRequest) -> Result<PageResponse<sys_apis::Model>, ServiceError> {
        SysApiService::list(&self.db, query).await
    }
    async fn get_by_id(&self, id: i64) -> Result<sys_apis::Model, ServiceError> {
        SysApiService::get_by_id(&self.db, id).await
    }
    async fn update(&self, id: i64, data: SysApiUpdateDTO) -> Result<sys_apis::Model, ServiceError> {
        SysApiService::update(&self.db, id, data).await
    }
    async fn delete(&self, id: i64) -> Result<(), ServiceError> {
        SysApiService::delete(&self.db, id).await
    }
}
