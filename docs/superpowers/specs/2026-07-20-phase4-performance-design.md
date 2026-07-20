# Phase 4 — 性能优化详细设计

> 日期：2026-07-20
> 状态：📋 设计完成，待实施
> 前置条件：Phase 1（代码质量）、Phase 2（测试体系）、Phase 3（开发效率）已完成

## 1. 现状分析

### 1.1 后端查询性能问题

#### 问题 1：菜单查询 N+1（`get_menus_by_username`）

非 admin 用户获取菜单需要 **4+ 次串行 DB 查询**：

```
1. inner_join sys_user_role + sys_user, filter username, .one()  → 获取 user_id（冗余）
2. filter sys_user_role.user_id = user_id, .all()                 → 获取所有角色关联
3. filter sys_role_menus.sys_role_role_id IN(role_ids), .all()   → 获取角色菜单关联
4. filter sys_menu.id IN(menu_ids), .all()                        → 获取菜单
5. while 循环补全父菜单                                            → 可能多次查询
```

**问题分析：**
- 第 1 步用 `inner_join` + `.one()` 只为获取 `user_id`，但 `sys_user_role` 表本身就有 `user_id`，可以直接 join `sys_user` 表用 username 过滤，一次查出所有角色关联
- while 循环补全父菜单在极端情况下（深层嵌套菜单）会导致多次 DB 往返

#### 问题 2：Dashboard 统计串行查询

`dashboard_stats` 执行 4 个独立的 count 查询，**完全串行**：

```rust
let user_count = SysUser::find().count(db).await?;       // ~5ms
let role_count = SysRole::find().count(db).await?;        // ~5ms
let menu_count = SysMenu::find().count(db).await?;        // ~5ms
let api_count = SysApis::find().count(db).await?;         // ~5ms
// 总计 ~20ms（串行）
```

可以用 `tokio::try_join!` 并行化，总计 ~5ms。

### 1.2 后端缓存现状

| 缓存对象 | 当前状态 | 问题 |
|----------|----------|------|
| JWT 黑名单 | ✅ Redis SETEX | 已实现 |
| 用户菜单 | ❌ 每次查 DB | 菜单数据变化不频繁，每次请求都查 DB 浪费 |
| 字典数据 | ❌ 每次查 DB | 字典数据极少变化，每次请求都查 DB |
| 用户信息 | ❌ 每次查 DB | 登录后 user_info 每次请求都查 DB |

### 1.3 前端 WASM 体积现状

| 配置项 | 状态 |
|--------|------|
| `opt-level = "z"` | ✅ 已配置 |
| `lto = true` | ✅ 已配置 |
| `codegen-units = 1` | ✅ 已配置 |
| `panic = "abort"` | ✅ 已配置 |
| `strip = true` | ✅ 已配置 |
| 依赖精简 | ❓ 待分析 |

**限制：** Dioxus 0.7 WASM 模式下，所有组件打包在单个 `.wasm` 文件中，不支持 code splitting / 动态导入。路由级懒加载在 WASM 模式下不可行。

## 2. 目标

1. **后端查询优化**：消除 N+1 查询，Dashboard 并行化
2. **热点缓存**：菜单 + 字典数据 Redis 缓存，命中率 > 80%
3. **前端 WASM 精简**：分析依赖，移除不必要的 features
4. **验证**：Dashboard 响应时间 < 100ms，缓存命中后菜单查询 < 5ms

## 3. 后端查询优化设计

### 3.1 菜单查询优化

**优化前**（4+ 次查询）：
```
1. inner_join + .one() → user_id
2. filter user_id → user_roles
3. filter role_ids → role_menus
4. filter menu_ids → menus
5. while → parent menus
```

**优化后**（2 次查询）：
```
1. 直接用 username 查 sys_user 表获取 user_id（或 join 一次查出角色关联）
2. 用 role_ids 一次性查菜单 + 父菜单（递归 CTE 或一次性查全量菜单后内存过滤）
```

**具体方案：**

