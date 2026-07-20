use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait};

use model::dao::sys_role;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_role_dto::{SysRoleInsertDTO, SysRoleUpdateDTO};
use model::prelude::SysRole;
use utils::prelude::ServiceError;

pub struct SysRoleService;

impl SysRoleService {
    pub async fn insert(db: &DatabaseConnection, data: SysRoleInsertDTO) -> Result<sys_role::Model, ServiceError> {
        let active = sys_role::ActiveModel {
            en_name: Set(Some(data.en_name)),
            cn_name: Set(Some(data.cn_name)),
            parent_id: Set(data.parent_id),
            ..Default::default()
        };
        let result = SysRole::insert(active).exec(db).await?;
        Self::get_by_id(db, result.last_insert_id).await
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_role::Model>, ServiceError> {
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

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<sys_role::Model, ServiceError> {
        SysRole::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("角色不存在".into()))
    }

    pub async fn update(db: &DatabaseConnection, id: i32, data: SysRoleUpdateDTO) -> Result<sys_role::Model, ServiceError> {
        let role: sys_role::ActiveModel = SysRole::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("角色不存在".into()))?
            .into();
        let mut updated = role;
        if let Some(v) = data.en_name { updated.en_name = Set(Some(v)); }
        if let Some(v) = data.cn_name { updated.cn_name = Set(Some(v)); }
        if let Some(v) = data.parent_id { updated.parent_id = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
        let txn = db.begin().await?;

        // 清理角色-菜单关联
        use model::dao::sys_role_menus;
        sys_role_menus::Entity::delete_many()
            .filter(sys_role_menus::Column::SysRoleRoleId.eq(id as u64))
            .exec(&txn)
            .await?;

        // 清理角色-按钮关联
        use model::dao::sys_role_btns;
        sys_role_btns::Entity::delete_many()
            .filter(sys_role_btns::Column::RoleId.eq(id as u64))
            .exec(&txn)
            .await?;

        // 清理用户-角色关联
        use model::dao::sys_user_role;
        sys_user_role::Entity::delete_many()
            .filter(sys_user_role::Column::RoleId.eq(id))
            .exec(&txn)
            .await?;

        // 清理 Casbin 策略中该角色的权限
        use model::dao::casbin_rule;
        casbin_rule::Entity::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(id.to_string()))
            .exec(&txn)
            .await?;

        // 删除角色
        SysRole::delete_by_id(id).exec(&txn).await?;
        txn.commit().await?;

        // 刷新 Casbin 缓存
        crate::enforcer::reload_policy().await;
        Ok(())
    }
}
