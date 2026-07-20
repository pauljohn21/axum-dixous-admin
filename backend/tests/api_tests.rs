//! API 层集成测试
//!
//! 使用 Mock Service 验证 ServiceError → AppError 转换和配置加载。

use async_trait::async_trait;
use utils::prelude::{AppError, ServiceError, DashboardStats, UserService, RoleService, MenuService, ApiService};
use model::dao::{sys_user, sys_role, sys_menu, sys_apis};
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use model::dto::sys_api_dto::{SysApiInsertDTO, SysApiUpdateDTO};

/// 构建测试用 sys_user::Model
fn test_user() -> sys_user::Model {
    sys_user::Model {
        id: 1,
        username: Some("admin".into()),
        password: Some("hash".into()),
        salt: Some("salt".into()),
        nick_name: Some("管理员".into()),
        side_mode: None,
        header_img: None,
        base_color: None,
        active_color: None,
        phone: None,
        email: None,
        enable: Some(1),
        wx_openid: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        created_ad: None,
        updated_ad: None,
        deleted_ad: None,
    }
}

// ===== Mock UserService =====

struct MockUserService {
    pub should_fail: bool,
}

#[async_trait]
impl UserService for MockUserService {
    async fn login(&self, _data: LoginDTO) -> Result<sys_user::Model, ServiceError> {
        if self.should_fail {
            Err(ServiceError::Auth("mock error".into()))
        } else {
            Ok(test_user())
        }
    }
    async fn register(&self, _data: SysUserInsertDTO) -> Result<(), ServiceError> { Ok(()) }
    async fn user_info(&self, username: String) -> Result<sys_user::Model, ServiceError> {
        if username == "admin" {
            Ok(test_user())
        } else {
            Err(ServiceError::UserNotFound)
        }
    }
    async fn list(&self, _query: PageRequest) -> Result<PageResponse<sys_user::Model>, ServiceError> {
        Ok(PageResponse { list: vec![], total: 0, page: 1, page_size: 10 })
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_user::Model, ServiceError> {
        if id == 1 {
            Ok(test_user())
        } else {
            Err(ServiceError::UserNotFound)
        }
    }
    async fn update(&self, _id: i32, _data: SysUserUpdateDTO) -> Result<sys_user::Model, ServiceError> {
        Ok(test_user())
    }
    async fn delete(&self, _id: i32) -> Result<(), ServiceError> { Ok(()) }
    async fn change_password(&self, _username: &str, _old: String, _new: String) -> Result<(), ServiceError> { Ok(()) }
    async fn wx_login(&self, _code: &str) -> Result<sys_user::Model, ServiceError> {
        Err(ServiceError::WechatApi("not configured".into()))
    }
    async fn wx_bind(&self, _username: &str, _code: &str) -> Result<(), ServiceError> { Ok(()) }
    async fn dashboard_stats(&self) -> Result<DashboardStats, ServiceError> {
        Ok(DashboardStats { user_count: 1, role_count: 1, menu_count: 1, api_count: 1 })
    }
}

// ===== 测试用例 =====

#[tokio::test]
async fn test_mock_user_service_login_success() {
    let service = MockUserService { should_fail: false };
    let result = service.login(LoginDTO {
        username: "admin".into(),
        password: "123456".into(),
    }).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, Some("admin".into()));
}

#[tokio::test]
async fn test_mock_user_service_login_failure() {
    let service = MockUserService { should_fail: true };
    let result = service.login(LoginDTO {
        username: "admin".into(),
        password: "wrong".into(),
    }).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ServiceError::Auth(msg) => assert_eq!(msg, "mock error"),
        _ => panic!("expected Auth error"),
    }
}

#[tokio::test]
async fn test_mock_user_service_user_info_not_found() {
    let service = MockUserService { should_fail: false };
    let result = service.user_info("nonexistent".into()).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ServiceError::UserNotFound));
}

#[tokio::test]
async fn test_mock_user_service_dashboard_stats() {
    let service = MockUserService { should_fail: false };
    let stats = service.dashboard_stats().await.unwrap();
    assert_eq!(stats.user_count, 1);
    assert_eq!(stats.role_count, 1);
    assert_eq!(stats.menu_count, 1);
    assert_eq!(stats.api_count, 1);
}

