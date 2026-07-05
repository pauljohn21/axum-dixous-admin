use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_apis;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_api_dto::{SysApiInsertDTO, SysApiUpdateDTO};
use model::prelude::SysApis;
use utils::db_conn;

pub struct SysApiService;

impl SysApiService {
    pub async fn insert(data: SysApiInsertDTO) -> Result<sys_apis::Model> {
        let db = db_conn!();
        let active = sys_apis::ActiveModel {
            path: Set(data.path),
            description: Set(data.description),
            api_group: Set(data.api_group),
            method: Set(data.method),
            ..Default::default()
        };
        let result = SysApis::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_apis::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysApis::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_apis::Column::Path.contains(keyword))
                    .add(sys_apis::Column::Description.contains(keyword))
                    .add(sys_apis::Column::ApiGroup.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: i64) -> Result<sys_apis::Model> {
        SysApis::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("API不存在"))
    }

    pub async fn update(id: i64, data: SysApiUpdateDTO) -> Result<sys_apis::Model> {
        let db = db_conn!();
        let api: sys_apis::ActiveModel = SysApis::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("API不存在"))?
            .into();
        let mut updated = api;
        if let Some(v) = data.path { updated.path = Set(Some(v)); }
        if let Some(v) = data.description { updated.description = Set(Some(v)); }
        if let Some(v) = data.api_group { updated.api_group = Set(Some(v)); }
        if let Some(v) = data.method { updated.method = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: i64) -> Result<()> {
        SysApis::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
