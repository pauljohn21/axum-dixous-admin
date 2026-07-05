# Plan: 前端角色管理系统

## 概述

参考 gin-vue-admin 前端，在 Dioxus 0.7 + daisyUI 5 项目中实现完整的后台管理系统前端，包括：登录、布局框架、用户管理、角色管理（含菜单/API/按钮/数据权限分配）、菜单管理、API管理、字典管理、操作记录。

后端 API 已全部就绪（14 个 API 模块，见下方对照表）。

## 当前状态

- `web/src/` 下仅有 `main.rs`（Dioxus CLI 默认脚手架）
- Cargo.toml 仅依赖 `dioxus 0.7.1`（router feature）
- daisyUI 5 + Tailwind CSS 4 已配置并可用
- 后端运行于 `0.0.0.0:8888`，无 CORS 中间件

## 技术决策

| 决策项 | 选择 | 原因 |
|--------|------|------|
| HTTP 客户端 | `gloo-net 0.6` | WASM 环境主流 HTTP 库 |
| 状态管理 | Dioxus Context + Signal | Dioxus 0.7 内置方案，无需额外依赖 |
| UI 组件 | daisyUI 5 + Tailwind CSS 4 | 项目已集成 |
| 跨域处理 | 后端添加 CORS 中间件 | 比前端代理更可靠，生产环境也需要 |
| Token 存储 | `web-sys` 直接操作 localStorage | 避免引入 gloo-storage 额外依赖 |

## 新增依赖（web/Cargo.toml）

```toml
gloo-net = "0.6"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## 后端变更

在 `gateway/src/main.rs` 添加 CORS 中间件（`tower-http` crate），允许开发环境跨域请求：

```rust
use tower_http::cors::{CorsLayer, Any};
// 在 Router 构建前：
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
let app = app.layer(cors);
```

需要在 `backend/server/Cargo.toml` workspace dependencies 添加 `tower-http = { version = "0.6", features = ["cors"] }`。

## 项目结构

```
web/src/
├── main.rs              # App 根组件 + Route 枚举
├── api/
│   ├── mod.rs           # HTTP 客户端封装（BASE_URL, token, 通用请求函数）
│   ├── auth.rs          # 登录 API
│   ├── user.rs          # 用户 CRUD API
│   ├── role.rs          # 角色 CRUD API
│   ├── menu.rs          # 菜单 CRUD API
│   ├── api_mgmt.rs      # API 管理 CRUD
│   ├── role_menu.rs     # 角色-菜单关联 API
│   ├── role_btn.rs      # 角色-按钮权限 API
│   ├── data_role.rs     # 数据权限 API
│   ├── casbin.rs        # Casbin 策略只读 API
│   ├── menu_btn.rs      # 菜单按钮 API
│   ├── menu_param.rs    # 菜单参数 API
│   ├── jwt.rs           # JWT 黑名单 API
│   ├── dictionary.rs    # 字典 CRUD API
│   ├── dict_detail.rs   # 字典详情 CRUD API
│   └── operation.rs     # 操作记录 API
├── store/
│   ├── mod.rs
│   └── auth.rs          # 认证状态 Context（token, username, is_authenticated）
├── components/
│   ├── mod.rs
│   ├── layout.rs        # AdminLayout（侧边栏+顶栏+Outlet）
│   ├── sidebar.rs       # 侧边导航菜单
│   ├── page_table.rs    # 通用分页表格
│   └── tree_check.rs    # 树形勾选组件
└── pages/
    ├── mod.rs
    ├── login.rs         # 登录页
    ├── dashboard.rs     # 仪表盘
    ├── user.rs          # 用户管理
    ├── role.rs          # 角色管理（含权限配置 tabs）
    ├── menu.rs          # 菜单管理
    ├── api_mgmt.rs      # API 管理
    ├── dictionary.rs    # 字典管理
    └── operation.rs     # 操作记录
```

## 路由设计

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/login")]
    Login {},

    #[layout(AdminLayout)]
    #[route("/")]
    Dashboard {},

    #[route("/user")]
    UserMgmt {},

    #[route("/role")]
    RoleMgmt {},

    #[route("/menu")]
    MenuMgmt {},

    #[route("/api")]
    ApiMgmt {},

    #[route("/dictionary")]
    DictionaryMgmt {},

    #[route("/operation")]
    OperationMgmt {},
}
```

## API 对接完整对照表

### 通用类型

```rust
// 后端统一响应格式
struct R<T> { code: u16, message: String, data: Option<T> }

// 分页请求
struct PageRequest { page: Option<u64>, page_size: Option<u64>, keyword: Option<String> }

// 分页响应
struct PageResponse<T> { list: Vec<T>, total: u64, page: u64, page_size: u64 }
```

