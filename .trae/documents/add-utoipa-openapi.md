# Plan: 为 API 添加 utoipa OpenAPI 文档

## Context

项目 workspace 已声明 `utoipa = "5"` 和 `utoipa-swagger-ui = "9"`，但所有源码中零使用。需要为全部 17 张表的 API 添加 OpenAPI 3.1 文档，包括：
- DTO struct 添加 `ToSchema` derive
- API handler 添加 `#[utoipa::path]` 标注
- 创建 `OpenApi` 文档结构体
- 挂载 Swagger UI 和 openapi.json 路由到 gateway

## 实施步骤

### Step 1: 更新 Cargo.toml 依赖

- `model/Cargo.toml`：添加 `utoipa = { workspace = true }`（DTO 的 ToSchema 需要）
- `gateway/Cargo.toml`：添加 `utoipa = { workspace = true }` 和 `utoipa-swagger-ui = { workspace = true }`

### Step 2: 给所有 DTO 添加 ToSchema

每个 DTO struct 的 derive 宏中添加 `ToSchema`。涉及 17 个 DTO 文件，约 50+ 个 struct。

修改模式（统一应用）：
```rust
// 之前
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// 之后
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
```

注意：
- `PageResponse<T>` 需加 `utoipa::ToSchema` + `T: ToSchema` bound
- `R<T>` (utils/res.rs) 也需加 `ToSchema`
- `AppError` 不需要 ToSchema（错误响应由 utoipa path 标注的 responses 声明）
- `LoginResp`（user_api.rs 中的局部 struct）需移至 DTO 或添加 ToSchema
- `CompositeIdPath`（role_btn_api/role_menu_api/data_role_api 中的局部 struct）需添加 `IntoParams` derive

### Step 3: 给所有 API handler 添加 #[utoipa::path] 标注

每个 handler 函数上方添加 `#[utoipa::path]` 宏，声明 path/method/params/responses。

标注模式：
```rust
#[utoipa::path(
    get,
    path = "/api/user/list",
    params(("keyword" = Option<String>, Query, description = "搜索关键字")),
    responses(
        (status = 200, description = "成功", body = R<PageResponse<sys_user::Model>>)
    ),
    tag = "用户管理"
)]
pub async fn list(Query(query): Query<PageRequest>) -> Result<impl IntoResponse, AppError> { ... }
```

复合主键路径标注：
```rust
#[utoipa::path(
    get,
    path = "/api/roleBtn/{role_id}/{sys_menu_id}/{sys_base_menu_btn_id}",
    params(
        ("role_id" = u64, Path, description = "角色ID"),
        ("sys_menu_id" = u64, Path, description = "菜单ID"),
        ("sys_base_menu_btn_id" = u64, Path, description = "按钮ID")
    ),
    responses((status = 200, description = "成功", body = R<sys_role_btns::Model>)),
    tag = "角色按钮"
)]
```

### Step 4: 创建 OpenApi 文档结构体

在 `api/src/lib.rs` 中创建 `ApiDoc`：

```rust
#[derive(OpenApi)]
#[openapi(
    info(title = "SCM Admin API", version = "1.0.0", description = "后台管理系统 API"),
    paths(
        user_api::login, user_api::register, user_api::list, user_api::get_by_id, user_api::update, user_api::delete_user,
        role_api::create, role_api::list, role_api::get_by_id, role_api::update, role_api::delete_role,
        menu_api::create, menu_api::list, menu_api::get_by_id, menu_api::update, menu_api::delete_menu,
        // ... 全部 handler
    ),
    components(schemas(
        // 全部 DTO 和 Entity Model schema
    )),
    tags(
        (name = "用户管理", description = "用户 CRUD"),
        (name = "角色管理", description = "角色 CRUD"),
        (name = "菜单管理", description = "菜单 CRUD"),
        (name = "API管理", description = "API 接口管理"),
        (name = "Casbin策略", description = "权限策略管理"),
        (name = "JWT管理", description = "JWT 黑名单管理"),
        (name = "角色按钮", description = "角色按钮权限"),
        (name = "角色菜单", description = "角色菜单权限"),
        (name = "菜单按钮", description = "菜单按钮管理"),
        (name = "菜单参数", description = "菜单路由参数"),
        (name = "数据权限", description = "角色数据权限"),
        (name = "字典管理", description = "系统字典 CRUD"),
        (name = "字典详情", description = "字典项 CRUD"),
        (name = "操作记录", description = "操作日志查询"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;
```

Security addon（Bearer JWT）：
```rust
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}
```

### Step 5: 挂载 Swagger UI 到 gateway

修改 `gateway/src/main.rs`：
1. 添加 `utoipa`、`utoipa::Modify`、`utoipa_swagger_ui` 导入
2. 在 Router 中添加 Swagger UI 路由（public，无需认证）

```rust
let app = Router::new()
    .merge(api::public_routes())
    .merge(api::swagger_routes())  // Swagger UI + openapi.json
    .merge(api::protected_routes().layer(...));
```

`api/src/lib.rs` 中新增：
```rust
pub fn swagger_routes() -> Router {
    utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
        .url("/openapi.json", ApiDoc::openapi())
        .into()
}
```

### Step 6: R<T> 添加 ToSchema

修改 `utils/src/res.rs`，给 `R<T>` 添加 `ToSchema` derive（需添加 utoipa 依赖到 utils — 已有）。

## 变更文件清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `model/Cargo.toml` | 修改 | 添加 utoipa workspace 依赖 |
| `gateway/Cargo.toml` | 修改 | 添加 utoipa + utoipa-swagger-ui |
| `utils/src/res.rs` | 修改 | R\<T\> 添加 ToSchema |
| `model/src/dto/*.rs`（17 个文件）| 修改 | 所有 struct 添加 ToSchema |
| `api/src/*.rs`（14 个文件）| 修改 | 所有 handler 添加 #[utoipa::path] |
| `api/src/lib.rs` | 修改 | 创建 ApiDoc + swagger_routes() |
| `gateway/src/main.rs` | 修改 | 挂载 swagger_routes() |

## 验证步骤

1. `cargo build -p gateway` — 编译通过
2. 启动服务后访问 `http://localhost:8888/swagger-ui` — 看到 Swagger UI
3. 访问 `http://localhost:8888/openapi.json` — 返回完整 OpenAPI 3.1 JSON
4. Swagger UI 中能看到所有 API 分组、参数、响应类型