#[tokio::test]
async fn test_mock_user_service_list() {
    let service = MockUserService { should_fail: false };
    let result = service.list(PageRequest {
        page: Some(1),
        page_size: Some(10),
        keyword: None,
    }).await.unwrap();
    assert_eq!(result.total, 0);
    assert_eq!(result.list.len(), 0);
}

#[tokio::test]
async fn test_service_error_to_app_error_conversion() {
    // UserNotFound → NotFoundError
    let app_error: AppError = ServiceError::UserNotFound.into();
    assert!(matches!(app_error, AppError::NotFoundError(_)));

    // InvalidPassword → AuthError
    let app_error: AppError = ServiceError::InvalidPassword.into();
    assert!(matches!(app_error, AppError::AuthError(_)));

    // WechatAlreadyBound → AuthError
    let app_error: AppError = ServiceError::WechatAlreadyBound.into();
    assert!(matches!(app_error, AppError::AuthError(_)));

    // NotFound → NotFoundError
    let app_error: AppError = ServiceError::NotFound("test".into()).into();
    assert!(matches!(app_error, AppError::NotFoundError(_)));

    // Forbidden → Forbidden
    let app_error: AppError = ServiceError::Forbidden("test".into()).into();
    assert!(matches!(app_error, AppError::Forbidden(_)));

    // Db → Internal (DbErr falls through to _ wildcard)
    let db_err = sea_orm::DbErr::RecordNotFound("test".into());
    let service_error: ServiceError = db_err.into();
    let app_error: AppError = service_error.into();
    assert!(matches!(app_error, AppError::Internal(_)));
}

#[tokio::test]
async fn test_config_values() {
    use utils::prelude::CONFIG;
    // 默认配置来自 config.yml
    assert_eq!(CONFIG.server.port, 8888);
    assert_eq!(CONFIG.datasource.database, "scm");
    assert_eq!(CONFIG.jwt.expire_hours, 24);
}

// ===== Mock RoleService =====

fn test_role() -> sys_role::Model {
    sys_role::Model {
        id: 1,
        en_name: Some("admin".into()),
        cn_name: Some("管理员".into()),
        parent_id: Some(0),
        created_ad: None,
        updated_ad: None,
        deleted_ad: None,
    }
}

struct MockRoleService;

