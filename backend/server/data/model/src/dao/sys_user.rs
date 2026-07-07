use sea_orm::entity::prelude::*;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub nick_name: Option<String>,
    pub side_mode: Option<String>,
    pub header_img: Option<String>,
    pub base_color: Option<String>,
    pub active_color: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub enable: Option<u64>,
    pub wx_openid: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
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

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::sys_user_role::Relation::SysRole.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sys_user_role::Relation::SysUser.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
