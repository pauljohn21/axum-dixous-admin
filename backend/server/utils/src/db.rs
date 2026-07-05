//! 数据源相关

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log::LevelFilter;

use crate::prelude::CONFIG;

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

    pub async fn db_connection() -> DatabaseConnection {
        let mut opt = ConnectOptions::new(CONFIG.datasource.db_url());
        opt.max_connections(CONFIG.datasource.config.max_conn)
            .min_connections(CONFIG.datasource.config.min_conn)
            .connect_timeout(Duration::from_secs(CONFIG.datasource.config.connect_timeout))
            .acquire_timeout(Duration::from_secs(CONFIG.datasource.config.acquire_timeout))
            .max_lifetime(Duration::from_secs(CONFIG.datasource.config.max_lifetime))
            .idle_timeout(Duration::from_secs(CONFIG.datasource.config.idle_timeout))
            .sqlx_logging_level(DB::sql_level());
        Database::connect(opt).await.expect("无法连接数据库")
    }
}
