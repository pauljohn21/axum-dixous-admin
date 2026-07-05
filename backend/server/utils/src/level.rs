use tracing::info;
use tracing::metadata::LevelFilter;

use crate::prelude::CONFIG;

#[derive(Debug)]
pub struct Level;

impl Level {
    pub fn init() {
        let level = match CONFIG.logger.level.to_string().trim() {
            "debug" => LevelFilter::DEBUG,
            "error" => LevelFilter::ERROR,
            "warn" => LevelFilter::WARN,
            "info" => LevelFilter::INFO,
            "trace" => LevelFilter::TRACE,
            _ => LevelFilter::OFF,
        };
        tracing_subscriber::fmt().with_max_level(level).init();
        info!("日志使用 {} 模式", level)
    }
}
