//! Redis 缓存工具模块
//!
//! 提供 Cache-Aside 模式的通用缓存操作。
//! 所有缓存值以 JSON 序列化存储。

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{de::DeserializeOwned, Serialize};
use tracing::warn;

/// 缓存常量 — Key 前缀和 TTL
pub mod keys {
    /// 用户菜单缓存前缀: `user_menus:{username}`
    pub const USER_MENUS_PREFIX: &str = "user_menus:";
    /// 用户菜单缓存 TTL (1 小时)
    pub const USER_MENUS_TTL: u64 = 3600;

    /// 字典列表缓存 Key
    pub const DICT_LIST: &str = "dict:list";
    /// 字典缓存 TTL (2 小时)
    pub const DICT_TTL: u64 = 7200;

    /// Dashboard 统计缓存 Key
    pub const DASHBOARD_STATS: &str = "dashboard:stats";
    /// Dashboard 缓存 TTL (5 分钟)
    pub const DASHBOARD_TTL: u64 = 300;
}

/// 通用缓存操作工具
pub struct Cache;

impl Cache {
    /// 获取缓存值（JSON 反序列化）
    ///
    /// 返回 `None` 表示缓存 miss 或反序列化失败
    pub async fn get<T: DeserializeOwned>(redis: &mut ConnectionManager, key: &str) -> Option<T> {
        let val: Option<String> = redis.get(key).await.ok()?;
        match val {
            Some(json) => serde_json::from_str(&json).ok(),
            None => None,
        }
    }

    /// 设置缓存值（JSON 序列化 + TTL）
    pub async fn set<T: Serialize>(redis: &mut ConnectionManager, key: &str, val: &T, ttl_secs: u64) {
        match serde_json::to_string(val) {
            Ok(json) => {
                if let Err(e) = redis.set_ex::<_, _, ()>(key, json, ttl_secs).await {
                    warn!("缓存写入失败 key={}: {}", key, e);
                }
            }
            Err(e) => warn!("缓存序列化失败 key={}: {}", key, e),
        }
    }

    /// 删除缓存
    pub async fn del(redis: &mut ConnectionManager, key: &str) {
        if let Err(e) = redis.del::<_, ()>(key).await {
            warn!("缓存删除失败 key={}: {}", key, e);
        }
    }

    /// 按模式删除缓存（如 `user_menus:*`）
    pub async fn del_pattern(redis: &mut ConnectionManager, pattern: &str) {
        match redis.keys::<_, Vec<String>>(pattern).await {
            Ok(keys) => {
                for key in keys {
                    if let Err(e) = redis.del::<_, ()>(&key).await {
                        warn!("缓存删除失败 key={}: {}", key, e);
                    }
                }
            }
            Err(e) => warn!("缓存模式查询失败 pattern={}: {}", pattern, e),
        }
    }
}
