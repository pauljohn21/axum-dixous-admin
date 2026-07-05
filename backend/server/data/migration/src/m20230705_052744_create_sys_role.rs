use sea_orm_migration::{prelude::*, schema::*};

use crate::Keyword::CurrentTimestamp;

/// 系统角色表
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SysRole::Table)
                    .if_not_exists()
                    .col(pk_auto(SysRole::Id))
                    .col(string_null(SysRole::EnName).comment("英文名"))
                    .col(string_null(SysRole::CnName).comment("中文名"))
                    .col(big_unsigned(SysRole::ParentId).comment("父级ID"))
                    .col(timestamp_with_time_zone(SysRole::CreatedAd).default(CurrentTimestamp).comment("创建时间"))
                    .col(timestamp_with_time_zone(SysRole::UpdatedAd).default(CurrentTimestamp).comment("修改时间"))
                    .col(timestamp_with_time_zone_null(SysRole::DeletedAd).comment("删除时间"))
                    .comment("系统角色")
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(SysRole::Table)
            .columns([SysRole::Id, SysRole::CnName, SysRole::EnName, SysRole::ParentId])
            .values_panic([888.into(), "管理员".into(), "admin".into(), "0".into()])
            .values_panic([8881.into(), "普通用户".into(), "user".into(), "888".into()])
            .values_panic([9528.into(), "测试".into(), "test".into(), "0".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager.drop_table(Table::drop().table(SysRole::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum SysRole {
    Table,
    Id,
    EnName,
    CnName,
    ParentId,
    CreatedAd,
    UpdatedAd,
    DeletedAd,
}
