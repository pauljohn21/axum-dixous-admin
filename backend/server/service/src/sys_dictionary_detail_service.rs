use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_dictionary_details;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_dictionary_detail_dto::{SysDictionaryDetailInsertDTO, SysDictionaryDetailUpdateDTO};
use model::prelude::SysDictionaryDetails;
use utils::prelude::ServiceError;

pub struct SysDictionaryDetailService;

impl SysDictionaryDetailService {
    pub async fn insert(db: &DatabaseConnection, data: SysDictionaryDetailInsertDTO) -> Result<sys_dictionary_details::Model, ServiceError> {
        let active = sys_dictionary_details::ActiveModel {
            label: Set(data.label),
            value: Set(data.value),
            extend: Set(data.extend),
            status: Set(data.status),
            sort: Set(data.sort),
            sys_dictionary_id: Set(data.sys_dictionary_id),
            ..Default::default()
        };
        let result = SysDictionaryDetails::insert(active).exec(db).await?;
        Self::get_by_id(db, result.last_insert_id).await
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_dictionary_details::Model>, ServiceError> {
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysDictionaryDetails::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_dictionary_details::Column::Label.contains(keyword))
                    .add(sys_dictionary_details::Column::Value.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<sys_dictionary_details::Model, ServiceError> {
        SysDictionaryDetails::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("字典详情不存在".into()))
    }

    pub async fn update(db: &DatabaseConnection, id: u64, data: SysDictionaryDetailUpdateDTO) -> Result<sys_dictionary_details::Model, ServiceError> {
        let detail: sys_dictionary_details::ActiveModel = SysDictionaryDetails::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("字典详情不存在".into()))?
            .into();
        let mut updated = detail;
        if let Some(v) = data.label { updated.label = Set(Some(v)); }
        if let Some(v) = data.value { updated.value = Set(Some(v)); }
        if let Some(v) = data.extend { updated.extend = Set(Some(v)); }
        if let Some(v) = data.status { updated.status = Set(Some(v)); }
        if let Some(v) = data.sort { updated.sort = Set(Some(v)); }
        if let Some(v) = data.sys_dictionary_id { updated.sys_dictionary_id = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: u64) -> Result<(), ServiceError> {
        SysDictionaryDetails::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
