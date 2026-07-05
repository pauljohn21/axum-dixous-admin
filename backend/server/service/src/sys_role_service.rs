use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_role;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use model::prelude::SysRole;
use utils::db_conn;

pub struct SysRoleService;

impl SysRoleService {
    pub async fn insert(data: SysRoleInsertDTO) -> Result<sys_role::Model> {
        let db = db_conn!();
        let active = sys_role::ActiveModel {
            en_name: Set(Some(data.en_name)),
            cn_name: Set(Some(data.cn_name)),
            parent_id: Set(data.parent_id),
            ..Default::default()
        };
        let result = SysRole::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_role::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysRole::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_role::Column::EnName.contains(keyword))
                    .add(sys_role::Column::CnName.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: i32) -> Result<sys_role::Model> {
        SysRole::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("角色不存在"))
    }

    pub async fn update(id: i32, data: SysRoleUpdateDTO) -> Result<sys_role::Model> {
        let db = db_conn!();
        let role: sys_role::ActiveModel = SysRole::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("角色不存在"))?
            .into();
        let mut updated = role;
        if let Some(v) = data.en_name { updated.en_name = Set(Some(v)); }
        if let Some(v) = data.cn_name { updated.cn_name = Set(Some(v)); }
        if let Some(v) = data.parent_id { updated.parent_id = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: i32) -> Result<()> {
        SysRole::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