#[async_trait]
impl RoleService for MockRoleService {
    async fn insert(&self, _data: SysRoleInsertDTO) -> Result<sys_role::Model, ServiceError> {
        Ok(test_role())
    }
    async fn list(&self, _query: PageRequest) -> Result<PageResponse<sys_role::Model>, ServiceError> {
        Ok(PageResponse { list: vec![test_role()], total: 1, page: 1, page_size: 10 })
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_role::Model, ServiceError> {
        if id == 1 {
            Ok(test_role())
        } else {
            Err(ServiceError::NotFound("角色不存在".into()))
        }
    }
    async fn update(&self, _id: i32, _data: SysRoleUpdateDTO) -> Result<sys_role::Model, ServiceError> {
        Ok(test_role())
    }
    async fn delete(&self, _id: i32) -> Result<(), ServiceError> { Ok(()) }
}

#[tokio::test]
async fn test_mock_role_service_crud() {
    let svc = MockRoleService;

    // insert
    let role = svc.insert(SysRoleInsertDTO {
        en_name: "admin".into(),
        cn_name: "管理员".into(),
        parent_id: Some(0),
    }).await.unwrap();
    assert_eq!(role.en_name, Some("admin".into()));

    // get_by_id
    let found = svc.get_by_id(1).await.unwrap();
    assert_eq!(found.cn_name, Some("管理员".into()));

    // get_by_id not found
    assert!(svc.get_by_id(999).await.is_err());

    // list
    let list = svc.list(PageRequest { page: Some(1), page_size: Some(10), keyword: None }).await.unwrap();
    assert_eq!(list.total, 1);

    // delete
    assert!(svc.delete(1).await.is_ok());
}

// ===== Mock MenuService =====

fn test_menu() -> sys_menu::Model {
    sys_menu::Model {
        id: 1,
        menu_level: Some(0),
        parent_id: Some(0),
        path: Some("/dashboard".into()),
        name: Some("dashboard".into()),
        hidden: Some(0),
        component: Some("Dashboard".into()),
        sort: Some(0),
        active_name: None,
        keep_alive: Some(0),
        default_menu: Some(0),
        title: Some("仪表盘".into()),
        icon: Some("dashboard".into()),
        close_tab: Some(0),
        created_ad: None,
        updated_ad: None,
        deleted_ad: None,
    }
}

struct MockMenuService;

#[async_trait]
impl MenuService for MockMenuService {
    async fn insert(&self, _data: SysMenuInsertDTO) -> Result<sys_menu::Model, ServiceError> {
        Ok(test_menu())
    }
    async fn list(&self, _query: PageRequest) -> Result<PageResponse<sys_menu::Model>, ServiceError> {
        Ok(PageResponse { list: vec![test_menu()], total: 1, page: 1, page_size: 10 })
    }
    async fn get_by_id(&self, id: i32) -> Result<sys_menu::Model, ServiceError> {
        if id == 1 { Ok(test_menu()) } else { Err(ServiceError::NotFound("菜单不存在".into())) }
    }
    async fn update(&self, _id: i32, _data: SysMenuUpdateDTO) -> Result<sys_menu::Model, ServiceError> {
        Ok(test_menu())
    }
    async fn delete(&self, _id: i32) -> Result<(), ServiceError> { Ok(()) }
    async fn get_menus_by_username(&self, _username: &str) -> Result<Vec<sys_menu::Model>, ServiceError> {
        Ok(vec![test_menu()])
    }
}

#[tokio::test]
async fn test_mock_menu_service_crud() {
    let svc = MockMenuService;

    let menu = svc.insert(SysMenuInsertDTO {
        menu_level: Some(0),
        parent_id: Some(0),
        path: Some("/dashboard".into()),
        name: Some("dashboard".into()),
        hidden: Some(0),
        component: Some("Dashboard".into()),
        sort: Some(0),
        active_name: None,
        keep_alive: Some(0),
        default_menu: Some(0),
        title: Some("仪表盘".into()),
        icon: Some("dashboard".into()),
        close_tab: Some(0),
    }).await.unwrap();
    assert_eq!(menu.path, Some("/dashboard".into()));

    assert!(svc.get_by_id(1).await.is_ok());
    assert!(svc.get_by_id(999).await.is_err());

    let menus = svc.get_menus_by_username("admin").await.unwrap();
    assert_eq!(menus.len(), 1);

    let list = svc.list(PageRequest { page: Some(1), page_size: Some(10), keyword: None }).await.unwrap();
    assert_eq!(list.total, 1);
}

// ===== Mock ApiService =====

fn test_api() -> sys_apis::Model {
    sys_apis::Model {
        id: 1,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        path: Some("/api/user".into()),
        description: Some("用户列表".into()),
        api_group: Some("用户管理".into()),
        method: Some("GET".into()),
    }
}

struct MockApiService;

#[async_trait]
impl ApiService for MockApiService {
    async fn insert(&self, _data: SysApiInsertDTO) -> Result<sys_apis::Model, ServiceError> {
        Ok(test_api())
    }
    async fn list(&self, _query: PageRequest) -> Result<PageResponse<sys_apis::Model>, ServiceError> {
        Ok(PageResponse { list: vec![test_api()], total: 1, page: 1, page_size: 10 })
    }
    async fn get_by_id(&self, id: i64) -> Result<sys_apis::Model, ServiceError> {
        if id == 1 { Ok(test_api()) } else { Err(ServiceError::NotFound("API不存在".into())) }
    }
    async fn update(&self, _id: i64, _data: SysApiUpdateDTO) -> Result<sys_apis::Model, ServiceError> {
        Ok(test_api())
    }
    async fn delete(&self, _id: i64) -> Result<(), ServiceError> { Ok(()) }
}

#[tokio::test]
async fn test_mock_api_service_crud() {
    let svc = MockApiService;

    let api = svc.insert(SysApiInsertDTO {
        path: Some("/api/user".into()),
        description: Some("用户列表".into()),
        api_group: Some("用户管理".into()),
        method: Some("GET".into()),
    }).await.unwrap();
    assert_eq!(api.path, Some("/api/user".into()));

    assert!(svc.get_by_id(1).await.is_ok());
    assert!(svc.get_by_id(999).await.is_err());

    let list = svc.list(PageRequest { page: Some(1), page_size: Some(10), keyword: None }).await.unwrap();
    assert_eq!(list.total, 1);

    assert!(svc.delete(1).await.is_ok());
}