```rust
pub async fn get_menus_by_username(db: &DatabaseConnection, username: &str) -> Result<Vec<sys_menu::Model>, ServiceError> {
    // admin 直接返回所有可见菜单
    if username == "admin" {
        return SysMenu::find()
            .filter(sys_menu::Column::Hidden.eq(0))
            .all(db).await;
    }

    // 非 admin：2 次查询
    // 1. 通过 username 直接查用户角色关联（join sys_user）
    let user_roles = SysUserRole::find()
        .inner_join(model::dao::sys_user::Entity)
        .filter(model::dao::sys_user::Column::Username.eq(username))
        .all(db)  // 注意：改为 .all() 而非 .one()
        .await?;

    let role_ids: Vec<u64> = user_roles.iter().map(|ur| ur.role_id as u64).collect();
    if role_ids.is_empty() {
        return Ok(vec![]);
    }

    // 2. 一次查全量菜单，内存过滤（菜单通常 < 200 条，全量加载更高效）
    let all_menus = SysMenu::find()
        .filter(sys_menu::Column::Hidden.eq(0))
        .all(db).await?;

    // 3. 查角色菜单关联，内存计算
    let role_menus = SysRoleMenus::find()
        .filter(sys_role_menus::Column::SysRoleRoleId.is_in(role_ids))
        .all(db).await?;

    let menu_ids: HashSet<i32> = role_menus.iter().map(|rm| rm.sys_base_menu_id as i32).collect();

    // 4. 内存过滤 + 补全父菜单
    let mut result: Vec<sys_menu::Model> = all_menus.iter()
        .filter(|m| menu_ids.contains(&m.id))
        .cloned()
        .collect();

    // 补全父菜单（内存操作，无 DB 查询）
    let all_menu_map: HashMap<i32, &sys_menu::Model> = all_menus.iter().map(|m| (m.id, m)).collect();
    let mut added: HashSet<i32> = result.iter().map(|m| m.id).collect();
    for m in &result {
        let mut parent_id = m.parent_id.map(|pid| pid as i32);
        while let Some(pid) = parent_id {
            if pid == 0 || added.contains(&pid) { break; }
            if let Some(parent) = all_menu_map.get(&pid) {
                result.push(parent.clone());
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
```

**优化效果**：4+ 次 DB 查询 → 3 次 DB 查询（且第 3 次可以合并到第 2 次），消除 while 循环 DB 往返。

### 3.2 Dashboard 并行化

```rust
pub async fn dashboard_stats(db: &DatabaseConnection) -> Result<DashboardStats, ServiceError> {
    let (user_count, role_count, menu_count, api_count) = tokio::try_join!(
        SysUser::find().count(db),
        SysRole::find().count(db),
        SysMenu::find().count(db),
        SysApis::find().count(db),
    )?;
    Ok(DashboardStats { user_count, role_count, menu_count, api_count })
}
```

**优化效果**：~20ms → ~5ms（4 倍提升）

## 4. 后端热点缓存设计

### 4.1 缓存架构

采用 **Cache-Aside** 模式：
1. 先查 Redis，命中则直接返回
2. Miss 则查 DB，回填 Redis（带 TTL）
3. 数据变更时主动删除缓存

### 4.2 缓存对象

| 缓存对象 | Redis Key | TTL | 序列化 | 失效时机 |
|----------|-----------|-----|--------|----------|
| 用户菜单 | `user_menus:{username}` | 1h | JSON | 菜单 CRUD、角色菜单变更 |
| 字典列表 | `dict:list` | 2h | JSON | 字典 CRUD |
| Dashboard 统计 | `dashboard:stats` | 5min | JSON | 用户/角色/菜单/API 变更 |

### 4.3 缓存工具模块

在 `utils/src/cache.rs` 新建缓存工具模块：

