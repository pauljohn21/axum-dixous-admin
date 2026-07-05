use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_sys_user::SysUser;
use crate::m20230705_052744_create_sys_role::SysRole;

// 系统用户角色关联表
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SysUserRole::Table)
                    .if_not_exists()
                    .col(integer(SysUserRole::UserId))
                    .col(integer(SysUserRole::RoleId))
                    .primary_key(Index::create().col(SysUserRole::UserId).col(SysUserRole::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("sys_user_role_fk_user_id")
                            .from(SysUserRole::Table, SysUserRole::UserId)
                            .to(SysUser::Table, SysUser::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("sys_user_role_fk_role_id")
                            .from(SysUserRole::Table, SysUserRole::RoleId)
                            .to(SysRole::Table, SysRole::Id),
                    )
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(SysUserRole::Table)
            .columns([SysUserRole::UserId, SysUserRole::RoleId])
            .values_panic([1.into(), 888.into()])
            .values_panic([1.into(), 8881.into()])
            .values_panic([1.into(), 9528.into()])
            .values_panic([2.into(), 9528.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager.drop_table(Table::drop().table(SysUserRole::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SysUserRole {
    Table,
    UserId,
    RoleId,
}
