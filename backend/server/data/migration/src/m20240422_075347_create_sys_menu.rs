use crate::Keyword::CurrentTimestamp;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysMenu::Table)
                    .if_not_exists()
                    .col(pk_auto(SysMenu::Id))
                    .col(big_unsigned(SysMenu::MenuLevel).comment("菜单等级"))
                    .col(big_unsigned(SysMenu::ParentId).comment("父菜单ID"))
                    .col(string_len(SysMenu::Path, 199).comment("路由path"))
                    .col(string_len(SysMenu::Name, 199).comment("路由name"))
                    .col(tiny_unsigned(SysMenu::Hidden).comment("是否在列表隐藏"))
                    .col(string(SysMenu::Component).comment("对应前端组件标识"))
                    .col(big_integer(SysMenu::Sort).comment("排序标记"))
                    .col(string(SysMenu::ActiveName).comment("附加属性"))
                    .col(tiny_integer(SysMenu::KeepAlive).comment("附加属性"))
                    .col(tiny_integer(SysMenu::DefaultMenu).comment("附加属性"))
                    .col(string(SysMenu::Title).comment("附加属性"))
                    .col(string(SysMenu::Icon).comment("附加属性"))
                    .col(tiny_integer(SysMenu::CloseTab).comment("附加属性"))
                    .col(timestamp_with_time_zone(SysMenu::CreatedAd).default(CurrentTimestamp).comment("创建时间"))
                    .col(timestamp_with_time_zone(SysMenu::UpdatedAd).default(CurrentTimestamp).comment("修改时间"))
                    .col(timestamp_with_time_zone_null(SysMenu::DeletedAd).comment("删除时间"))
                    .index(Index::create().name("idx_sys_base_menus_deleted_at").col(SysMenu::DeletedAd))
                    .comment("系统菜单表")
                    .to_owned(),
            )
            .await?;

        // 菜单数据 — 与前端 Dioxus 路由保持一致
        //
        // 菜单结构:
        //   1. 仪表盘        (dashboard)     parent=0  sort=1
        //   2. 超级管理员     (admin)         parent=0  sort=2  [父菜单]
        //      3. 角色管理   (role)          parent=2  sort=1
        //      4. 菜单管理   (menu)          parent=2  sort=2
        //      5. API管理    (api)           parent=2  sort=3
        //      6. 用户管理   (user)          parent=2  sort=4
        //   7. 字典管理      (dictionary)    parent=0  sort=3
        //   8. 个人信息      (profile)       parent=0  sort=4
        //   9. 系统设置      (settings)      parent=0  sort=5
        let insert = Query::insert()
            .into_table(SysMenu::Table)
            .columns([
                SysMenu::MenuLevel,
                SysMenu::ParentId,
                SysMenu::Path,
                SysMenu::Name,
                SysMenu::Hidden,
                SysMenu::Component,
                SysMenu::Sort,
                SysMenu::ActiveName,
                SysMenu::KeepAlive,
                SysMenu::DefaultMenu,
                SysMenu::Title,
                SysMenu::Icon,
                SysMenu::CloseTab,
            ])
            .values_panic([
                // ID=1: 仪表盘
                0.into(),
                0.into(),
                "dashboard".into(),
                "dashboard".into(),
                0.into(),
                "".into(),
                1.into(),
                "".into(),
                0.into(),
                0.into(),
                "仪表盘".into(),
                "odometer".into(),
                0.into(),
            ])
            .values_panic([
                // ID=2: 超级管理员（父菜单）
                0.into(),
                0.into(),
                "admin".into(),
                "superAdmin".into(),
                0.into(),
                "".into(),
                2.into(),
                "".into(),
                0.into(),
                0.into(),
                "超级管理员".into(),
                "user".into(),
                0.into(),
            ])
            .values_panic([
                // ID=3: 角色管理（子菜单，parent=2）
                0.into(),
                2.into(),
                "role".into(),
                "role".into(),
                0.into(),
                "".into(),
                1.into(),
                "".into(),
                0.into(),
                0.into(),
                "角色管理".into(),
                "avatar".into(),
                0.into(),
            ])
            .values_panic([
                // ID=4: 菜单管理（子菜单，parent=2）
                0.into(),
                2.into(),
                "menu".into(),
                "menu".into(),
                0.into(),
                "".into(),
                2.into(),
                "".into(),
                0.into(),
                0.into(),
                "菜单管理".into(),
                "tickets".into(),
                0.into(),
            ])
            .values_panic([
                // ID=5: API管理（子菜单，parent=2）
                0.into(),
                2.into(),
                "api".into(),
                "api".into(),
                0.into(),
                "".into(),
                3.into(),
                "".into(),
                0.into(),
                0.into(),
                "API管理".into(),
                "platform".into(),
                0.into(),
            ])
            .values_panic([
                // ID=6: 用户管理（子菜单，parent=2）
                0.into(),
                2.into(),
                "user".into(),
                "user".into(),
                0.into(),
                "".into(),
                4.into(),
                "".into(),
                0.into(),
                0.into(),
                "用户管理".into(),
                "coordinate".into(),
                0.into(),
            ])
            .values_panic([
                // ID=7: 字典管理
                0.into(),
                0.into(),
                "dictionary".into(),
                "dictionary".into(),
                0.into(),
                "".into(),
                3.into(),
                "".into(),
                0.into(),
                0.into(),
                "字典管理".into(),
                "dict".into(),
                0.into(),
            ])
            .values_panic([
                // ID=8: 个人信息
                0.into(),
                0.into(),
                "profile".into(),
                "profile".into(),
                0.into(),
                "".into(),
                4.into(),
                "".into(),
                0.into(),
                0.into(),
                "个人信息".into(),
                "message".into(),
                0.into(),
            ])
            .values_panic([
                // ID=9: 系统设置
                0.into(),
                0.into(),
                "settings".into(),
                "settings".into(),
                0.into(),
                "".into(),
                5.into(),
                "".into(),
                0.into(),
                0.into(),
                "系统设置".into(),
                "setting".into(),
                0.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SysMenu::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum SysMenu {
    Table,
    Id,
    MenuLevel,
    ParentId,
    Path,
    Name,
    Hidden,
    Component,
    Sort,
    ActiveName,
    KeepAlive,
    DefaultMenu,
    Title,
    Icon,
    CloseTab,
    CreatedAd,
    UpdatedAd,
    DeletedAd,
}
