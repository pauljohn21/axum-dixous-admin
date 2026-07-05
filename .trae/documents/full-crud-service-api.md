# Plan: 全表 Service + API + AGENTS.md

## Context

项目有 17 张数据表（已全部有迁移文件），但仅 3 张表有 SeaORM Entity（且字段不完整），2 个 Service，1 个 API 文件。需要为所有表补全 Entity → DTO → Service → API 的完整 CRUD 链路，并编写 AGENTS.md 规范文档。

## 分阶段实施

### Phase 0：基础设施

1. **修复 `sys_menu_domain` 迁移** — 当前迁移文件创建的是 `Post` 表而非 `sys_menu_domain`
2. **补全 `sys_user` Entity 字段**（缺 13 个：nick_name, side_mode, header_img, base_color, active_color, email, enable, created_at, updated_at, deleted_at, created_ad, updated_ad, deleted_ad）
3. **补全 `sys_role` Entity 字段**（缺 4 个：parent_id, created_ad, updated_ad, deleted_ad）
4. **新建 `model/src/dto/page_dto.rs`** — 通用分页请求 `PageRequest` + 响应 `PageResponse<T>`
5. **扩展 `AppError`** — 增加 `NotFoundError(String)` 变体
6. **确保 `model/Cargo.toml` 的 sea-orm 启用 `with-time` feature**（OffsetDateTime 类型需要）

### Phase 1：核心业务（用户/角色/菜单）— 6 张表

| 表 | 操作 |
|---|---|
| `sys_user` | Entity 补全字段 → DTO 补全(Insert/Update/Query/Resp) → Service 补全 CRUD → API |
| `sys_role` | Entity 补全字段 → 新建 DTO → 新建 Service → API |
| `sys_user_role` | Entity 已完整 → DTO 补全 → Service 补全 → API |
| `sys_menu` | 新建 Entity → DTO → Service → API |
| `sys_menu_role` | 新建 Entity → DTO → Service → API |
| `sys_menu_domain` | 新建 Entity（修复迁移后）→ DTO → Service → API |

**API 路由设计：**

```
POST   /api/user/login            登录
POST   /api/user/register         注册
GET    /api/user/info             当前用户信息
PUT    /api/user/info             更新当前用户信息
PUT    /api/user/password         修改密码
GET    /api/user/list             分页列表
GET    /api/user/{id}             单条查询
PUT    /api/user/{id}             更新
DELETE /api/user/{id}             删除
POST   /api/user/{id}/roles       设置用户角色

POST   /api/role                  创建角色
GET    /api/role/list             角色列表
GET    /api/role/{id}             单条查询
PUT    /api/role/{id}             更新
DELETE /api/role/{id}             删除

POST   /api/menu                  创建菜单
GET    /api/menu/list             菜单列表
GET    /api/menu/tree             菜单树
GET    /api/menu/{id}             单条查询
PUT    /api/menu/{id}             更新
DELETE /api/menu/{id}             删除
```

### Phase 2：系统基础设施 — 5 张表

| 表 | 操作 | 备注 |
|---|---|---|
| `sys_apis` | 新建 Entity → DTO → Service → API | API 管理 |
| `jwt_blacklists` | 新建 Entity → DTO → Service → API | JWT 黑名单 |
| `casbin_rule` | 新建 Entity → DTO → Service(只读) → API | 由 casbin adapter 管理，仅读取 |
| `sys_role_btns` | 新建 Entity(复合主键) → DTO → Service → API | 无自增主键，全字段作复合主键 |
| `sys_role_menus` | 新建 Entity(复合主键) → DTO → Service → API | 复合主键 |

### Phase 3：菜单扩展 + 字典 + 操作记录 — 6 张表

| 表 | 操作 |
|---|---|
| `sys_base_menu_btns` | 新建 Entity → DTO → Service → API |
| `sys_base_menu_parameters` | 新建 Entity → DTO → Service → API |
| `sys_data_role_id` | 新建 Entity(复合主键) → DTO → Service → API |
| `sys_dictionaries` | 新建 Entity → DTO → Service → API |
| `sys_dictionary_details` | 新建 Entity → DTO → Service → API |
| `sys_operation_records` | 新建 Entity → DTO → Service → API |

### Phase 4：AGENTS.md + 收尾

1. 编写 `backend/AGENTS.md` 更新（添加新的 service/api 模式说明）
2. 更新 `gateway/src/main.rs` 的 Casbin seed 策略（为所有新路由添加策略）
3. 确保全量 `cargo build` 通过

## 代码模式

**Entity 模板**（手写，不使用 sea-orm-cli）：
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Eq)]
#[sea_orm(table_name = "sys_apis")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: Option<OffsetDateTime>,
    // ...
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
```

**Service 模板**（静态方法，db_conn!() 获取连接）：
```rust
pub struct SysXxxService;
impl SysXxxService {
    pub async fn list(query: SysXxxQueryDTO) -> Result<PageResponse<sys_xxx::Model>> { ... }
    pub async fn get_by_id(id: u64) -> Result<sys_xxx::Model> { ... }
    pub async fn insert(data: SysXxxInsertDTO) -> Result<sys_xxx::Model> { ... }
    pub async fn update(id: u64, data: SysXxxUpdateDTO) -> Result<sys_xxx::Model> { ... }
    pub async fn delete(id: u64) -> Result<()> { ... }
}
```

**API 模板**（每个模块一个文件，导出 `routes()` 函数）：
```rust
pub fn routes() -> Router {
    Router::new()
        .route("/api/xxx/list", get(list))
        .route("/api/xxx/{id}", get(get_by_id).put(update).delete(delete))
        .route("/api/xxx", post(create))
}
```

## 变更文件清单

**新建文件（~45 个）：**
- `model/src/dao/` — 14 个新 Entity 文件
- `model/src/dto/` — 14 个新 DTO 文件 + page_dto.rs
- `service/src/` — 15 个新 Service 文件
- `api/src/` — 13 个新 API 文件
- `backend/AGENTS.md` — 更新

**修改文件：**
- `model/src/dao/sys_user.rs` — 补全 13 个字段
- `model/src/dao/sys_role.rs` — 补全 4 个字段
- `model/src/dao/mod.rs` + `prelude.rs` — 注册新 Entity
- `model/src/dto/mod.rs` — 注册新 DTO
- `service/src/lib.rs` — 注册新 Service
- `api/src/lib.rs` — 重构为多模块路由注册
- `migration/src/m20250211_071223_create_sys_menu_domain.rs` — 修复表名
- `gateway/src/main.rs` — 更新 Casbin seed 策略
- `utils/src/error.rs` — 添加 NotFoundError

## 特殊表处理

| 表 | 问题 | 方案 |
|---|---|---|
| `sys_role_btns` | 无自增主键 | 三个字段全部标 `#[sea_orm(primary_key, auto_increment = false)]` |
| `sys_role_menus` | 复合主键 | 双字段标 `primary_key, auto_increment = false` |
| `sys_data_role_id` | 复合主键 | 同上 |
| `casbin_rule` | 由 casbin adapter 管理 | Entity + 只读 Service（list/get_by_id） |
| `jwt_blacklists` | 退出登录专用 | insert = 加入黑名单，需在 auth_middleware 增加黑名单检查 |

## 验证步骤

1. 每个 Phase 完成后 `cargo build -p gateway` 编译通过
2. Phase 1 后：登录、注册、用户列表 CRUD 测试
3. Phase 2 后：API 管理 CRUD、Casbin 策略读取测试
4. Phase 3 后：字典 CRUD、操作记录查询测试
5. 最终 `cargo build` 全量编译零错误
