use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_dictionaries;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_dictionary_dto::{SysDictionaryInsertDTO, SysDictionaryUpdateDTO};
use model::prelude::SysDictionaries;
use utils::db_conn;

pub struct SysDictionaryService;

impl SysDictionaryService {
    pub async fn insert(data: SysDictionaryInsertDTO) -> Result<sys_dictionaries::Model> {
        let db = db_conn!();
        let active = sys_dictionaries::ActiveModel {
            name: Set(data.name),
            r#type: Set(data.r#type),
            status: Set(data.status),
            desc: Set(data.desc),
            ..Default::default()
        };
        let result = SysDictionaries::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_dictionaries::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysDictionaries::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_dictionaries::Column::Name.contains(keyword))
                    .add(sys_dictionaries::Column::Desc.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<sys_dictionaries::Model> {
        SysDictionaries::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("字典不存在"))
    }

    pub async fn update(id: u64, data: SysDictionaryUpdateDTO) -> Result<sys_dictionaries::Model> {
        let db = db_conn!();
        let dict: sys_dictionaries::ActiveModel = SysDictionaries::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("字典不存在"))?
            .into();
        let mut updated = dict;
        if let Some(v) = data.name { updated.name = Set(Some(v)); }
        if let Some(v) = data.r#type { updated.r#type = Set(Some(v)); }
        if let Some(v) = data.status { updated.status = Set(Some(v)); }
        if let Some(v) = data.desc { updated.desc = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: u64) -> Result<()> {
        SysDictionaries::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
