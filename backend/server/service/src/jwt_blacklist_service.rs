use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::jwt_blacklists;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::jwt_blacklist_dto::{JwtBlacklistInsertDTO, JwtBlacklistUpdateDTO};
use model::prelude::JwtBlacklists;
use utils::prelude::ServiceError;

pub struct JwtBlacklistService;

impl JwtBlacklistService {
    pub async fn insert(db: &DatabaseConnection, data: JwtBlacklistInsertDTO) -> Result<jwt_blacklists::Model, ServiceError> {
        let active = jwt_blacklists::ActiveModel {
            jwt: Set(data.jwt),
            ..Default::default()
        };
        let result = JwtBlacklists::insert(active).exec(db).await?;
        Self::get_by_id(db, result.last_insert_id).await
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<jwt_blacklists::Model>, ServiceError> {
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

    pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<jwt_blacklists::Model, ServiceError> {
        JwtBlacklists::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("JWT黑名单记录不存在".into()))
    }

    pub async fn update(db: &DatabaseConnection, id: u64, data: JwtBlacklistUpdateDTO) -> Result<jwt_blacklists::Model, ServiceError> {
        let record: jwt_blacklists::ActiveModel = JwtBlacklists::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("JWT黑名单记录不存在".into()))?
            .into();
        let mut updated = record;
        if let Some(v) = data.jwt { updated.jwt = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: u64) -> Result<(), ServiceError> {
        JwtBlacklists::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
