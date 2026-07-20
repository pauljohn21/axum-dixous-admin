//! Casbin Enforcer 全局持有器
//!
//! 在 main.rs 启动时通过 `set_enforcer` 注入，业务层修改策略后调用 `reload_policy` 刷新缓存。

use std::sync::{Arc, OnceLock};

use casbin::{CachedEnforcer, CoreApi};
use tokio::sync::RwLock;

static ENFORCER: OnceLock<Arc<RwLock<CachedEnforcer>>> = OnceLock::new();

/// 注入 Enforcer（仅在 main.rs 启动时调用一次）
pub fn set_enforcer(e: Arc<RwLock<CachedEnforcer>>) {
    let _ = ENFORCER.set(e);
}

/// 重新加载 Casbin 策略（使用传入的 enforcer）
pub async fn reload_policy_with(enforcer: &Arc<RwLock<CachedEnforcer>>) {
    if let Err(err) = enforcer.write().await.load_policy().await {
        tracing::error!("Casbin 策略重载失败: {}", err);
    }
}

/// 重新加载 Casbin 策略并刷新缓存（使用全局 enforcer，过渡期保留）
///
/// 在 CasbinService 的 create / update / delete 操作后调用，
/// 确保内存中的 CachedEnforcer 与数据库同步。
pub async fn reload_policy() {
    if let Some(e) = ENFORCER.get() {
        if let Err(err) = e.write().await.load_policy().await {
            tracing::error!("Casbin 策略重载失败: {}", err);
        }
    }
}
