use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub menu_level: Option<u64>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<u8>,
    pub component: Option<String>,
    pub sort: Option<i64>,
    pub active_name: Option<String>,
    pub keep_alive: Option<i8>,
    pub default_menu: Option<i8>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub close_tab: Option<i8>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_ad: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_ad: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_ad: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
