use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_base_menu_parameters;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_base_menu_param_dto::{SysBaseMenuParamInsertDTO, SysBaseMenuParamUpdateDTO};
use model::prelude::SysBaseMenuParameters;
use utils::db_conn;

pub struct SysBaseMenuParamService;

impl SysBaseMenuParamService {
    pub async fn insert(data: SysBaseMenuParamInsertDTO) -> Result<sys_base_menu_parameters::Model> {
        let db = db_conn!();
        let active = sys_base_menu_parameters::ActiveModel {
            sys_base_menu_id: Set(data.sys_base_menu_id),
            r#type: Set(data.r#type),
            key: Set(data.key),
            value: Set(data.value),
            ..Default::default()
        };
        let result = SysBaseMenuParameters::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_base_menu_parameters::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysBaseMenuParameters::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_base_menu_parameters::Column::Key.contains(keyword))
                    .add(sys_base_menu_parameters::Column::Value.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<sys_base_menu_parameters::Model> {
        SysBaseMenuParameters::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("菜单参数不存在"))
    }

    pub async fn update(id: u64, data: SysBaseMenuParamUpdateDTO) -> Result<sys_base_menu_parameters::Model> {
        let db = db_conn!();
        let param: sys_base_menu_parameters::ActiveModel = SysBaseMenuParameters::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("菜单参数不存在"))?
            .into();
        let mut updated = param;
        if let Some(v) = data.sys_base_menu_id { updated.sys_base_menu_id = Set(Some(v)); }
        if let Some(v) = data.r#type { updated.r#type = Set(Some(v)); }
        if let Some(v) = data.key { updated.key = Set(Some(v)); }
        if let Some(v) = data.value { updated.value = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(id: u64) -> Result<()> {
        SysBaseMenuParameters::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }
}
