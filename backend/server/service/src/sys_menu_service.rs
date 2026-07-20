use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait};
use redis::aio::ConnectionManager;
use std::collections::{HashMap, HashSet};

use model::dao::sys_menu;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use model::prelude::SysMenu;
use utils::cache::keys;
use utils::prelude::{Cache, ServiceError};

pub struct SysMenuService;

impl SysMenuService {
    pub async fn insert(db: &DatabaseConnection, data: SysMenuInsertDTO) -> Result<sys_menu::Model, ServiceError> {
        let active = sys_menu::ActiveModel {
            menu_level: Set(data.menu_level),
            parent_id: Set(data.parent_id),
            path: Set(data.path),
            name: Set(data.name),
            hidden: Set(data.hidden),
            component: Set(data.component),
            sort: Set(data.sort),
            active_name: Set(data.active_name),
            keep_alive: Set(data.keep_alive),
            default_menu: Set(data.default_menu),
            title: Set(data.title),
            icon: Set(data.icon),
            close_tab: Set(data.close_tab),
            ..Default::default()
        };
        let result = SysMenu::insert(active).exec(db).await?;
        Self::get_by_id(db, result.last_insert_id).await
    }

    pub async fn list(db: &DatabaseConnection, query: PageRequest) -> Result<PageResponse<sys_menu::Model>, ServiceError> {
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysMenu::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_menu::Column::Title.contains(keyword))
                    .add(sys_menu::Column::Path.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<sys_menu::Model, ServiceError> {
        SysMenu::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("菜单不存在".into()))
    }

    pub async fn update(db: &DatabaseConnection, id: i32, data: SysMenuUpdateDTO) -> Result<sys_menu::Model, ServiceError> {
        let menu: sys_menu::ActiveModel = SysMenu::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ServiceError::NotFound("菜单不存在".into()))?
            .into();
        let mut updated = menu;
        if let Some(v) = data.menu_level { updated.menu_level = Set(Some(v)); }
        if let Some(v) = data.parent_id { updated.parent_id = Set(Some(v)); }
        if let Some(v) = data.path { updated.path = Set(Some(v)); }
        if let Some(v) = data.name { updated.name = Set(Some(v)); }
        if let Some(v) = data.hidden { updated.hidden = Set(Some(v)); }
        if let Some(v) = data.component { updated.component = Set(Some(v)); }
        if let Some(v) = data.sort { updated.sort = Set(Some(v)); }
        if let Some(v) = data.title { updated.title = Set(Some(v)); }
        if let Some(v) = data.icon { updated.icon = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
        let txn = db.begin().await?;

        // 清理角色-菜单关联
        use model::dao::sys_role_menus;
        sys_role_menus::Entity::delete_many()
            .filter(sys_role_menus::Column::SysBaseMenuId.eq(id as u64))
            .exec(&txn)
            .await?;

        // 清理菜单按钮
        use model::dao::sys_base_menu_btns;
        sys_base_menu_btns::Entity::delete_many()
            .filter(sys_base_menu_btns::Column::SysBaseMenuId.eq(id as u64))
            .exec(&txn)
            .await?;

        // 清理菜单参数
        use model::dao::sys_base_menu_parameters;
        sys_base_menu_parameters::Entity::delete_many()
            .filter(sys_base_menu_parameters::Column::SysBaseMenuId.eq(id as u64))
            .exec(&txn)
            .await?;

        // 删除菜单
        SysMenu::delete_by_id(id).exec(&txn).await?;
        txn.commit().await?;
        Ok(())
    }

    /// 根据用户名查询菜单列表（带 Redis 缓存）
    /// admin 用户返回所有可见菜单，其他用户通过角色关联查询
    pub async fn get_menus_with_cache(
        db: &DatabaseConnection,
        redis: &mut ConnectionManager,
        username: &str,
    ) -> Result<Vec<sys_menu::Model>, ServiceError> {
        let cache_key = format!("{}{}", keys::USER_MENUS_PREFIX, username);

        // 1. 查缓存
        if let Some(cached) = Cache::get::<Vec<sys_menu::Model>>(redis, &cache_key).await {
            return Ok(cached);
        }

        // 2. Miss → 查 DB
        let menus = Self::get_menus_by_username(db, username).await?;

        // 3. 回填缓存
        Cache::set(redis, &cache_key, &menus, keys::USER_MENUS_TTL).await;

        Ok(menus)
    }

    /// 根据用户名查询菜单列表（无缓存，直接查 DB）
    /// admin 用户返回所有可见菜单，其他用户通过角色关联查询
    ///
    /// 查询优化（Phase 4）：
    /// - 消除冗余 `.one()` 查询，直接 `.all()` 获取全部角色关联
    /// - 补全父菜单改为内存操作（从全量菜单中查找），无 DB 往返
    pub async fn get_menus_by_username(db: &DatabaseConnection, username: &str) -> Result<Vec<sys_menu::Model>, ServiceError> {
        // admin 用户直接返回所有可见菜单
        if username == "admin" {
            let mut menus: Vec<sys_menu::Model> = SysMenu::find()
                .filter(sys_menu::Column::Hidden.eq(0))
                .all(db)
                .await?;
            menus.sort_by_key(|m| m.sort.unwrap_or(0));
            return Ok(menus);
        }

        // 非 admin 用户：通过角色-菜单关联查询（3 次 DB 查询）
        use model::dao::sys_role_menus;
        use model::prelude::{SysUserRole, SysRoleMenus};

        // 1. 通过 username 直接查用户所有角色关联（join sys_user，.all() 一次获取）
        let user_roles = SysUserRole::find()
            .inner_join(model::dao::sys_user::Entity)
            .filter(model::dao::sys_user::Column::Username.eq(username))
            .all(db)
            .await?;

        let role_ids: Vec<u64> = user_roles.iter().map(|ur| ur.role_id as u64).collect();
        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        // 2. 查角色菜单关联
        let role_menus = SysRoleMenus::find()
            .filter(sys_role_menus::Column::SysRoleRoleId.is_in(role_ids))
            .all(db)
            .await?;
        let menu_ids: HashSet<i32> = role_menus.iter().map(|rm| rm.sys_base_menu_id as i32).collect();
        if menu_ids.is_empty() {
            return Ok(vec![]);
        }

        // 3. 一次查全量可见菜单（hidden=0），内存过滤 + 补全父菜单
        let all_menus: Vec<sys_menu::Model> = SysMenu::find()
            .filter(sys_menu::Column::Hidden.eq(0))
            .all(db)
            .await?;
        let all_menu_map: HashMap<i32, &sys_menu::Model> = all_menus.iter().map(|m| (m.id, m)).collect();

        // 4. 内存过滤：选出用户有权限的菜单
        let mut result: Vec<sys_menu::Model> = all_menus.iter()
            .filter(|m| menu_ids.contains(&m.id))
            .cloned()
            .collect();

        // 5. 内存补全父菜单（无 DB 查询）
        let mut added: HashSet<i32> = result.iter().map(|m| m.id).collect();
        for m in &result.clone() {
            let mut parent_id = m.parent_id.map(|pid| pid as i32);
            while let Some(pid) = parent_id {
                if pid == 0 || added.contains(&pid) {
                    break;
                }
                if let Some(parent) = all_menu_map.get(&pid) {
                    result.push((*parent).clone());
                    added.insert(pid);
                    parent_id = parent.parent_id.map(|p| p as i32);
                } else {
                    break;
                }
            }
        }

        result.sort_by_key(|m| m.sort.unwrap_or(0));
        Ok(result)
    }
}
