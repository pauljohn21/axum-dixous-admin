use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait};
use serde::Deserialize;
use tracing::error;

use model::dao::sys_user;
use model::dao::sys_user::ActiveModel;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_user_dto::{LoginDTO, SysUserInsertDTO, SysUserUpdateDTO};
use model::dto::sys_user_role::SysUserRoleAddDto;
use model::prelude::SysUser;
use utils::prelude::{PasswordUtils, ServiceError, WechatConfig};

use crate::sys_user_role_service::SysUserRoleService;

pub struct SysUserService;

/// 用户名校验: 3-20 字符，仅字母数字下划线
fn validate_username(username: &str) -> Result<(), ServiceError> {
    if username.len() < 3 || username.len() > 20 {
        return Err(ServiceError::BadRequest("用户名长度需 3-20 字符".into()));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ServiceError::BadRequest("用户名仅允许字母数字下划线".into()));
    }
    Ok(())
}

/// 密码校验: 最少 6 位
fn validate_password(password: &str) -> Result<(), ServiceError> {
    if password.len() < 6 {
        return Err(ServiceError::BadRequest("密码至少 6 位".into()));
    }
    Ok(())
}

/// 邮箱校验: 基本格式（含 @）
fn validate_email(email: &str) -> Result<(), ServiceError> {
    if !email.is_empty() && (!email.contains('@') || !email.contains('.')) {
        return Err(ServiceError::BadRequest("邮箱格式不正确".into()));
    }
    Ok(())
}

/// 手机号校验: 11 位数字（中国手机号）
fn validate_phone(phone: &str) -> Result<(), ServiceError> {
    if !phone.is_empty() && (phone.len() != 11 || !phone.chars().all(|c| c.is_ascii_digit())) {
        return Err(ServiceError::BadRequest("手机号格式不正确".into()));
    }
    Ok(())
}

impl SysUserService {
    pub async fn insert(db: &DatabaseConnection, data: SysUserInsertDTO) -> Result<(), ServiceError> {
        // 输入校验
        validate_username(&data.username)?;
        validate_password(&data.password)?;
        if let Some(ref email) = data.email {
            validate_email(email)?;
        }
        if let Some(ref phone) = data.phone {
            validate_phone(phone)?;
        }

        let txn = db.begin().await?;
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
        // 仅当指定了 role_id 时才创建用户-角色关联
        if let Some(role_id) = data.role_id {
            let role = SysUserRoleAddDto {
                user_id: save.last_insert_id,
                role_id,
            };
            SysUserRoleService::add_users(&txn, role).await?;
        }
        txn.commit().await?;
        Ok(())
    }

