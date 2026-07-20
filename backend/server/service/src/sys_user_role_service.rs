use sea_orm::{EntityTrait, Set};
use sea_orm::DatabaseTransaction;

use model::dao::sys_user_role;
use model::dto::sys_user_role::SysUserRoleAddDto;
use model::prelude::SysUserRole;
use utils::prelude::ServiceError;

#[derive(Debug)]
pub struct SysUserRoleService;

impl SysUserRoleService {
    pub async fn add_users(db: &DatabaseTransaction, data: SysUserRoleAddDto) -> Result<(), ServiceError> {
        let user = sys_user_role::ActiveModel {
            user_id: Set(data.user_id),
            role_id: Set(data.role_id),
        };
        SysUserRole::insert(user).exec(db).await?;
        Ok(())
    }
}