```rust
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{de::DeserializeOwned, Serialize};

pub struct Cache;

impl Cache {
    /// 获取缓存（JSON 序列化）
    pub async fn get<T: DeserializeOwned>(redis: &mut ConnectionManager, key: &str) -> Option<T> {
        let val: Option<String> = redis.get(key).await.ok()?;
        serde_json::from_str(&val?).ok()
    }

    /// 设置缓存（JSON 序列化 + TTL）
    pub async fn set<T: Serialize>(redis: &mut ConnectionManager, key: &str, val: &T, ttl_secs: u64) {
        if let Ok(json) = serde_json::to_string(val) {
            let _: Result<(), _> = redis.set_ex(key, json, ttl_secs).await;
        }
    }

    /// 删除缓存
    pub async fn del(redis: &mut ConnectionManager, key: &str) {
        let _: Result<(), _> = redis.del(key).await;
    }

    /// 按模式删除缓存
    pub async fn del_pattern(redis: &mut ConnectionManager, pattern: &str) {
        let keys: Vec<String> = redis.keys(pattern).await.unwrap_or_default();
        for key in keys {
            let _: Result<(), _> = redis.del(&key).await;
        }
    }
}
```

### 4.4 菜单缓存集成

在 `SysMenuService::get_menus_by_username` 中集成缓存：

```rust
pub async fn get_menus_by_username(
    db: &DatabaseConnection,
    redis: &mut ConnectionManager,
    username: &str,
) -> Result<Vec<sys_menu::Model>, ServiceError> {
    let cache_key = format!("user_menus:{}", username);

    // 1. 查缓存
    if let Some(cached) = Cache::get::<Vec<sys_menu::Model>>(redis, &cache_key).await {
        return Ok(cached);
    }

    // 2. Miss → 查 DB
    let menus = Self::query_menus_by_username(db, username).await?;

    // 3. 回填缓存
    Cache::set(redis, &cache_key, &menus, 3600).await;

    Ok(menus)
}
```

### 4.5 缓存失效策略

| 触发操作 | 失效的缓存 |
|----------|-----------|
| 菜单 CRUD | `user_menus:*`（全部用户菜单缓存） |
| 角色菜单变更 | `user_menus:*` |
| 字典 CRUD | `dict:list` |
| 用户/角色/菜单/API CRUD | `dashboard:stats` |

在 Service 层的 `insert`/`update`/`delete` 方法中，操作完成后调用缓存失效。

### 4.6 API 层变更

`get_menus_by_username` 需要传入 `redis` 参数，API handler 从 `state.redis` 获取：

```rust
pub async fn get_menus(
    State(state): State<AppState>,
    Extension(username): Extension<Username>,
) -> Result<impl IntoResponse, AppError> {
    let menus = SysMenuService::get_menus_by_username(&state.db, &mut state.redis.clone(), &username.0).await?;
    Ok(R::ok(menus))
}
```

## 5. 前端 WASM 体积优化

### 5.1 依赖分析

检查 `web/Cargo.toml` 中的依赖，识别可精简的 features：

| 依赖 | 当前 features | 可精简 |
|------|-------------|--------|
| `web-sys` | Storage, Window, MediaQueryList, Blob, Url, Document, Element, HtmlElement, FileReader, File, FileList, Event | 检查未使用的 |
| `dioxus` | router | 必需 |
| `dioxus-element-plug` | web | 必需 |

### 5.2 优化措施

1. **web-sys features 精简**：分析实际使用情况，移除未使用的 features
2. **serde_json 优化**：确保 release profile 已启用优化
3. **构建产物分析**：使用 `twiggy` 工具分析 WASM 体积构成

### 5.3 限制说明

Dioxus 0.7 WASM 模式下：
- 所有组件打包在单个 `.wasm` 文件中
- 不支持 code splitting / 动态导入
- 路由级懒加载不可行
- 体积优化主要依赖 release profile + 依赖精简

## 6. 验收标准

- [ ] 菜单查询从 4+ 次 DB 查询降至 2-3 次
- [ ] Dashboard 统计并行化，响应时间 < 100ms
- [ ] 用户菜单 Redis 缓存上线，TTL 1h
- [ ] 字典列表 Redis 缓存上线，TTL 2h
- [ ] 菜单/角色/字典 CRUD 时自动失效缓存
- [ ] `cargo clippy` 零 warning
- [ ] `cargo test` 全部通过
- [ ] 缓存工具模块有单元测试
