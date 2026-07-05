use anyhow::{anyhow, Result};
use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect, Set};

use model::dao::sys_menu_role;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::prelude::SysMenuRole;
use utils::db_conn;

pub struct SysMenuRoleService;

impl SysMenuRoleService {
    pub async fn insert(title: Option<String>, text: Option<String>) -> Result<sys_menu_role::Model> {
        let db = db_conn!();
        let active = sys_menu_role::ActiveModel {
            title: Set(title),
            text: Set(text),
            ..Default::default()
        };
        let result = SysMenuRole::insert(active).exec(db).await?;
        SysMenuRole::find_by_id(result.last_insert_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("创建失败"))
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_menu_role::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let total = SysMenuRole::find().count(db).await?;
        let list = SysMenuRole::find()
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: i32) -> Result<sys_menu_role::Model> {
        SysMenuRole::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("记录不存在"))
    }

    pub async fn delete(id: i32) -> Result<()> {
        SysMenuRole::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