### 各模块 API 函数 → 后端端点映射

| 前端函数 | 后端端点 | 方法 | 请求体 | 响应 data 类型 |
|----------|----------|------|--------|---------------|
| **auth.rs** | | | | |
| `login(username, password)` | `/api/user/login` | POST | `LoginDTO` | `LoginResp { token }` |
| **user.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/user/list` | GET | Query | `PageResponse<SysUser>` |
| `get_by_id(id)` | `/api/user/{id}` | GET | - | `SysUser` |
| `create(data)` | `/api/user/register` | POST | `SysUserInsertDTO` | `SysUser` |
| `update(id, data)` | `/api/user/{id}` | PUT | `SysUserUpdateDTO` | `SysUser` |
| `delete(id)` | `/api/user/{id}` | DELETE | - | Value |
| **role.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/role/list` | GET | Query | `PageResponse<SysRole>` |
| `get_by_id(id)` | `/api/role/{id}` | GET | - | `SysRole` |
| `create(data)` | `/api/role` | POST | `SysRoleInsertDTO` | `SysRole` |
| `update(id, data)` | `/api/role/{id}` | PUT | `SysRoleUpdateDTO` | `SysRole` |
| `delete(id)` | `/api/role/{id}` | DELETE | - | Value |
| **menu.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/menu/list` | GET | Query | `PageResponse<SysMenu>` |
| `get_by_id(id)` | `/api/menu/{id}` | GET | - | `SysMenu` |
| `create(data)` | `/api/menu` | POST | `SysMenuInsertDTO` | `SysMenu` |
| `update(id, data)` | `/api/menu/{id}` | PUT | `SysMenuUpdateDTO` | `SysMenu` |
| `delete(id)` | `/api/menu/{id}` | DELETE | - | Value |
| **api_mgmt.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/apis/list` | GET | Query | `PageResponse<SysApis>` |
| `get_by_id(id)` | `/api/apis/{id}` | GET | - | `SysApis` |
| `create(data)` | `/api/apis` | POST | `SysApiInsertDTO` | `SysApis` |
| `update(id, data)` | `/api/apis/{id}` | PUT | `SysApiUpdateDTO` | `SysApis` |
| `delete(id)` | `/api/apis/{id}` | DELETE | - | Value |
| **role_menu.rs** | | | | |
| `list(page, page_size)` | `/api/roleMenu/list` | GET | Query | `PageResponse<SysRoleMenu>` |
| `assign(menu_id, role_id)` | `/api/roleMenu` | POST | `SysRoleMenuInsertDTO` | `SysRoleMenu` |
| `remove(menu_id, role_id)` | `/api/roleMenu/{menu_id}/{role_id}` | DELETE | - | Value |
| **role_btn.rs** | | | | |
| `list(page, page_size)` | `/api/roleBtn/list` | GET | Query | `PageResponse<SysRoleBtn>` |
| `assign(role_id, menu_id, btn_id)` | `/api/roleBtn` | POST | `SysRoleBtnInsertDTO` | `SysRoleBtn` |
| `remove(role_id, menu_id, btn_id)` | `/api/roleBtn/{role_id}/{menu_id}/{btn_id}` | DELETE | - | Value |
| **data_role.rs** | | | | |
| `list(page, page_size)` | `/api/dataRole/list` | GET | Query | `PageResponse<SysDataRole>` |
| `assign(role_id, data_role_id)` | `/api/dataRole` | POST | `SysDataRoleInsertDTO` | `SysDataRole` |
| `remove(role_id, data_role_id)` | `/api/dataRole/{role_id}/{data_role_id}` | DELETE | - | Value |
| **casbin.rs** | | | | |
| `list(page, page_size)` | `/api/casbin/list` | GET | Query | `PageResponse<CasbinRule>` |
| `get_by_id(id)` | `/api/casbin/{id}` | GET | - | `CasbinRule` |
| **menu_btn.rs** | | | | |
| `list(page, page_size)` | `/api/menuBtn/list` | GET | Query | `PageResponse<SysMenuBtn>` |
| `create(data)` | `/api/menuBtn` | POST | `SysBaseMenuBtnInsertDTO` | `SysMenuBtn` |
| `update(id, data)` | `/api/menuBtn/{id}` | PUT | `SysBaseMenuBtnUpdateDTO` | `SysMenuBtn` |
| `delete(id)` | `/api/menuBtn/{id}` | DELETE | - | Value |
| **menu_param.rs** | | | | |
| `list(page, page_size)` | `/api/menuParam/list` | GET | Query | `PageResponse<SysMenuParam>` |
| `create(data)` | `/api/menuParam` | POST | `SysBaseMenuParamInsertDTO` | `SysMenuParam` |
| `update(id, data)` | `/api/menuParam/{id}` | PUT | `SysBaseMenuParamUpdateDTO` | `SysMenuParam` |
| `delete(id)` | `/api/menuParam/{id}` | DELETE | - | Value |
| **dictionary.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/dictionary/list` | GET | Query | `PageResponse<SysDictionary>` |
| `get_by_id(id)` | `/api/dictionary/{id}` | GET | - | `SysDictionary` |
| `create(data)` | `/api/dictionary` | POST | `SysDictionaryInsertDTO` | `SysDictionary` |
| `update(id, data)` | `/api/dictionary/{id}` | PUT | `SysDictionaryUpdateDTO` | `SysDictionary` |
| `delete(id)` | `/api/dictionary/{id}` | DELETE | - | Value |
| **dict_detail.rs** | | | | |
| `list(page, page_size)` | `/api/dictionaryDetail/list` | GET | Query | `PageResponse<SysDictDetail>` |
| `create(data)` | `/api/dictionaryDetail` | POST | `SysDictionaryDetailInsertDTO` | `SysDictDetail` |
| `update(id, data)` | `/api/dictionaryDetail/{id}` | PUT | `SysDictionaryDetailUpdateDTO` | `SysDictDetail` |
| `delete(id)` | `/api/dictionaryDetail/{id}` | DELETE | - | Value |
| **operation.rs** | | | | |
| `list(page, page_size, keyword)` | `/api/operationRecord/list` | GET | Query | `PageResponse<SysOperationRecord>` |
| `delete(id)` | `/api/operationRecord/{id}` | DELETE | - | Value |

## 分阶段实施

### Phase 1：基础设施（登录 + 布局 + API 封装 + CORS）

**目标**：建立前端基础设施，能登录并进入管理后台首页。

1. **后端添加 CORS 中间件**
   - 文件：`backend/server/Cargo.toml` — 添加 `tower-http = { version = "0.6", features = ["cors"] }` 到 workspace deps
   - 文件：`backend/server/gateway/Cargo.toml` — 添加 `tower-http = { workspace = true }`
   - 文件：`backend/server/gateway/src/main.rs` — 在 Router 上 `.layer(cors)` 添加 CORS

2. **更新 web/Cargo.toml** — 添加 `gloo-net`, `serde`, `serde_json`

3. **创建 api/mod.rs** — HTTP 客户端封装
   - `const BASE_URL: &str = "http://localhost:8888"`
   - `get_token() -> Option<String>` — 从 `js_sys::Reflect` 读取 localStorage
   - `set_token(token: &str)` — 写入 localStorage
   - `remove_token()` — 清除 localStorage
   - `api_get<T: DeserializeOwned>(path: &str) -> Result<R<T>>`
   - `api_post<T: DeserializeOwned, B: Serialize>(path: &str, body: &B) -> Result<R<T>>`
   - `api_put<T: DeserializeOwned, B: Serialize>(path: &str, body: &B) -> Result<R<T>>`
   - `api_delete<T: DeserializeOwned>(path: &str) -> Result<R<T>>`
   - 所有函数自动注入 `Authorization: Bearer {token}` header
   - 统一错误处理（401 自动跳转登录）

4. **创建 api/auth.rs** — 登录 API
   - `login(username: String, password: String) -> Result<String>` → POST `/api/user/login`

5. **创建 store/mod.rs + store/auth.rs** — 认证状态
   - `AuthState` struct：`token: Signal<Option<String>>`, `username: Signal<Option<String>>`
   - `use_auth_state()` — 提供 Context
   - `login(username, password)` — 调用 API + set_token + 更新 Signal
   - `logout()` — remove_token + 清空 Signal + 导航到 /login
   - `is_authenticated()` — token 非空

6. **创建 pages/login.rs** — 登录页
   - 用户名/密码输入框（daisyUI `input input-bordered`）
   - 登录按钮（`btn btn-primary`）
   - 错误提示（`alert alert-error`）
   - 登录成功后 `navigator.push(Route::Dashboard {})`

7. **创建 components/layout.rs** — AdminLayout 管理后台布局
   - 左侧侧边栏：品牌名 + 导航菜单（daisyUI `menu`）
   - 顶部导航栏：当前用户名 + 登出按钮（daisyUI `navbar`）
   - 内容区：`Outlet::<Route> {}`
   - 认证守卫：未登录自动重定向到 /login

8. **创建 components/sidebar.rs** — 侧边导航
   - 菜单项：仪表盘、用户管理、角色管理、菜单管理、API管理、字典管理、操作记录
   - 当前路由高亮（`menu-active`）
   - 可折叠侧边栏

9. **创建 pages/dashboard.rs** — 简单仪表盘首页
   - 显示欢迎信息和系统统计卡片

10. **重写 main.rs** — 路由定义 + Auth provider
    - Route 枚举定义
    - App 组件：引入 CSS + data-theme="dark" + use_context_provider(AuthState)
    - Login 路由不在 AdminLayout 内

### Phase 2：用户管理

**目标**：完整的用户 CRUD 页面。

11. **创建 api/user.rs** — 用户 API
    - `list`, `get_by_id`, `create`, `update`, `delete`

12. **创建 pages/user.rs** — 用户管理页
    - 顶部：搜索栏（关键字输入 + 搜索按钮 + 新增按钮）
    - 主体：用户列表表格（daisyUI `table table-zebra`）
      - 列：ID、用户名、昵称、手机号、邮箱、启用状态、操作
      - 操作：编辑、删除
    - 底部：分页控件（上一页/下一页 + 页码显示）
    - 新增/编辑弹窗（daisyUI `modal`）
      - 字段：username, password, role_id, nick_name, phone, email
    - 删除确认弹窗

### Phase 3：角色管理（核心）

**目标**：完整的角色 CRUD + 三种权限分配（菜单/API/数据），参考 gin-vue-admin role.vue。

13. **创建 api/role.rs** — 角色 API
    - `list`, `get_by_id`, `create`, `update`, `delete`

14. **创建 api/role_menu.rs** — 角色-菜单关联 API
    - `list`, `assign`, `remove`

15. **创建 api/role_btn.rs** — 角色-按钮权限 API
    - `list`, `assign`, `remove`

16. **创建 api/data_role.rs** — 数据权限 API
    - `list`, `assign`, `remove`

17. **创建 api/menu.rs** — 菜单 API
    - `list`（用于构建菜单树）

18. **创建 api/casbin.rs** — Casbin 策略 API
    - `list`（用于查看 API 权限）

19. **创建 api/api_mgmt.rs** — API 管理 API
    - `list`（用于构建 API 树）

20. **创建 components/tree_check.rs** — 通用树形勾选组件
    - 递归 RSX 渲染树形结构
    - 每个节点带 checkbox
    - 支持 `on_check` 回调
    - 用于菜单权限树和 API 权限树

21. **创建 pages/role.rs** — 角色管理页（核心页面）
    - **角色列表**：表格展示（ID, en_name, cn_name, parent_id, 操作）
      - 操作按钮：设置权限、新增子角色、编辑、删除
    - **新增/编辑角色弹窗**（daisyUI `modal`）
      - 字段：en_name, cn_name, parent_id（级联选择）
    - **权限配置区域**（使用 daisyUI `tabs` 切换，参考 GVA 的三个 Tab）：
      - **Tab 1: 角色菜单** — 菜单树勾选（`tree_check` 组件）
        - 加载全部菜单树 + 当前角色已分配的菜单
        - 保存：批量 assign/remove 角色-菜单关联
        - 子功能：按钮权限分配（展开菜单节点下的按钮勾选）
      - **Tab 2: 角色API** — API 列表按 api_group 分组展示
        - 加载全部 API + 当前角色的 Casbin 策略
        - 勾选 API → 更新 Casbin 策略
      - **Tab 3: 数据权限** — 角色列表勾选框
        - 全选/本角色/本角色及子角色 快捷按钮
        - 保存：批量 assign/remove 数据权限关联
    - **删除确认弹窗**

### Phase 4：菜单管理

**目标**：菜单树形管理 + 按钮配置。

22. **创建 api/menu_btn.rs** — 菜单按钮 API
23. **创建 api/menu_param.rs** — 菜单参数 API

24. **创建 pages/menu.rs** — 菜单管理页
    - 树形表格展示菜单（层级缩进）
    - 列：ID, 名称, 路径, 组件, 排序, 隐藏, 操作
    - 新增/编辑弹窗：所有菜单字段（path, name, component, title, icon, sort, hidden 等）
    - 可控按钮配置子区域（参考 GVA menu.vue 的 menuBtn 编辑）

### Phase 5：系统管理

**目标**：API管理、字典管理、操作记录页。

25. **创建 pages/api_mgmt.rs** — API 管理页
    - 标准 CRUD 表格（path, method, api_group, description）
    - 新增/编辑/删除

26. **创建 api/dictionary.rs + api/dict_detail.rs** — 字典 API
27. **创建 pages/dictionary.rs** — 字典管理页
    - 左侧字典类型列表，右侧字典详情列表
    - 双栏 CRUD

28. **创建 api/operation.rs** — 操作记录 API
29. **创建 pages/operation.rs** — 操作记录页
    - 只读列表 + 搜索 + 分页 + 删除

### Phase 6：收尾

30. **创建 api/jwt.rs** — JWT 黑名单 API（备用）
31. **创建 components/page_table.rs** — 提取通用分页表格组件（如果重复代码过多）
32. **更新 web/AGENTS.md** — 添加前端页面开发规范

## daisyUI 组件对照表（gin-vue-admin → 本项目）

| Element Plus (GVA) | daisyUI 5 | 用途 |
|---------------------|-----------|------|
| `el-table` | `table table-zebra` + `overflow-x-auto` | 数据列表 |
| `el-drawer` | `modal` (dialog) | 侧滑面板/弹窗 |
| `el-dialog` | `modal` (dialog) | 弹窗 |
| `el-form` | `fieldset` + `label` + `input` | 表单 |
| `el-tree` | 自定义 `tree_check` 组件 | 树形勾选 |
| `el-pagination` | `join` 按钮组 | 分页 |
| `el-button` | `btn` + 变体 | 按钮 |
| `el-input` | `input input-bordered` | 输入框 |
| `el-select` | `select select-bordered` | 下拉选择 |
| `el-checkbox` | `checkbox` | 复选框 |
| `el-tabs` | `tabs tabs-boxed` + `tab` | 标签页切换 |
| `el-message` | `toast` / `alert` | 消息提示 |
| `el-card` | `card` | 卡片 |
| `el-menu` | `menu` | 导航菜单 |
| `el-cascader` | `select` + 递归嵌套 | 级联选择 |
| `el-tag` | `badge` | 标签 |
| `el-switch` | `toggle` | 开关 |

## 前端 DTO 类型定义

所有前端类型与后端 DTO 保持字段名一致（snake_case），使用 `#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]`：