    pub async fn login(db: &DatabaseConnection, data: LoginDTO) -> Result<sys_user::Model, ServiceError> {
        // 输入校验
        validate_username(&data.username)?;
        validate_password(&data.password)?;

        let user = SysUser::find()
            .filter(sys_user::Column::Username.eq(data.username.as_str()))
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)?;
        PasswordUtils::verify(&data.password, &user.password.clone().unwrap_or_default(), &user.salt.clone().unwrap_or_default())
            .map_err(|_| ServiceError::InvalidPassword)?;
        Ok(user)
    }

    pub async fn user_info(db: &DatabaseConnection, username: String) -> Result<sys_user::Model, ServiceError> {
        SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_user::Model>, ServiceError> {
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

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<sys_user::Model, ServiceError> {
        SysUser::find_by_id(id)
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, data: SysUserUpdateDTO) -> Result<sys_user::Model, ServiceError> {
        let user: ActiveModel = SysUser::find_by_id(id)
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)?
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
    pub async fn change_password(db: &DatabaseConnection, username: &str, old_password: String, new_password: String) -> Result<(), ServiceError> {
        let user = SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)?;

        // 验证旧密码
        PasswordUtils::verify(&old_password, &user.password.clone().unwrap_or_default(), &user.salt.clone().unwrap_or_default())
            .map_err(|_| ServiceError::InvalidPassword)?;

        // 加密新密码
        let hash = PasswordUtils::encrypt(&new_password);

        let mut active: ActiveModel = user.into();
        active.password = Set(Some(hash.password_hash));
        active.salt = Set(Some(hash.salt));
        active.update(db).await?;
        Ok(())
    }

    /// 删除用户并清理关联数据 (sys_user_role)
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
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

    /// 微信登录 — 通过 wx.login 的 code 换取 openid，查找或自动注册用户
    pub async fn wx_login(
        db: &DatabaseConnection,
        http_client: &reqwest::Client,
        wechat: &WechatConfig,
        code: &str,
    ) -> Result<sys_user::Model, ServiceError> {
        // 1. 调用微信 code2Session 接口获取 openid
        let openid = Self::code2session(http_client, wechat, code).await?;

        // 2. 根据 openid 查找用户
        if let Some(user) = SysUser::find()
            .filter(sys_user::Column::WxOpenid.eq(&openid))
            .one(db)
            .await?
        {
            // 已绑定过微信，直接返回
            return Ok(user);
        }

        // 3. 未绑定 — 自动注册新用户
        let username = format!("wx_{}", &openid[..openid.len().min(10)]);
        let random_password = utils::prelude::rand_utils(16);
        let hash = PasswordUtils::encrypt(&random_password);

        let new_user = ActiveModel {
            username: Set(Some(username)),
            password: Set(Some(hash.password_hash)),
            salt: Set(Some(hash.salt)),
            wx_openid: Set(Some(openid)),
            nick_name: Set(Some("微信用户".to_string())),
            enable: Set(Some(1)),
            ..Default::default()
        };

        let saved = SysUser::insert(new_user).exec(db).await?;
        SysUser::find_by_id(saved.last_insert_id)
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)
    }

    /// 微信绑定 — 将当前登录用户绑定到微信 openid
    /// 如果该 openid 已被其他用户绑定，则返回错误
    pub async fn wx_bind(
        db: &DatabaseConnection,
        http_client: &reqwest::Client,
        wechat: &WechatConfig,
        username: &str,
        code: &str,
    ) -> Result<(), ServiceError> {
        let openid = Self::code2session(http_client, wechat, code).await?;

        // 检查 openid 是否已被其他用户绑定
        if let Some(existing) = SysUser::find()
            .filter(sys_user::Column::WxOpenid.eq(&openid))
            .one(db)
            .await?
        {
            if existing.username.as_deref() != Some(username) {
                return Err(ServiceError::WechatAlreadyBound);
            }
            // 已经绑定的是当前用户，无需重复操作
            return Ok(());
        }

        // 查找当前用户并绑定 openid
        let user = SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(ServiceError::UserNotFound)?;

        let mut active: ActiveModel = user.into();
        active.wx_openid = Set(Some(openid));
        active.update(db).await?;
        Ok(())
    }

    /// 调用微信 code2Session 接口 — 用 code 换取 openid + session_key
    async fn code2session(
        http_client: &reqwest::Client,
        wechat: &WechatConfig,
        code: &str,
    ) -> Result<String, ServiceError> {
        /// 微信 code2Session 响应体
        #[derive(Deserialize)]
        struct WxSessionResp {
            openid: Option<String>,
            #[serde(default)]
            errcode: i32,
            #[serde(default)]
            errmsg: String,
        }

        let url = format!(
            "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
            wechat.appid,
            wechat.secret,
            code
        );

        let resp: WxSessionResp = http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| ServiceError::WechatApi(e.to_string()))?
            .json()
            .await
            .map_err(|e| ServiceError::WechatApi(e.to_string()))?;

        if resp.errcode != 0 {
            error!("微信 code2Session 错误: {} - {}", resp.errcode, resp.errmsg);
            return Err(ServiceError::WechatApi(resp.errmsg));
        }

        resp.openid.ok_or_else(|| ServiceError::WechatApi("openid 为空".into()))
    }

    /// 仪表盘统计数据 — 并行查询 4 个 count
    pub async fn dashboard_stats(db: &DatabaseConnection) -> Result<crate::DashboardStats, ServiceError> {
        let (user_count, role_count, menu_count, api_count) = tokio::try_join!(
            SysUser::find().count(db),
            model::prelude::SysRole::find().count(db),
            model::prelude::SysMenu::find().count(db),
            model::prelude::SysApis::find().count(db),
        )?;
        Ok(crate::DashboardStats {
            user_count,
            role_count,
            menu_count,
            api_count,
        })
    }
}
