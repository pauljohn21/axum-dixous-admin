use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::jwt_blacklists;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::jwt_blacklist_dto::{JwtBlacklistInsertDTO, JwtBlacklistUpdateDTO};
use model::prelude::JwtBlacklists;
use utils::db_conn;

pub struct JwtBlacklistService;

impl JwtBlacklistService {
    pub async fn insert(data: JwtBlacklistInsertDTO) -> Result<jwt_blacklists::Model> {
        let db = db_conn!();
        let active = jwt_blacklists::ActiveModel {
            jwt: Set(data.jwt),
            ..Default::default()
        };
        let result = JwtBlacklists::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<jwt_blacklists::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = JwtBlacklists::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(jwt_blacklists::Column::Jwt.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<jwt_blacklists::Model> {
        JwtBlacklists::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("JWT黑名单记录不存在"))
    }

    pub async fn update(id: u64, data: JwtBlacklistUpdateDTO) -> Result<jwt_blacklists::Model> {
        let db = db_conn!();
        let record: jwt_blacklists::ActiveModel = JwtBlacklists::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("JWT黑名单记录不存在"))?
            .into();
        let mut updated = record;
        if let Some(v) = data.jwt { updated.jwt = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: u64) -> Result<()> {
        JwtBlacklists::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
