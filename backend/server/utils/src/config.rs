//! 系统配置文件
//!
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub datasource: Datasource,
    pub cache: Cache,
    pub logger: Logger,
    pub jwt: JwtConfig,
    #[serde(default)]
    pub wechat: WechatConfig,
}

impl Config {
    /// 从编译期内嵌的 config.yml 加载配置，并用环境变量覆盖关键字段
    pub fn load() -> Self {
        let data = include_str!("../../config.yml");
        let mut config: Config = serde_yaml::from_str(data).expect("无法读取配置信息");

        // 环境变量覆盖（命名规范: ADMIN_{SECTION}_{FIELD}）
        if let Ok(v) = env::var("ADMIN_SERVER_HOST") {
            config.server.host = v;
        }
        if let Ok(v) = env::var("ADMIN_SERVER_PORT") {
            config.server.port = v.parse().unwrap_or(config.server.port);
        }
        if let Ok(v) = env::var("ADMIN_DB_HOST") {
            config.datasource.host = v;
        }
        if let Ok(v) = env::var("ADMIN_DB_PORT") {
            config.datasource.port = v.parse().unwrap_or(config.datasource.port);
        }
        if let Ok(v) = env::var("ADMIN_DB_DATABASE") {
            config.datasource.database = v;
        }
        if let Ok(v) = env::var("ADMIN_DB_USERNAME") {
            config.datasource.username = v;
        }
        if let Ok(v) = env::var("ADMIN_DB_PASSWORD") {
            config.datasource.password = v;
        }
        if let Ok(v) = env::var("ADMIN_REDIS_HOST") {
            config.cache.host = v;
        }
        if let Ok(v) = env::var("ADMIN_REDIS_PORT") {
            config.cache.port = v.parse().unwrap_or(config.cache.port);
        }
        if let Ok(v) = env::var("ADMIN_REDIS_PASSWORD") {
            config.cache.password = v;
        }
        if let Ok(v) = env::var("ADMIN_JWT_SECRET") {
            config.jwt.secret = v;
        }
        if let Ok(v) = env::var("ADMIN_JWT_EXPIRE_HOURS") {
            config.jwt.expire_hours = v.parse().unwrap_or(config.jwt.expire_hours);
        }
        if let Ok(v) = env::var("ADMIN_WECHAT_APPID") {
            config.wechat.appid = v;
        }
        if let Ok(v) = env::var("ADMIN_WECHAT_SECRET") {
            config.wechat.secret = v;
        }

        config
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datasource {
    pub driver: String,
    pub host: String,
    pub port: i16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub config: DataOption,
    pub migration: String,
}

impl Datasource {
    pub fn db_url(&self) -> String {
        format!("{}://{}:{}@{}:{}/{}", self.driver, self.username, self.password, self.host, self.port, self.database)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub driver: String,
    pub host: String,
    pub port: i16,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: i16,
}

impl Server {
    pub fn addr(self) -> SocketAddr {
        let addr = format!("{}:{}", self.host, self.port);
        let addr = addr.as_str();
        SocketAddr::from_str(addr).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_hours: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WechatConfig {
    pub appid: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataOption {
    pub max_conn: u32,
    pub min_conn: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
    pub sqlx_level: String,
}
