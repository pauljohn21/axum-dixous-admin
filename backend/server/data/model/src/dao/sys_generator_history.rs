use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

/// 代码生成器历史记录实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_generator_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    /// 表名
    pub table_name: String,
    /// 资源名
    pub resource: String,
    /// 中文模块名
    pub module_cn: String,
    /// 前端传入的完整 YAML 配置
    #[sea_orm(column_type = "Text")]
    pub request: String,
    /// 标记: 0=创建, 1=回滚
    pub flag: i32,
    /// 生成的文件列表 JSON
    #[sea_orm(column_type = "Text", nullable)]
    pub generated_files: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
