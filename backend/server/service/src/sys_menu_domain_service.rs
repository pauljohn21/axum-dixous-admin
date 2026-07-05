use anyhow::{anyhow, Result};
use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect, Set};

use model::dao::sys_menu_domain;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::prelude::SysMenuDomain;
use utils::db_conn;

pub struct SysMenuDomainService;

impl SysMenuDomainService {
    pub async fn insert(title: Option<String>, text: Option<String>) -> Result<sys_menu_domain::Model> {
        let db = db_conn!();
        let active = sys_menu_domain::ActiveModel {
            title: Set(title),
            text: Set(text),
            ..Default::default()
        };
        let result = SysMenuDomain::insert(active).exec(db).await?;
        SysMenuDomain::find_by_id(result.last_insert_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("创建失败"))
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_menu_domain::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let total = SysMenuDomain::find().count(db).await?;
        let list = SysMenuDomain::find()
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: i32) -> Result<sys_menu_domain::Model> {
        SysMenuDomain::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("记录不存在"))
    }

    pub async fn delete(id: i32) -> Result<()> {
        SysMenuDomain::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
