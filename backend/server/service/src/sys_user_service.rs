use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait};

use model::dao::sys_user;
use model::dao::sys_user::ActiveModel;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use model::dto::sys_user_role::SysUserRoleAddDto;
use model::prelude::SysUser;
use utils::db_conn;
use utils::prelude::PasswordUtils;

use crate::sys_user_role_service::SysUserRoleService;

pub struct SysUserService;

impl SysUserService {
    pub async fn insert(data: SysUserInsertDTO) -> Result<(), DbErr> {
        let txn = db_conn!().begin().await?;
        let hash = PasswordUtils::encrypt(&data.password);

        let insert = ActiveModel {
            username: Set(Some(data.username)),
            password: Set(Some(hash.password_hash)),
            salt: Set(Some(hash.salt)),
            nick_name: Set(data.nick_name),
            phone: Set(data.phone),
            email: Set(data.email),
            ..Default::default()
        };

        let save = SysUser::insert(insert).exec(&txn).await?;
        let role = SysUserRoleAddDto {
            user_id: save.last_insert_id,
            role_id: data.role_id,
        };
        SysUserRoleService::add_users(&txn, role).await;
        txn.commit().await
    }

    pub async fn login(data: LoginDTO) -> Result<sys_user::Model> {
        let user = SysUser::find()
            .filter(sys_user::Column::Username.eq(data.username.as_str()))
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("无此用户"))?;
        PasswordUtils::verify(&data.password, &user.password.clone().unwrap_or_default(), &user.salt.clone().unwrap_or_default())
            .map_err(|_| anyhow!("密码错误"))?;
        Ok(user)
    }

    pub async fn user_info(username: String) -> Result<sys_user::Model> {
        SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("无此用户"))
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_user::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysUser::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_user::Column::Username.contains(keyword))
                    .add(sys_user::Column::NickName.contains(keyword))
                    .add(sys_user::Column::Phone.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: i32) -> Result<sys_user::Model> {
        SysUser::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))
    }

    pub async fn update(id: i32, data: SysUserUpdateDTO) -> Result<sys_user::Model> {
        let db = db_conn!();
        let user: ActiveModel = SysUser::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?
            .into();
        let mut updated = user;
        if let Some(v) = data.nick_name { updated.nick_name = Set(Some(v)); }
        if let Some(v) = data.phone { updated.phone = Set(Some(v)); }
        if let Some(v) = data.email { updated.email = Set(Some(v)); }
        if let Some(v) = data.header_img { updated.header_img = Set(Some(v)); }
        if let Some(v) = data.side_mode { updated.side_mode = Set(Some(v)); }
        if let Some(v) = data.enable { updated.enable = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    /// 修改密码
    pub async fn change_password(username: &str, old_password: String, new_password: String) -> Result<()> {
        let db = db_conn!();
        let user = SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 验证旧密码
        PasswordUtils::verify(&old_password, &user.password.clone().unwrap_or_default(), &user.salt.clone().unwrap_or_default())
            .map_err(|_| anyhow!("原密码错误"))?;

        // 加密新密码
        let hash = PasswordUtils::encrypt(&new_password);

        let mut active: ActiveModel = user.into();
        active.password = Set(Some(hash.password_hash));
        active.salt = Set(Some(hash.salt));
        active.update(db).await?;
        Ok(())
    }

    /// 删除用户并清理关联数据 (sys_user_role)
    pub async fn delete(id: i32) -> Result<()> {
        let db = db_conn!();
        let txn = db.begin().await?;

        // 清理用户-角色关联
        use model::dao::sys_user_role;
        sys_user_role::Entity::delete_many()
            .filter(sys_user_role::Column::UserId.eq(id))
            .exec(&txn)
            .await?;

        // 删除用户
        SysUser::delete_by_id(id).exec(&txn).await?;
        txn.commit().await?;
        Ok(())
    }

    /// 仪表盘统计数据
    pub async fn dashboard_stats() -> Result<crate::DashboardStats> {
        let db = db_conn!();
        let user_count = SysUser::find().count(db).await?;
        let role_count = model::prelude::SysRole::find().count(db).await?;
        let menu_count = model::prelude::SysMenu::find().count(db).await?;
        let api_count = model::prelude::SysApis::find().count(db).await?;
        Ok(crate::DashboardStats {
            user_count,
            role_count,
            menu_count,
            api_count,
        })
    }
}
