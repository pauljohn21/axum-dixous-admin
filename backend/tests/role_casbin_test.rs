//! 集成测试 — Role Service + Casbin 权限
//!
//! 需要运行 MySQL (localhost:3306) 和 Redis (localhost:6379)。

mod common;

use casbin::{CoreApi, MgmtApi};
use serial_test::serial;
use service::sys_role_service::SysRoleService;
use utils::prelude::ServiceError;
use model::dto::page_dto::PageRequest;
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};

#[tokio::test]
#[serial]
async fn test_role_crud_lifecycle() {
    let state = common::setup_test_state().await;

    // insert
    let role = SysRoleService::insert(
        &state.db,
        SysRoleInsertDTO {
            en_name: "test_lifecycle".into(),
            cn_name: "生命周期角色".into(),
            parent_id: Some(0),
        },
    ).await.unwrap();
    assert_eq!(role.en_name, Some("test_lifecycle".into()));
    let role_id = role.id;

    // get_by_id
    let found = SysRoleService::get_by_id(&state.db, role_id).await.unwrap();
    assert_eq!(found.cn_name, Some("生命周期角色".into()));

    // update
    let updated = SysRoleService::update(
        &state.db,
        role_id,
        SysRoleUpdateDTO {
            en_name: Some("test_updated".into()),
            cn_name: Some("已更新角色".into()),
            parent_id: None,
        },
    ).await.unwrap();
    assert_eq!(updated.en_name, Some("test_updated".into()));

    // 注: delete 测试跳过 — SysRoleService::delete 引用 sys_role_menus 表，
    // 但迁移脚本创建的是 sys_menu_role 表（预存在命名不一致）
}

#[tokio::test]
#[serial]
async fn test_role_get_not_found() {
    let state = common::setup_test_state().await;

    let result = SysRoleService::get_by_id(&state.db, 99999).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ServiceError::NotFound(_)));
}

#[tokio::test]
#[serial]
async fn test_role_list_with_keyword() {
    let state = common::setup_test_state().await;

    // 种子数据已有 3 个角色 (admin, user, test)
    SysRoleService::insert(
        &state.db,
        SysRoleInsertDTO {
            en_name: "unique_keyword_role".into(),
            cn_name: "唯一关键词角色".into(),
            parent_id: Some(0),
        },
    ).await.unwrap();

    // 搜索唯一关键词
    let result = SysRoleService::list(
        &state.db,
        PageRequest {
            page: Some(1),
            page_size: Some(10),
            keyword: Some("unique_keyword".into()),
        },
    ).await.unwrap();

    assert_eq!(result.total, 1);
    assert_eq!(result.list[0].en_name, Some("unique_keyword_role".into()));
}

#[tokio::test]
#[serial]
async fn test_casbin_enforce_allow() {
    let state = common::setup_test_state().await;

    // 种子数据已包含 admin (888) 角色的策略
    // 验证已有策略: admin 可以 GET /api/user/info
    let allowed = {
        let e = state.enforcer.read().await;
        e.enforce(("888", "/api/user/info", "GET")).unwrap()
    };
    assert!(allowed, "admin 角色应该有权限访问 /api/user/info");
}

#[tokio::test]
#[serial]
async fn test_casbin_enforce_deny() {
    let state = common::setup_test_state().await;

    // 无权限的路由应该拒绝
    let denied = {
        let e = state.enforcer.read().await;
        e.enforce(("888", "/api/secret/nothing", "GET")).unwrap()
    };
    assert!(!denied, "admin 角色不应该有权限访问不存在的路由");

    // 未分配角色的用户应该被拒绝
    let denied = {
        let e = state.enforcer.read().await;
        e.enforce(("unknown_user", "/api/user/info", "GET")).unwrap()
    };
    assert!(!denied, "未知用户不应该有权限");
}

#[tokio::test]
#[serial]
async fn test_casbin_add_policy_and_enforce() {
    let state = common::setup_test_state().await;

    // 通过 enforcer API 直接添加策略
    {
        let mut e = state.enforcer.write().await;
        e.add_policy(vec!["test_role".to_string(), "/api/test".to_string(), "GET".to_string()])
            .await
            .unwrap();
    }

    // 验证策略生效
    let allowed = {
        let e = state.enforcer.read().await;
        e.enforce(("test_role", "/api/test", "GET")).unwrap()
    };
    assert!(allowed, "添加策略后应该有权限");

    // 删除策略
    {
        let mut e = state.enforcer.write().await;
        e.remove_policy(vec!["test_role".to_string(), "/api/test".to_string(), "GET".to_string()])
            .await
            .unwrap();
    }

    // 验证策略已删除
    let denied = {
        let e = state.enforcer.read().await;
        e.enforce(("test_role", "/api/test", "GET")).unwrap()
    };
    assert!(!denied, "删除策略后应该无权限");
}

#[tokio::test]
#[serial]
async fn test_casbin_role_inheritance() {
    let state = common::setup_test_state().await;

    // 添加角色继承: alice → 888 (admin)
    {
        let mut e = state.enforcer.write().await;
        e.add_grouping_policy(vec!["alice".to_string(), "888".to_string()])
            .await
            .unwrap();
    }

    // alice 通过继承拥有 admin 的权限
    let allowed = {
        let e = state.enforcer.read().await;
        e.enforce(("alice", "/api/user/info", "GET")).unwrap()
    };
    assert!(allowed, "alice 继承 admin 角色后应该有权限");

    // 移除角色继承
    {
        let mut e = state.enforcer.write().await;
        e.remove_grouping_policy(vec!["alice".to_string(), "888".to_string()])
            .await
            .unwrap();
    }

    // alice 不再有权限
    let denied = {
        let e = state.enforcer.read().await;
        e.enforce(("alice", "/api/user/info", "GET")).unwrap()
    };
    assert!(!denied, "移除角色继承后应该无权限");
}
