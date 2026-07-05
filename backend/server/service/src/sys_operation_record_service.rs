use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_operation_records;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_operation_record_dto::{SysOperationRecordInsertDTO, SysOperationRecordUpdateDTO};
use model::prelude::SysOperationRecords;
use utils::db_conn;

pub struct SysOperationRecordService;

impl SysOperationRecordService {
    pub async fn insert(data: SysOperationRecordInsertDTO) -> Result<sys_operation_records::Model> {
        let db = db_conn!();
        let active = sys_operation_records::ActiveModel {
            ip: Set(data.ip),
            method: Set(data.method),
            path: Set(data.path),
            status: Set(data.status),
            latency: Set(data.latency),
            agent: Set(data.agent),
            error_message: Set(data.error_message),
            body: Set(data.body),
            resp: Set(data.resp),
            user_id: Set(data.user_id),
            ..Default::default()
        };
        let result = SysOperationRecords::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_operation_records::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysOperationRecords::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_operation_records::Column::Ip.contains(keyword))
                    .add(sys_operation_records::Column::Path.contains(keyword))
                    .add(sys_operation_records::Column::Method.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<sys_operation_records::Model> {
        SysOperationRecords::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("操作记录不存在"))
    }

    pub async fn update(id: u64, data: SysOperationRecordUpdateDTO) -> Result<sys_operation_records::Model> {
        let db = db_conn!();
        let record: sys_operation_records::ActiveModel = SysOperationRecords::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("操作记录不存在"))?
            .into();
        let mut updated = record;
        if let Some(v) = data.ip { updated.ip = Set(Some(v)); }
        if let Some(v) = data.method { updated.method = Set(Some(v)); }
        if let Some(v) = data.path { updated.path = Set(Some(v)); }
        if let Some(v) = data.status { updated.status = Set(Some(v)); }
        if let Some(v) = data.latency { updated.latency = Set(Some(v)); }
        if let Some(v) = data.agent { updated.agent = Set(Some(v)); }
        if let Some(v) = data.error_message { updated.error_message = Set(Some(v)); }
        if let Some(v) = data.body { updated.body = Set(Some(v)); }
        if let Some(v) = data.resp { updated.resp = Set(Some(v)); }
        if let Some(v) = data.user_id { updated.user_id = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: u64) -> Result<()> {
        SysOperationRecords::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
