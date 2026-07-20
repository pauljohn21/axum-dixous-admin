use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_data_role_id;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_data_role_dto::SysDataRoleInsertDTO;
use model::prelude::SysDataRoleId;
use utils::prelude::ServiceError;

pub struct SysDataRoleService;

impl SysDataRoleService {
    pub async fn insert(db: &DatabaseConnection, data: SysDataRoleInsertDTO) -> Result<sys_data_role_id::Model, ServiceError> {
        let active = sys_data_role_id::ActiveModel {
            sys_role_role_id: Set(data.sys_role_role_id),
            data_role_id_role_id: Set(data.data_role_id_role_id),
        };
        SysDataRoleId::insert(active).exec(db).await?;
        // For composite PK, find the inserted record
        SysDataRoleId::find()
            .filter(sys_data_role_id::Column::SysRoleRoleId.eq(data.sys_role_role_id))
            .filter(sys_data_role_id::Column::DataRoleIdRoleId.eq(data.data_role_id_role_id))
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("创建失败".into()))
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_data_role_id::Model>, ServiceError> {
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let total = SysDataRoleId::find().count(db).await?;
        let list = SysDataRoleId::find()
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_composite_id(db: &DatabaseConnection, sys_role_role_id: u64, data_role_id_role_id: u64) -> Result<sys_data_role_id::Model, ServiceError> {
        SysDataRoleId::find()
            .filter(sys_data_role_id::Column::SysRoleRoleId.eq(sys_role_role_id))
            .filter(sys_data_role_id::Column::DataRoleIdRoleId.eq(data_role_id_role_id))
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("数据角色关联不存在".into()))
    }

    pub async fn delete(db: &DatabaseConnection, sys_role_role_id: u64, data_role_id_role_id: u64) -> Result<(), ServiceError> {
        SysDataRoleId::delete_many()
            .filter(sys_data_role_id::Column::SysRoleRoleId.eq(sys_role_role_id))
            .filter(sys_data_role_id::Column::DataRoleIdRoleId.eq(data_role_id_role_id))
            .exec(db)
            .await?;
        Ok(())
    }
}
