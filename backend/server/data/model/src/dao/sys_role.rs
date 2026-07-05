use sea_orm::entity::prelude::*;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub en_name: Option<String>,
    pub cn_name: Option<String>,
    pub parent_id: Option<u64>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_ad: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_ad: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_ad: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_user_role::Entity")]
    SysUserRole,
}

impl Related<super::sys_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUserRole.def()
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        super::sys_user_role::Relation::SysUser.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sys_user_role::Relation::SysRole.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
