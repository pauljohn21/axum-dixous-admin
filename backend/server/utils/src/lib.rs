//! 工具类 模块
//! 主要提供密码加密,配置信息获取

#[macro_use]
extern crate serde;

mod auth;
mod config;
mod db;
mod error;
mod level;
mod password_utils;
pub mod prelude;
mod rand_utils;
mod res;
mod state;

#[macro_export]
macro_rules! db_conn {
    () => {
        utils::prelude::DB::db_connection().await
    };
}

