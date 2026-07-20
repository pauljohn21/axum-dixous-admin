//! 测试辅助模块
//!
//! 提供测试用 AppState 构建、数据库初始化、fixtures 等。

use std::sync::Arc;

use casbin::{CachedEnforcer, CoreApi};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::MigratorTrait;
use tokio::sync::RwLock;

use utils::prelude::{AppState, CONFIG};

/// 创建测试数据库连接（scm_test）并执行迁移
pub async fn setup_test_db() -> DatabaseConnection {
    let db_url = format!(
        "mysql://{}:{}@{}:{}/scm_test",
        CONFIG.datasource.username,
        CONFIG.datasource.password,
        CONFIG.datasource.host,
        CONFIG.datasource.port,
    );
    let db = Database::connect(&db_url).await.expect("连接测试 DB 失败");
    migration::Migrator::fresh(&db).await.expect("迁移失败");
    db
}

/// 创建测试 Redis 连接（使用 DB index 1 避免与开发环境冲突）
pub async fn setup_test_redis() -> redis::aio::ConnectionManager {
    let url = format!("redis://{}:{}/1", CONFIG.cache.host, CONFIG.cache.port);
    let client = redis::Client::open(url).expect("创建 Redis 客户端失败");
    let mut conn = client
        .get_connection_manager()
        .await
        .expect("连接 Redis 失败");
    let _: () = redis::cmd("FLUSHDB")
        .query_async(&mut conn)
        .await
        .expect("FLUSHDB 失败");
    conn
}

/// 创建测试 Casbin Enforcer
pub async fn setup_test_enforcer(db: &DatabaseConnection) -> Arc<RwLock<CachedEnforcer>> {
    let model_str = include_str!("../../server/casbin/rbac_model.conf");
    let model = casbin::DefaultModel::from_str(model_str)
        .await
        .expect("加载 Casbin 模型失败");
    let adapter = casbin_adapter::SeaOrmAdapter::new(db.clone())
        .await
        .expect("创建 Casbin 适配器失败");
    Arc::new(RwLock::new(
        CachedEnforcer::new(model, adapter)
            .await
            .expect("创建 Enforcer 失败"),
    ))
}

/// 构建完整测试 AppState
pub async fn setup_test_state() -> AppState {
    let db = setup_test_db().await;
    let enforcer = setup_test_enforcer(&db).await;
    let redis = setup_test_redis().await;
    let http_client = reqwest::Client::new();
    AppState {
        db,
        enforcer,
        http_client,
        config: CONFIG.clone(),
        redis,
    }
}

/// 插入测试用户并返回 (id, username)
#[allow(dead_code)]
pub async fn insert_test_user(db: &DatabaseConnection) -> (i32, String) {
    use model::dao::sys_user;
    use sea_orm::Set;
    use utils::prelude::PasswordUtils;

    let hash = PasswordUtils::encrypt("123456");
    let active = sys_user::ActiveModel {
        username: Set(Some("testuser".into())),
        password: Set(Some(hash.password_hash)),
        salt: Set(Some(hash.salt)),
        nick_name: Set(Some("测试用户".into())),
        enable: Set(Some(1)),
        ..Default::default()
    };
    let result = sys_user::Entity::insert(active)
        .exec(db)
        .await
        .expect("插入测试用户失败");
    (result.last_insert_id as i32, "testuser".to_string())
}

/// 插入测试角色并返回 id
#[allow(dead_code)]
pub async fn insert_test_role(db: &DatabaseConnection) -> i32 {
    use model::dao::sys_role;
    use sea_orm::Set;

    let active = sys_role::ActiveModel {
        en_name: Set(Some("test_role".into())),
        cn_name: Set(Some("测试角色".into())),
        parent_id: Set(Some(0)),
        ..Default::default()
    };
    let result = sys_role::Entity::insert(active)
        .exec(db)
        .await
        .expect("插入测试角色失败");
    result.last_insert_id as i32
}
