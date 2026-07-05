# 动态菜单实现方案

## Context

当前侧边栏菜单是硬编码的 8 个 `<Link to: Route::Xxx>` 项，无法反映数据库中的菜单配置。需要改为从后端获取菜单数据动态渲染，支持树形结构（如"超级管理员"分组下含"角色管理"、"菜单管理"等子菜单），并按用户角色过滤可见菜单。

## 实现步骤

### 第1步：后端 — auth middleware 注入 Username extension

**文件：** `backend/server/auth-layer/src/middleware.rs`

- 新增 `#[derive(Clone, Debug)] pub struct Username(pub String);`
- 在 `call()` 中 JWT 验证成功后，将 `subject` 克隆并插入 request extensions：`req.extensions_mut().insert(Username(subject_clone));`
- 在 `lib.rs` 中 `pub use` 导出 `Username`

### 第2步：后端 — 新增 `get_menus_by_username` service 方法

**文件：** `backend/server/service/src/sys_menu_service.rs`

新增静态方法：
1. 根据 username 查 `sys_user` 获取 user_id
2. 查 `sys_user_role` 获取 role_ids（`user_id: i32, role_id: i32`）
3. 查 `sys_role_menus` 获取 menu_ids（`sys_role_role_id: u64, sys_base_menu_id: u64`）
4. 查 `sys_menu` 获取菜单列表，过滤 `hidden == 0`
5. 补全父菜单：遍历已查菜单的 `parent_id`，将缺失的父菜单也查出来
6. 按 `sort` 排序后返回

### 第3步：后端 — 新增 `GET /api/menu/user` API

**文件：** `backend/server/api/src/menu_api.rs`

- 新增 handler：提取 `Extension<Username>`，调用 `SysMenuService::get_menus_by_username`
- 在 `routes()` 中注册 `.route("/api/menu/user", get(get_user_menus))`

### 第4步：后端 — Casbin 策略添加新路由

**文件：** `backend/server/gateway/src/main.rs`

- `seed_policies` 中 user_policies 添加：`"/api/menu/user"` GET 权限

### 第5步：前端 — 新增 `api::menu::get_user_menus`

**文件：** `web/src/api/menu.rs`

```rust
pub async fn get_user_menus() -> Result<Vec<SysMenu>, String> {
    let resp: R<Vec<SysMenu>> = api_get("/api/menu/user").await?;
    resp.data.ok_or_else(|| resp.message)
}
```

### 第6步：前端 — 新建 `store/menu.rs`

**文件：** `web/src/store/menu.rs`（新建）

定义：
- `MenuItem` struct：id, path, title, icon, sort, children: Vec<MenuItem>
- `build_menu_tree(menus: &[SysMenu]) -> Vec<MenuItem>`：用 `parent_id` 构建树，顶级 parent_id 为 0 或 None
- `MenuState` struct：menus: Signal<Vec<MenuItem>>, loaded: Signal<bool>
- `MenuState::load()`：调用 `get_user_menus()`，过滤 hidden，构建树
- `MenuState::invalidate()`：标记需要重新加载

**文件：** `web/src/store/mod.rs` — 添加 `pub mod menu; pub use menu::MenuState;`

### 第7步：前端 — 添加 `path_to_route` 映射函数

**文件：** `web/src/main.rs`

```rust
pub fn path_to_route(path: &str) -> Option<Route> {
    match path {
        "dashboard" | "/" => Some(Route::Dashboard {}),
        "user" => Some(Route::UserMgmt {}),
        "role" => Some(Route::RoleMgmt {}),
        "menu" => Some(Route::MenuMgmt {}),
        "api" => Some(Route::ApiMgmt {}),
        "dictionary" => Some(Route::DictionaryMgmt {}),
        "operation" => Some(Route::OperationMgmt {}),
        "about" => Some(Route::About {}),
        _ => None,  // 分组菜单(如 "admin")无对应 Route
    }
}
```

同时在 App 组件中注册 `MenuState` context provider。

### 第8步：前端 — 重写 Sidebar 组件

**文件：** `web/src/components/sidebar.rs`

从 context 获取 `MenuState`，遍历菜单树动态渲染：
- 叶子节点 + `path_to_route` 返回 Some → 渲染 `<Link to: route>`
- 叶子节点 + `path_to_route` 返回 None → 不渲染
- 有子菜单的父节点 → 渲染 `<details><summary>...<ul>子菜单</ul></details>`（daisyUI 树形菜单）
- 图标：维护 icon name → emoji 映射表
- 保留折叠/展开功能

### 第9步：前端 — 修改 AdminLayout 加载菜单

**文件：** `web/src/components/layout.rs`

- 获取 `Signal<MenuState>` context
- 认证后用 `use_future` 触发 `menu_state.write().load().await`
- 退出时调用 `menu_state.write().invalidate()`

## 验证方式

1. `cargo check -p gateway` 后端编译通过
2. `cargo check -p web` 前端编译通过
3. 启动后端 + 前端，登录后侧边栏显示数据库中配置的菜单树
4. "超级管理员"分组可展开显示子菜单
5. 切换用户角色后菜单可见项不同
