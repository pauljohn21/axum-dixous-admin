use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_role_menus")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_base_menu_id: u64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sys_role_role_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
