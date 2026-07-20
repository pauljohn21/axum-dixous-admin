//! 数据源相关

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;
use tracing::log::LevelFilter;

use crate::prelude::CONFIG;

/// 全局数据库连接 (OnceCell 保证只初始化一次)
static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// 全局 Redis 连接管理器 (OnceCell 保证只初始化一次)
static REDIS_CONN: OnceCell<redis::aio::ConnectionManager> = OnceCell::const_new();

#[derive(Debug, Clone, Serialize)]
pub struct DB;

impl DB {
    fn sql_level() -> LevelFilter {
        match CONFIG.datasource.config.sqlx_level.to_string().trim() {
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "warn" => LevelFilter::Warn,
            "trace" => LevelFilter::Trace,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Off,
        }
    }

    /// 获取数据库连接 (单例，复用连接池)
    pub async fn db_connection() -> &'static DatabaseConnection {
        DB_CONN.get_or_init(|| async {
            let mut opt = ConnectOptions::new(CONFIG.datasource.db_url());
            opt.max_connections(CONFIG.datasource.config.max_conn)
                .min_connections(CONFIG.datasource.config.min_conn)
                .connect_timeout(Duration::from_secs(CONFIG.datasource.config.connect_timeout))
                .acquire_timeout(Duration::from_secs(CONFIG.datasource.config.acquire_timeout))
                .max_lifetime(Duration::from_secs(CONFIG.datasource.config.max_lifetime))
                .idle_timeout(Duration::from_secs(CONFIG.datasource.config.idle_timeout))
                .sqlx_logging_level(DB::sql_level());
            Database::connect(opt).await.expect("无法连接数据库")
        }).await
    }

    /// 获取 Redis 连接管理器 (单例，自动重连)
    pub async fn redis_connection() -> &'static redis::aio::ConnectionManager {
        REDIS_CONN.get_or_init(|| async {
            let url = if CONFIG.cache.password.is_empty() {
                format!("redis://{}:{}/", CONFIG.cache.host, CONFIG.cache.port)
            } else {
                format!("redis://:{}@{}:{}/", CONFIG.cache.password, CONFIG.cache.host, CONFIG.cache.port)
            };
            let client = redis::Client::open(url).expect("无法创建 Redis 客户端");
            client.get_connection_manager().await.expect("无法连接 Redis")
        }).await
    }
}
