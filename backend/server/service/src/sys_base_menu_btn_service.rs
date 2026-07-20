use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_base_menu_btns;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_base_menu_btn_dto::{SysBaseMenuBtnInsertDTO, SysBaseMenuBtnUpdateDTO};
use model::prelude::SysBaseMenuBtns;
use utils::prelude::ServiceError;

pub struct SysBaseMenuBtnService;

impl SysBaseMenuBtnService {
    pub async fn insert(db: &DatabaseConnection, data: SysBaseMenuBtnInsertDTO) -> Result<sys_base_menu_btns::Model, ServiceError> {
        let active = sys_base_menu_btns::ActiveModel {
            name: Set(data.name),
            desc: Set(data.desc),
            sys_base_menu_id: Set(data.sys_base_menu_id),
            ..Default::default()
        };
        let result = SysBaseMenuBtns::insert(active).exec(db).await?;
        Self::get_by_id(db, result.last_insert_id).await
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_base_menu_btns::Model>, ServiceError> {
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysBaseMenuBtns::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_base_menu_btns::Column::Name.contains(keyword))
                    .add(sys_base_menu_btns::Column::Desc.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<sys_base_menu_btns::Model, ServiceError> {
        SysBaseMenuBtns::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("菜单按钮不存在".into()))
    }

    pub async fn update(db: &DatabaseConnection, id: u64, data: SysBaseMenuBtnUpdateDTO) -> Result<sys_base_menu_btns::Model, ServiceError> {
        let btn: sys_base_menu_btns::ActiveModel = SysBaseMenuBtns::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("菜单按钮不存在".into()))?
            .into();
        let mut updated = btn;
        if let Some(v) = data.name { updated.name = Set(Some(v)); }
        if let Some(v) = data.desc { updated.desc = Set(Some(v)); }
        if let Some(v) = data.sys_base_menu_id { updated.sys_base_menu_id = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: u64) -> Result<(), ServiceError> {
        SysBaseMenuBtns::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
