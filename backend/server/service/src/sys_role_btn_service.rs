use anyhow::{anyhow, Result};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_role_btns;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_btn_dto::SysRoleBtnInsertDTO;
use model::prelude::SysRoleBtns;
use utils::db_conn;

pub struct SysRoleBtnService;

impl SysRoleBtnService {
    pub async fn insert(data: SysRoleBtnInsertDTO) -> Result<sys_role_btns::Model> {
        let db = db_conn!();
        let active = sys_role_btns::ActiveModel {
            role_id: Set(data.role_id),
            sys_menu_id: Set(data.sys_menu_id),
            sys_base_menu_btn_id: Set(data.sys_base_menu_btn_id),
        };
        SysRoleBtns::insert(active).exec(db).await?;
        // For composite PK, find the inserted record
        Self::get_by_composite_id(data.role_id, data.sys_menu_id, data.sys_base_menu_btn_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_role_btns::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let total = SysRoleBtns::find().count(db).await?;
        let list = SysRoleBtns::find()
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_composite_id(role_id: u64, sys_menu_id: u64, sys_base_menu_btn_id: u64) -> Result<sys_role_btns::Model> {
        SysRoleBtns::find()
            .filter(sys_role_btns::Column::RoleId.eq(role_id))
            .filter(sys_role_btns::Column::SysMenuId.eq(sys_menu_id))
            .filter(sys_role_btns::Column::SysBaseMenuBtnId.eq(sys_base_menu_btn_id))
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("角色按钮关联不存在"))
    }

    pub async fn delete(role_id: u64, sys_menu_id: u64, sys_base_menu_btn_id: u64) -> Result<()> {
        SysRoleBtns::delete_many()
            .filter(sys_role_btns::Column::RoleId.eq(role_id))
            .filter(sys_role_btns::Column::SysMenuId.eq(sys_menu_id))
            .filter(sys_role_btns::Column::SysBaseMenuBtnId.eq(sys_base_menu_btn_id))
            .exec(db_conn!())
            .await?;
        Ok(())
    }
}
