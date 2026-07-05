use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_menu;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_menu_dto::{SysMenuInsertDTO, SysMenuUpdateDTO};
use model::prelude::SysMenu;
use utils::db_conn;

pub struct SysMenuService;

impl SysMenuService {
    pub async fn insert(data: SysMenuInsertDTO) -> Result<sys_menu::Model> {
        let db = db_conn!();
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
        Self::get_by_id(result.last_insert_id).await
    }

    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_menu::Model>> {
        let db = db_conn!();
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

    pub async fn get_by_id(id: i32) -> Result<sys_menu::Model> {
        SysMenu::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("菜单不存在"))
    }

    pub async fn update(id: i32, data: SysMenuUpdateDTO) -> Result<sys_menu::Model> {
        let db = db_conn!();
        let menu: sys_menu::ActiveModel = SysMenu::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("菜单不存在"))?
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

    pub async fn delete(id: i32) -> Result<()> {
        SysMenu::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }

    /// 根据用户名查询菜单列表
    /// admin 用户返回所有可见菜单，其他用户通过角色关联查询
    pub async fn get_menus_by_username(username: &str) -> Result<Vec<sys_menu::Model>> {
        let db = db_conn!();

        // admin 用户直接返回所有可见菜单
        if username == "admin" {
            let mut menus: Vec<sys_menu::Model> = SysMenu::find()
                .filter(sys_menu::Column::Hidden.eq(0))
                .all(db)
                .await?;
            menus.sort_by_key(|m| m.sort.unwrap_or(0));
            return Ok(menus);
        }

        // 非 admin 用户：通过角色-菜单关联查询
        use model::dao::sys_user_role;
        use model::dao::sys_role_menus;
        use model::prelude::{SysUserRole, SysRoleMenus};
        use std::collections::HashSet;

        // 1. 查用户角色
        let user = SysUserRole::find()
            .inner_join(model::dao::sys_user::Entity)
            .filter(model::dao::sys_user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        let user_roles = SysUserRole::find()
            .filter(sys_user_role::Column::UserId.eq(user.user_id))
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

        // 3. 查菜单
        let mut menus: Vec<sys_menu::Model> = SysMenu::find()
            .filter(sys_menu::Column::Id.is_in(menu_ids.iter().cloned()))
            .filter(sys_menu::Column::Hidden.eq(0))
            .all(db)
            .await?;

        // 4. 补全父菜单
        let mut all_ids: HashSet<i32> = menus.iter().map(|m| m.id).collect();
        let mut needs_parent: Vec<i32> = menus.iter()
            .filter_map(|m| m.parent_id.filter(|&pid| pid != 0 && !all_ids.contains(&(pid as i32))).map(|pid| pid as i32))
            .collect();

        while !needs_parent.is_empty() {
            let parents: Vec<sys_menu::Model> = SysMenu::find()
                .filter(sys_menu::Column::Id.is_in(needs_parent.iter().cloned()))
                .all(db)
                .await?;
            needs_parent = parents.iter()
                .filter_map(|m| m.parent_id.filter(|&pid| pid != 0 && !all_ids.contains(&(pid as i32))).map(|pid| pid as i32))
                .collect();
            for p in &parents { all_ids.insert(p.id); }
            menus.extend(parents);
        }

        menus.sort_by_key(|m| m.sort.unwrap_or(0));
        Ok(menus)
    }
}
