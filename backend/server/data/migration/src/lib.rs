//! 数据迁移 模块
//! 用于定义数据源结构与初始化数据
pub use sea_orm_migration::prelude::*;
use tracing::info;

use utils::prelude::{CONFIG, DB};

mod m20220101_000001_create_sys_user;
mod m20230705_052744_create_sys_role;
mod m20230705_053111_create_sys_user_role;
mod m20240422_075347_create_sys_menu;
mod m20240423_112033_create_sys_apis;
mod m20240424_074636_create_sys_menu_role;
mod m20250211_071223_create_sys_menu_domain;
mod m20260701_000001_create_casbin_rule;
mod m20260701_000002_create_jwt_blacklists;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_sys_user::Migration),
            Box::new(m20230705_052744_create_sys_role::Migration),
            Box::new(m20230705_053111_create_sys_user_role::Migration),
            Box::new(m20240422_075347_create_sys_menu::Migration),
            Box::new(m20240423_112033_create_sys_apis::Migration),
            Box::new(m20240424_074636_create_sys_menu_role::Migration),
            Box::new(m20250211_071223_create_sys_menu_domain::Migration),
            Box::new(m20260701_000001_create_casbin_rule::Migration),
            Box::new(m20260701_000002_create_jwt_blacklists::Migration),
        ]
    }
}

impl Migrator {
    pub async fn migration_init() {
        match CONFIG.datasource.migration.to_string().trim() {
            "fresh" => match Migrator::fresh(&DB::db_connection().await).await {
                Ok(_) => info!("初始化成功"),
                Err(e) => info!("初始化失败: {}",e)
            }
            "up" => Migrator::up(&DB::db_connection().await, None).await.expect("无法升级数据源"),
            "down" => Migrator::down(&DB::db_connection().await, None).await.expect("无法降级数据源"),
            "reset" => Migrator::reset(&DB::db_connection().await).await.expect("无法重置数据源"),
            _ => println!("无数据迁移"),
        }
    }
}
