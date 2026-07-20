//! Casbin Enforcer 策略重载工具
//!
//! 业务层修改策略后调用 `reload_policy_with` 刷新 CachedEnforcer 内存缓存。

use std::sync::Arc;

use casbin::CoreApi;
use tokio::sync::RwLock;

use casbin::CachedEnforcer;

/// 重新加载 Casbin 策略并刷新缓存
///
/// 在 CasbinService 的 create / update / delete 操作后调用，
/// 确保内存中的 CachedEnforcer 与数据库同步。
pub async fn reload_policy_with(enforcer: &Arc<RwLock<CachedEnforcer>>) {
    if let Err(err) = enforcer.write().await.load_policy().await {
        tracing::error!("Casbin 策略重载失败: {}", err);
    }
}
