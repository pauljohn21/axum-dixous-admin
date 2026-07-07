use sea_orm_migration::{prelude::*, schema::*};

use crate::Keyword::CurrentTimestamp;
use utils::prelude::PasswordUtils;

/// 系统用户表
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SysUser::Table)
                    .if_not_exists()
                    .col(pk_auto(SysUser::Id))
                    .col(string(SysUser::Username).comment("用户登录名"))
                    .col(string(SysUser::Password).comment("密码"))
                    .col(string(SysUser::Salt).comment("加密盐"))
                    .col(string_null(SysUser::NickName).comment("用户昵称"))
                    .col(string(SysUser::SideMode).default("dark").comment("用户侧边主题"))
                    .col(string(SysUser::HeaderImg).default("https://qmplusimg.henrongyi.top/gva_header.jpg").comment("用户头像"))
                    .col(string(SysUser::BaseColor).string().default("#fff").comment("基础颜色"))
                    .col(string(SysUser::ActiveColor).default("'#1890ff'").comment("活跃颜色"))
                    .col(string_null(SysUser::Phone).comment("手机号"))
                    .col(string_null(SysUser::Email).comment("邮箱"))
                    .col(big_unsigned(SysUser::Enable).default("1").comment("用户是否被冻结 1正常 2冻结"))
                    .col(string_len_null(SysUser::WxOpenid, 64).comment("微信 openid"))
                    .col(string_null(SysUser::CreatedAt).comment("创建者"))
                    .col(string_null(SysUser::UpdatedAt).comment("修改者"))
                    .col(string_null(SysUser::DeletedAt).comment("删除者"))
                    .col(timestamp_with_time_zone(SysUser::CreatedAd).default(CurrentTimestamp).comment("创建时间"))
                    .col(timestamp_with_time_zone(SysUser::UpdatedAd).default(CurrentTimestamp).comment("修改时间"))
                    .col(timestamp_with_time_zone_null(SysUser::DeletedAd).comment("删除时间"))
                    .index(Index::create().unique().name("idx_sys_user_username").col(SysUser::Username))
                    .index(Index::create().unique().name("idx_sys_user_wx_openid").col(SysUser::WxOpenid))
                    .comment("系统用户表")
                    .to_owned(),
            )
            .await?;
        let admin = "admin";
        let pass = "123456";
        let hash = PasswordUtils::encrypt(pass);
        let insert = Query::insert()
            .into_table(SysUser::Table)
            .columns([SysUser::Username, SysUser::Password, SysUser::Salt, SysUser::Phone, SysUser::Email])
            .values_panic([
                admin.into(),
                hash.password_hash.clone().into(),
                hash.salt.clone().into(),
                "1234567890123".into(),
                "12354@qq.com".into(),
            ])
            .values_panic([
                "test".into(),
                hash.password_hash.clone().into(),
                hash.salt.clone().into(),
                "123456790123".into(),
                "test@qq.com".into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own .migration scripts

        manager.drop_table(Table::drop().table(SysUser::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum SysUser {
    Table,
    Id,
    Username,
    Password,
    Salt,
    NickName,
    SideMode,
    HeaderImg,
    BaseColor,
    ActiveColor,
    Phone,
    Email,
    Enable,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    CreatedAd,
    UpdatedAd,
    DeletedAd,
    WxOpenid,
}
