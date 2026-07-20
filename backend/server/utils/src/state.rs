//! 应用全局共享状态

use std::sync::Arc;

use casbin::CachedEnforcer;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

use crate::config::Config;

/// 应用全局共享状态，通过 Axum State 注入到所有 handler
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接池
    pub db: DatabaseConnection,
    /// Casbin 权限执行器
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    /// HTTP 客户端单例（复用连接池）
    pub http_client: reqwest::Client,
    /// 应用配置
    pub config: Config,
    // P2 阶段追加:
    // pub redis: redis::aio::ConnectionManager,
}