```rust
// api/mod.rs 中定义通用类型
struct R<T> { code: u16, message: String, data: Option<T> }
struct PageRequest { page: Option<u64>, page_size: Option<u64>, keyword: Option<String> }
struct PageResponse<T> { list: Vec<T>, total: u64, page: u64, page_size: u64 }

// 各 API 文件中定义业务 DTO（仅前端需要的字段）
// 注意：后端 offset_datetime 类型在前端用 Option<String> 接收
```

## 关键实现细节

### 1. Token 管理

```rust
// 使用 web_sys 直接操作 localStorage
fn get_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get("token").ok()?
}

fn set_token(token: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            storage.set("token", token).ok();
        }
    }
}
```

### 2. 认证守卫

在 `AdminLayout` 组件中检查 `AuthState`，未认证时用 `navigator.push(Route::Login {})` 重定向。

### 3. 角色权限分配流程

参考 gin-vue-admin 的全量覆盖模式：
- 获取全部资源列表（菜单树/API列表/角色列表）
- 获取当前角色已有权限
- 对比差异，批量调用 assign/remove 接口

### 4. 菜单树构建

后端返回的菜单列表是扁平的（带 parent_id），前端需要构建树形结构：

```rust
fn build_menu_tree(menus: Vec<SysMenu>, parent_id: Option<u64>) -> Vec<MenuNode> {
    menus.iter()
        .filter(|m| m.parent_id == parent_id)
        .map(|m| MenuNode {
            menu: m.clone(),
            children: build_menu_tree(menus.clone(), Some(m.id as u64)),
        })
        .collect()
}
```

### 5. API 权限树构建

参考 gin-vue-admin 的 `buildApiTree`，按 `api_group` 分组：

```rust
fn build_api_tree(apis: Vec<SysApis>) -> Vec<ApiGroup> {
    // 按 api_group 分组
    // 每个组作为父节点，组内 API 作为子节点
}
```

## 验证步骤

1. `cargo check -p web` — 前端编译通过
2. `cargo check -p gateway` — 后端编译通过（含 CORS 变更）
3. 启动后端 `cargo run -p gateway`
4. 启动前端 `npm run tailwind` + `dx serve`
5. 登录页 → 输入凭据 → 跳转 Dashboard
6. 角色管理 → CRUD + 权限分配（三个 Tab）
7. 用户管理 → CRUD
8. 菜单管理 → 树形展示 + CRUD + 按钮配置
9. API 管理 → CRUD
10. 字典管理 → 双栏 CRUD
11. 操作记录 → 列表查询
