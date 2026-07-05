use anyhow::{anyhow, Result};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_role_menus;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_menu_dto::SysRoleMenuInsertDTO;
use model::prelude::SysRoleMenus;
use utils::db_conn;

pub struct SysRoleMenuService;

impl SysRoleMenuService {
    pub async fn insert(data: SysRoleMenuInsertDTO) -> Result<sys_role_menus::Model> {
        let db = db_conn!();
        let active = sys_role_menus::ActiveModel {
            sys_base_menu_id: Set(data.sys_base_menu_id),
            sys_role_role_id: Set(data.sys_role_role_id),
        };
        SysRoleMenus::insert(active).exec(db).await?;
        // For composite PK, find the inserted record
        SysRoleMenus::find()
            .filter(sys_role_menus::Column::SysBaseMenuId.eq(data.sys_base_menu_id))
            .filter(sys_role_menus::Column::SysRoleRoleId.eq(data.sys_role_role_id))
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("创建失败"))
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_role_menus::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let total = SysRoleMenus::find().count(db).await?;
        let list = SysRoleMenus::find()
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_composite_id(sys_base_menu_id: u64, sys_role_role_id: u64) -> Result<sys_role_menus::Model> {
        SysRoleMenus::find()
            .filter(sys_role_menus::Column::SysBaseMenuId.eq(sys_base_menu_id))
            .filter(sys_role_menus::Column::SysRoleRoleId.eq(sys_role_role_id))
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("角色菜单关联不存在"))
    }

    pub async fn delete(sys_base_menu_id: u64, sys_role_role_id: u64) -> Result<()> {
        SysRoleMenus::delete_many()
            .filter(sys_role_menus::Column::SysBaseMenuId.eq(sys_base_menu_id))
            .filter(sys_role_menus::Column::SysRoleRoleId.eq(sys_role_role_id))
            .exec(db_conn!())
            .await?;
        Ok(())
    }
}
