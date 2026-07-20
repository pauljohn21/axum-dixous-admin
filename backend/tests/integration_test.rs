//! 集成测试 — 真实 DB + Redis
//!
//! 需要运行 MySQL (localhost:3306) 和 Redis (localhost:6379)。

mod common;

use serial_test::serial;
use service::sys_user_service::SysUserService;
use utils::prelude::ServiceError;
use model::dto::page_dto::PageRequest;
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};

#[tokio::test]
#[serial]
async fn test_user_login_success() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    let result = SysUserService::login(
        &state.db,
        LoginDTO {
            username: "testuser".into(),
            password: "123456".into(),
        },
    ).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.username, Some("testuser".into()));
}

#[tokio::test]
#[serial]
async fn test_user_login_wrong_password() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    let result = SysUserService::login(
        &state.db,
        LoginDTO {
            username: "testuser".into(),
            password: "wrongpassword".into(),
        },
    ).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ServiceError::InvalidPassword));
}

#[tokio::test]
#[serial]
async fn test_user_login_not_found() {
    let state = common::setup_test_state().await;

    let result = SysUserService::login(
        &state.db,
        LoginDTO {
            username: "nonexistent".into(),
            password: "123456".into(),
        },
    ).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ServiceError::UserNotFound));
}

#[tokio::test]
#[serial]
async fn test_user_crud_lifecycle() {
    let state = common::setup_test_state().await;

    // insert
    SysUserService::insert(
        &state.db,
        SysUserInsertDTO {
            username: "lifecycle_user".into(),
            password: "pass123".into(),
            nick_name: Some("生命周期测试".into()),
            role_id: None,
            phone: None,
            email: None,
        },
    ).await.unwrap();

    // user_info (通过用户名查找)
    let user = SysUserService::user_info(&state.db, "lifecycle_user".into()).await.unwrap();
    assert_eq!(user.nick_name, Some("生命周期测试".into()));
    let user_id = user.id;

    // update
    let updated = SysUserService::update(
        &state.db,
        user_id,
        SysUserUpdateDTO {
            nick_name: Some("已更新".into()),
            phone: Some("13800000000".into()),
            email: Some("test@test.com".into()),
            header_img: None,
            side_mode: None,
            enable: None,
        },
    ).await.unwrap();
    assert_eq!(updated.nick_name, Some("已更新".into()));
    assert_eq!(updated.phone, Some("13800000000".into()));

    // delete
    SysUserService::delete(&state.db, user_id).await.unwrap();

    // 确认已删除
    let result = SysUserService::get_by_id(&state.db, user_id).await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_user_list_with_keyword() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    // 再插入一个用户
    SysUserService::insert(
        &state.db,
        SysUserInsertDTO {
            username: "admin2".into(),
            password: "pass123".into(),
            nick_name: Some("管理员2".into()),
            role_id: None,
            phone: None,
            email: None,
        },
    ).await.unwrap();

    // 搜索 "testuser"（种子数据中有 admin 和 test 两个用户）
    let result = SysUserService::list(
        &state.db,
        PageRequest {
            page: Some(1),
            page_size: Some(10),
            keyword: Some("testuser".into()),
        },
    ).await.unwrap();

    assert_eq!(result.total, 1);
    assert_eq!(result.list[0].username, Some("testuser".into()));

    // 搜索全部（2 种子 + 2 插入 = 4）
    let result = SysUserService::list(
        &state.db,
        PageRequest {
            page: Some(1),
            page_size: Some(10),
            keyword: None,
        },
    ).await.unwrap();

    assert_eq!(result.total, 4);
}

#[tokio::test]
#[serial]
async fn test_user_change_password() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    // 修改密码
    SysUserService::change_password(
        &state.db,
        "testuser",
        "123456".into(),
        "newpass789".into(),
    ).await.unwrap();

    // 旧密码登录失败
    let result = SysUserService::login(
        &state.db,
        LoginDTO {
            username: "testuser".into(),
            password: "123456".into(),
        },
    ).await;
    assert!(result.is_err());

    // 新密码登录成功
    let result = SysUserService::login(
        &state.db,
        LoginDTO {
            username: "testuser".into(),
            password: "newpass789".into(),
        },
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_dashboard_stats() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;
    common::insert_test_role(&state.db).await;

    // 种子数据：2 用户 + 3 角色，加上我们插入的 1 用户 + 1 角色
    let stats = SysUserService::dashboard_stats(&state.db).await.unwrap();
    assert_eq!(stats.user_count, 3);
    assert_eq!(stats.role_count, 4);
}
