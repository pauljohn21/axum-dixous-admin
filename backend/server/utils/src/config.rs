//! 系统配置文件
//!
use std::net::SocketAddr;
use std::str::FromStr;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::default);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub datasource: Datasource,
    pub cache: Cache,
    pub logger: Logger,
    pub jwt: JwtConfig,
}

impl Default for Config {
    fn default() -> Self {
        let data = include_str!("../../config.yml");
        let result: Config = serde_yaml::from_str(data).expect("无法读取配置信息");
        result
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
