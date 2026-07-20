use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, Set};

use model::dao::sys_menu_domain;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::prelude::SysMenuDomain;
use utils::prelude::ServiceError;

pub struct SysMenuDomainService;

impl SysMenuDomainService {
    pub async fn insert(db: &DatabaseConnection, title: Option<String>, text: Option<String>) -> Result<sys_menu_domain::Model, ServiceError> {
        let active = sys_menu_domain::ActiveModel {
            title: Set(title),
            text: Set(text),
            ..Default::default()
        };
        let result = SysMenuDomain::insert(active).exec(db).await?;
        SysMenuDomain::find_by_id(result.last_insert_id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("创建失败".into()))
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_menu_domain::Model>, ServiceError> {
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

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<sys_menu_domain::Model, ServiceError> {
        SysMenuDomain::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("记录不存在".into()))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
        SysMenuDomain::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
