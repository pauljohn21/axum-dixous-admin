use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CasbinRule::Table)
                    .if_not_exists()
                    .col(big_integer(CasbinRule::Id).auto_increment().primary_key())
                    .col(string_len(CasbinRule::Ptype, 100).comment("策略类型: p/g").default(""))
                    .col(string_len(CasbinRule::V0, 100).default(""))
                    .col(string_len(CasbinRule::V1, 100).default(""))
                    .col(string_len(CasbinRule::V2, 100).default(""))
                    .col(string_len(CasbinRule::V3, 100).default(""))
                    .col(string_len(CasbinRule::V4, 100).default(""))
                    .col(string_len(CasbinRule::V5, 100).default(""))
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_casbin_rule")
                            .col(CasbinRule::Ptype)
                            .col(CasbinRule::V0)
                            .col(CasbinRule::V1)
                            .col(CasbinRule::V2)
                            .col(CasbinRule::V3)
                            .col(CasbinRule::V4)
                            .col(CasbinRule::V5),
                    )
                    .to_owned(),
            )
            .await?;

        // 插入基于项目实际API的策略数据
        let insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2])
            // 角色888的API权限 (超级管理员)
            .values_panic(["p".into(), "888".into(), "/api/user/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/user/register".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/user/info".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/user/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/user/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/user/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/role".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/role/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/role/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/role/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/role/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu/user".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/menu/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/apis".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/apis/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/apis/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/apis/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/apis/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/batch".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/casbin/user/:user".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuBtn".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuBtn/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuBtn/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuBtn/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuBtn/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuParam".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuParam/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuParam/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuParam/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/menuParam/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleBtn".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleBtn/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleBtn/:role_id/:sys_menu_id/:sys_base_menu_btn_id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleBtn/:role_id/:sys_menu_id/:sys_base_menu_btn_id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleMenu".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleMenu/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleMenu/:sys_base_menu_id/:sys_role_role_id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/roleMenu/:sys_base_menu_id/:sys_role_role_id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/dataRole".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/dataRole/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dataRole/:sys_role_role_id/:data_role_id_role_id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dataRole/:sys_role_role_id/:data_role_id_role_id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/jwtBlacklist".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/jwtBlacklist/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/jwtBlacklist/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/jwtBlacklist/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/jwtBlacklist/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionary".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionary/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionary/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionary/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionary/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionaryDetail".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionaryDetail/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionaryDetail/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionaryDetail/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/dictionaryDetail/:id".into(), "DELETE".into()])
            .values_panic(["p".into(), "888".into(), "/api/operationRecord".into(), "POST".into()])
            .values_panic(["p".into(), "888".into(), "/api/operationRecord/list".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/operationRecord/:id".into(), "GET".into()])
            .values_panic(["p".into(), "888".into(), "/api/operationRecord/:id".into(), "PUT".into()])
            .values_panic(["p".into(), "888".into(), "/api/operationRecord/:id".into(), "DELETE".into()])
            // 角色8881的API权限 (普通管理员)
            .values_panic(["p".into(), "8881".into(), "/api/user/list".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/user/info".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/role/list".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/menu/list".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/menu/user".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/apis/list".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/dictionary/list".into(), "GET".into()])
            .values_panic(["p".into(), "8881".into(), "/api/dictionaryDetail/list".into(), "GET".into()])
            // 角色9528的API权限 (测试角色)
            .values_panic(["p".into(), "9528".into(), "/api/user/info".into(), "GET".into()])
            .values_panic(["p".into(), "9528".into(), "/api/role/list".into(), "GET".into()])
            .values_panic(["p".into(), "9528".into(), "/api/menu/list".into(), "GET".into()])
            .values_panic(["p".into(), "9528".into(), "/api/menu/user".into(), "GET".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        // 插入角色绑定策略 (g策略) - 连接用户和角色
        let g_insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1])
            // 用户admin绑定到角色888, 8881, 9528
            .values_panic(["g".into(), "admin".into(), "888".into()])
            .values_panic(["g".into(), "admin".into(), "8881".into()])
            .values_panic(["g".into(), "admin".into(), "9528".into()])
            // 用户test绑定到角色9528
            .values_panic(["g".into(), "test".into(), "9528".into()])
            .to_owned();

        manager.exec_stmt(g_insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CasbinRule::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum CasbinRule {
    Table,
    Id,
    Ptype,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
}
