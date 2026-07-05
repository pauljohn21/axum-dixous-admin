use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysApis::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysApis::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysApis::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysApis::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysApis::DeletedAt).date_time().null())
                    .col(
                        ColumnDef::new(SysApis::Path)
                            .string()
                            .string_len(191)
                            .null()
                            .comment("api路径"),
                    )
                    .col(
                        ColumnDef::new(SysApis::Description)
                            .string()
                            .string_len(191)
                            .null()
                            .comment("api中文描述"),
                    )
                    .col(
                        ColumnDef::new(SysApis::ApiGroup)
                            .string()
                            .string_len(191)
                            .null()
                            .comment("api组"),
                    )
                    .col(
                        ColumnDef::new(SysApis::Method)
                            .string()
                            .string_len(191)
                            .default("POST")
                            .comment("方法"),
                    )
                    .index(
                        Index::create()
                            .name("idx_sys_apis_deleted_at")
                            .col(SysApis::DeletedAt),
                    )
                    .to_owned(),
            )
            .await?;

            // 插入基于项目实际API的API数据
            let insert = Query::insert()
                .into_table(SysApis::Table)
                .columns([
                    SysApis::Id,
                    SysApis::CreatedAt,
                    SysApis::UpdatedAt,
                    SysApis::Path,
                    SysApis::Description,
                    SysApis::ApiGroup,
                    SysApis::Method,
                ])
            // 用户管理API
            .values_panic([
                1.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/login".into(),
                "用户登录".into(),
                "用户管理".into(),
                "POST".into(),
            ])
            .values_panic([
                2.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/register".into(),
                "用户注册".into(),
                "用户管理".into(),
                "POST".into(),
            ])
            .values_panic([
                3.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/list".into(),
                "获取用户列表".into(),
                "用户管理".into(),
                "GET".into(),
            ])
            .values_panic([
                4.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/info".into(),
                "获取用户信息".into(),
                "用户管理".into(),
                "GET".into(),
            ])
            .values_panic([
                5.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/:id".into(),
                "根据ID获取用户".into(),
                "用户管理".into(),
                "GET".into(),
            ])
            .values_panic([
                6.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/:id".into(),
                "更新用户信息".into(),
                "用户管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                7.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/user/:id".into(),
                "删除用户".into(),
                "用户管理".into(),
                "DELETE".into(),
            ])
            // 角色管理API
            .values_panic([
                8.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/role".into(),
                "创建角色".into(),
                "角色管理".into(),
                "POST".into(),
            ])
            .values_panic([
                9.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/role/list".into(),
                "获取角色列表".into(),
                "角色管理".into(),
                "GET".into(),
            ])
            .values_panic([
                10.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/role/:id".into(),
                "根据ID获取角色".into(),
                "角色管理".into(),
                "GET".into(),
            ])
            .values_panic([
                11.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/role/:id".into(),
                "更新角色信息".into(),
                "角色管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                12.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/role/:id".into(),
                "删除角色".into(),
                "角色管理".into(),
                "DELETE".into(),
            ])
            // 菜单管理API
            .values_panic([
                13.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu".into(),
                "创建菜单".into(),
                "菜单管理".into(),
                "POST".into(),
            ])
            .values_panic([
                14.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu/list".into(),
                "获取菜单列表".into(),
                "菜单管理".into(),
                "GET".into(),
            ])
            .values_panic([
                15.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu/user".into(),
                "获取用户菜单".into(),
                "菜单管理".into(),
                "GET".into(),
            ])
            .values_panic([
                16.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu/:id".into(),
                "根据ID获取菜单".into(),
                "菜单管理".into(),
                "GET".into(),
            ])
            .values_panic([
                17.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu/:id".into(),
                "更新菜单信息".into(),
                "菜单管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                18.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menu/:id".into(),
                "删除菜单".into(),
                "菜单管理".into(),
                "DELETE".into(),
            ])
            // API管理API
            .values_panic([
                19.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/apis".into(),
                "创建API".into(),
                "API管理".into(),
                "POST".into(),
            ])
            .values_panic([
                20.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/apis/list".into(),
                "获取API列表".into(),
                "API管理".into(),
                "GET".into(),
            ])
            .values_panic([
                21.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/apis/:id".into(),
                "根据ID获取API".into(),
                "API管理".into(),
                "GET".into(),
            ])
            .values_panic([
                22.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/apis/:id".into(),
                "更新API信息".into(),
                "API管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                23.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/apis/:id".into(),
                "删除API".into(),
                "API管理".into(),
                "DELETE".into(),
            ])
            // Casbin策略管理API
            .values_panic([
                24.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/list".into(),
                "获取Casbin规则列表".into(),
                "权限策略".into(),
                "GET".into(),
            ])
            .values_panic([
                25.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/:id".into(),
                "根据ID获取Casbin规则".into(),
                "权限策略".into(),
                "GET".into(),
            ])
            .values_panic([
                26.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin".into(),
                "创建Casbin规则".into(),
                "权限策略".into(),
                "POST".into(),
            ])
            .values_panic([
                27.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/:id".into(),
                "更新Casbin规则".into(),
                "权限策略".into(),
                "PUT".into(),
            ])
            .values_panic([
                28.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/:id".into(),
                "删除Casbin规则".into(),
                "权限策略".into(),
                "DELETE".into(),
            ])
            .values_panic([
                29.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/batch".into(),
                "批量删除Casbin规则".into(),
                "权限策略".into(),
                "DELETE".into(),
            ])
            .values_panic([
                30.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/role/{role}".into(),
                "获取角色策略".into(),
                "权限策略".into(),
                "GET".into(),
            ])
            .values_panic([
                31.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/role/{role}".into(),
                "更新角色策略".into(),
                "权限策略".into(),
                "PUT".into(),
            ])
            .values_panic([
                32.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/casbin/user/{user}".into(),
                "获取用户角色".into(),
                "权限策略".into(),
                "GET".into(),
            ])
            // 菜单按钮管理API
            .values_panic([
                33.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuBtn".into(),
                "创建菜单按钮".into(),
                "菜单按钮".into(),
                "POST".into(),
            ])
            .values_panic([
                34.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuBtn/list".into(),
                "获取菜单按钮列表".into(),
                "菜单按钮".into(),
                "GET".into(),
            ])
            .values_panic([
                35.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuBtn/:id".into(),
                "根据ID获取菜单按钮".into(),
                "菜单按钮".into(),
                "GET".into(),
            ])
            .values_panic([
                36.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuBtn/:id".into(),
                "更新菜单按钮".into(),
                "菜单按钮".into(),
                "PUT".into(),
            ])
            .values_panic([
                37.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuBtn/:id".into(),
                "删除菜单按钮".into(),
                "菜单按钮".into(),
                "DELETE".into(),
            ])
            // 菜单参数管理API
            .values_panic([
                38.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuParam".into(),
                "创建菜单参数".into(),
                "菜单参数".into(),
                "POST".into(),
            ])
            .values_panic([
                39.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuParam/list".into(),
                "获取菜单参数列表".into(),
                "菜单参数".into(),
                "GET".into(),
            ])
            .values_panic([
                40.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuParam/:id".into(),
                "根据ID获取菜单参数".into(),
                "菜单参数".into(),
                "GET".into(),
            ])
            .values_panic([
                41.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuParam/:id".into(),
                "更新菜单参数".into(),
                "菜单参数".into(),
                "PUT".into(),
            ])
            .values_panic([
                42.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/menuParam/:id".into(),
                "删除菜单参数".into(),
                "菜单参数".into(),
                "DELETE".into(),
            ])
            // 角色按钮管理API
            .values_panic([
                43.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleBtn".into(),
                "创建角色按钮权限".into(),
                "角色按钮".into(),
                "POST".into(),
            ])
            .values_panic([
                44.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleBtn/list".into(),
                "获取角色按钮权限列表".into(),
                "角色按钮".into(),
                "GET".into(),
            ])
            .values_panic([
                45.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}".into(),
                "根据复合ID获取角色按钮权限".into(),
                "角色按钮".into(),
                "GET".into(),
            ])
            .values_panic([
                46.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}".into(),
                "删除角色按钮权限".into(),
                "角色按钮".into(),
                "DELETE".into(),
            ])
            // 角色菜单管理API
            .values_panic([
                47.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleMenu".into(),
                "创建角色菜单权限".into(),
                "角色菜单".into(),
                "POST".into(),
            ])
            .values_panic([
                48.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleMenu/list".into(),
                "获取角色菜单权限列表".into(),
                "角色菜单".into(),
                "GET".into(),
            ])
            .values_panic([
                49.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleMenu/{sys_base_menu_id}/{sys_role_role_id}".into(),
                "根据复合ID获取角色菜单权限".into(),
                "角色菜单".into(),
                "GET".into(),
            ])
            .values_panic([
                50.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/roleMenu/{sys_base_menu_id}/{sys_role_role_id}".into(),
                "删除角色菜单权限".into(),
                "角色菜单".into(),
                "DELETE".into(),
            ])
            // 数据权限管理API
            .values_panic([
                51.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dataRole".into(),
                "创建数据权限".into(),
                "数据权限".into(),
                "POST".into(),
            ])
            .values_panic([
                52.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dataRole/list".into(),
                "获取数据权限列表".into(),
                "数据权限".into(),
                "GET".into(),
            ])
            .values_panic([
                53.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dataRole/{sys_role_role_id}/{data_role_id_role_id}".into(),
                "根据复合ID获取数据权限".into(),
                "数据权限".into(),
                "GET".into(),
            ])
            .values_panic([
                54.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dataRole/{sys_role_role_id}/{data_role_id_role_id}".into(),
                "删除数据权限".into(),
                "数据权限".into(),
                "DELETE".into(),
            ])
            // JWT黑名单管理API
            .values_panic([
                55.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/jwtBlacklist".into(),
                "创建JWT黑名单".into(),
                "JWT管理".into(),
                "POST".into(),
            ])
            .values_panic([
                56.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/jwtBlacklist/list".into(),
                "获取JWT黑名单列表".into(),
                "JWT管理".into(),
                "GET".into(),
            ])
            .values_panic([
                57.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/jwtBlacklist/:id".into(),
                "根据ID获取JWT黑名单".into(),
                "JWT管理".into(),
                "GET".into(),
            ])
            .values_panic([
                58.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/jwtBlacklist/:id".into(),
                "更新JWT黑名单".into(),
                "JWT管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                59.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/jwtBlacklist/:id".into(),
                "删除JWT黑名单".into(),
                "JWT管理".into(),
                "DELETE".into(),
            ])
            // 字典管理API
            .values_panic([
                60.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionary".into(),
                "创建字典".into(),
                "字典管理".into(),
                "POST".into(),
            ])
            .values_panic([
                61.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionary/list".into(),
                "获取字典列表".into(),
                "字典管理".into(),
                "GET".into(),
            ])
            .values_panic([
                62.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionary/:id".into(),
                "根据ID获取字典".into(),
                "字典管理".into(),
                "GET".into(),
            ])
            .values_panic([
                63.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionary/:id".into(),
                "更新字典信息".into(),
                "字典管理".into(),
                "PUT".into(),
            ])
            .values_panic([
                64.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionary/:id".into(),
                "删除字典".into(),
                "字典管理".into(),
                "DELETE".into(),
            ])
            // 字典详情管理API
            .values_panic([
                65.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionaryDetail".into(),
                "创建字典详情".into(),
                "字典详情".into(),
                "POST".into(),
            ])
            .values_panic([
                66.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionaryDetail/list".into(),
                "获取字典详情列表".into(),
                "字典详情".into(),
                "GET".into(),
            ])
            .values_panic([
                67.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionaryDetail/:id".into(),
                "根据ID获取字典详情".into(),
                "字典详情".into(),
                "GET".into(),
            ])
            .values_panic([
                68.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionaryDetail/:id".into(),
                "更新字典详情".into(),
                "字典详情".into(),
                "PUT".into(),
            ])
            .values_panic([
                69.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/dictionaryDetail/:id".into(),
                "删除字典详情".into(),
                "字典详情".into(),
                "DELETE".into(),
            ])
            // 操作记录管理API
            .values_panic([
                70.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/operationRecord".into(),
                "创建操作记录".into(),
                "操作记录".into(),
                "POST".into(),
            ])
            .values_panic([
                71.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/operationRecord/list".into(),
                "获取操作记录列表".into(),
                "操作记录".into(),
                "GET".into(),
            ])
            .values_panic([
                72.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/operationRecord/:id".into(),
                "根据ID获取操作记录".into(),
                "操作记录".into(),
                "GET".into(),
            ])
            .values_panic([
                73.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/operationRecord/:id".into(),
                "更新操作记录".into(),
                "操作记录".into(),
                "PUT".into(),
            ])
            .values_panic([
                74.into(),
                "2024-04-23 11:20:33.270".into(),
                "2024-04-23 11:20:33.270".into(),
                "/api/operationRecord/:id".into(),
                "删除操作记录".into(),
                "操作记录".into(),
                "DELETE".into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysApis::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum SysApis {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Path,
    Description,
    ApiGroup,
    Method,
}