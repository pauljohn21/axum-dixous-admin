# Casbin 域 (Domain) 扩展实现计划

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 为 Casbin 增加"域"（domain）维度，实现 admin/wechat/openapi 三模块权限隔离。

**架构：** Casbin 模型从 `(sub, obj, act)` 升级为 `(sub, obj, act, dom)`，`AuthLayer` 持有 `domain` 字段在编译期确定域，路由从 `protected_routes()` 拆分为 `admin_routes()` + `wechat_routes()`，迁移数据为现有策略填充 `domain=admin`。

**技术栈：** Axum 0.8 + Casbin 2 + SeaORM 1 + Dioxus 0.7

**设计文档：** `docs/superpowers/specs/2026-07-20-casbin-domain-design.md`

---

## 文件结构

| 文件 | 操作 | 职责 |
|------|------|------|
| `backend/server/casbin/rbac_model.conf` | 修改 | r/p 增加 dom，g 变 3 元 |
| `backend/server/auth-layer/src/middleware.rs` | 修改 | AuthLayer/AuthMiddleware 增加 domain 字段，enforce 四参数 |
| `backend/server/data/migration/src/m20260701_000001_create_casbin_rule.rs` | 修改 | p 策略 V3 填 admin，g 策略 V2 填 admin |
| `backend/server/service/src/casbin_service.rs` | 修改 | get_policy_by_role/update_role_policies 增加 domain 参数 |
| `backend/server/api/src/casbin_api.rs` | 修改 | role 策略接口增加 domain 路径参数 |
| `backend/server/api/src/lib.rs` | 修改 | protected_routes → admin_routes + wechat_routes |
| `backend/server/gateway/src/main.rs` | 修改 | 两个 AuthLayer 实例分别绑定 admin/wechat 域 |
| `backend/server/service/src/sys_role_service.rs` | 修改 | delete 时清理 casbin 策略增加 domain 过滤 |
| `web/src/components/casbin_manage.rs` | 修改 | 表格增加域列，对话框域输入框 |
| `web/src/i18n/mod.rs` | 修改 | 新增 RuleDomain 等翻译 key |

---

### 任务 1：修改 Casbin 模型配置

**文件：**
- 修改：`backend/server/casbin/rbac_model.conf`

- [ ] **步骤 1：修改 rbac_model.conf**

将文件内容替换为：

```ini
[request_definition]
r = sub, obj, act, dom

[policy_definition]
p = sub, obj, act, dom

[role_definition]
g = _, _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub, r.dom) && keyMatch2(r.obj, p.obj) && regexMatch(r.act, p.act) && r.dom == p.dom
```

- [ ] **步骤 2：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/casbin/rbac_model.conf
git commit -m "feat(casbin): 模型增加 dom 域维度 (r/p/g)"
```

---

### 任务 2：修改 AuthLayer 中间件

**文件：**
- 修改：`backend/server/auth-layer/src/middleware.rs`

- [ ] **步骤 1：AuthLayer 结构体增加 domain 字段**

将 `AuthLayer` 结构体（第 17-21 行）替换为：

```rust
#[derive(Clone)]
pub struct AuthLayer {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub redis: redis::aio::ConnectionManager,
    pub domain: String,
}
```

- [ ] **步骤 2：修改 AuthLayer::new 构造函数**

将 `impl AuthLayer` 块（第 23-30 行）替换为：

```rust
impl AuthLayer {
    pub fn new(
        enforcer: Arc<RwLock<CachedEnforcer>>,
        redis: redis::aio::ConnectionManager,
        domain: String,
    ) -> Self {
        Self { enforcer, redis, domain }
    }
}
```

- [ ] **步骤 3：修改 Layer impl 传递 domain**

将 `impl<S> Layer<S> for AuthLayer` 块（第 32-42 行）替换为：

```rust
impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            enforcer: self.enforcer.clone(),
            redis: self.redis.clone(),
            domain: self.domain.clone(),
        }
    }
}
```

- [ ] **步骤 4：AuthMiddleware 结构体增加 domain 字段**

将 `AuthMiddleware` 结构体（第 44-49 行）替换为：

```rust
#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    enforcer: Arc<RwLock<CachedEnforcer>>,
    redis: redis::aio::ConnectionManager,
    domain: String,
}
```

- [ ] **步骤 5：修改 call 方法中的 enforce 调用**

在 `call` 方法中，找到第 76 行 `let redis = self.redis.clone();`，在其后添加：

```rust
        let domain = self.domain.clone();
```

然后找到第 117 行 `let args = vec![subject, path, action];`，替换为：

```rust
            let args = vec![subject, path, action, domain];
```

- [ ] **步骤 6：编译验证**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo check 2>&1 | head -30
```

预期：`auth-layer` crate 编译报错（因为 `main.rs` 中的 `AuthLayer::new` 还是两参数），但 `auth-layer` 自身编译通过。

- [ ] **步骤 7：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/auth-layer/src/middleware.rs
git commit -m "feat(auth-layer): AuthLayer 增加 domain 字段，enforce 四参数"
```

---

### 任务 3：修改迁移脚本 — 为现有策略填充 domain

**文件：**
- 修改：`backend/server/data/migration/src/m20260701_000001_create_casbin_rule.rs`

- [ ] **步骤 1：修改 p 策略插入语句**

找到第 39-41 行的 columns 声明：

```rust
        let insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2])
```

替换为（增加 V3 列）：

```rust
        let insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2, CasbinRule::V3])
```

- [ ] **步骤 2：为所有 p 策略 values_panic 添加 "admin" 参数**

每一条 `.values_panic(["p".into(), "888".into(), ...])` 都需要在末尾追加 `"admin".into()`。

示例（第一行）：
```rust
// 修改前
.values_panic(["p".into(), "888".into(), "/api/user/list".into(), "GET".into()])
// 修改后
.values_panic(["p".into(), "888".into(), "/api/user/list".into(), "GET".into(), "admin".into()])
```

对所有 p 策略行（第 43-153 行）执行同样操作。具体规则：
- 角色 888 的策略（第 43-131 行）：全部追加 `"admin".into()`
- 角色 8881 的策略（第 133-144 行）：全部追加 `"admin".into()`
- 角色 9528 的策略（第 146-153 行）：全部追加 `"admin".into()`

- [ ] **步骤 3：修改 g 策略插入语句**

找到第 159-168 行：

```rust
        let g_insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1])
            // 用户admin绑定到角色888, 8881, 9528
            .values_panic(["g".into(), "admin".into(), "888".into()])
            .values_panic(["g".into(), "admin".into(), "8881".into()])
            .values_panic(["g".into(), "admin".into(), "9528".into()])
            // 用户test绑定到角色9528
            .values_panic(["g".into(), "test".into(), "9528".into()])
            .to_owned();
```

替换为（增加 V2 列，每条追加 "admin"）：

```rust
        let g_insert = Query::insert()
            .into_table(CasbinRule::Table)
            .columns([CasbinRule::Ptype, CasbinRule::V0, CasbinRule::V1, CasbinRule::V2])
            // 用户admin绑定到角色888, 8881, 9528
            .values_panic(["g".into(), "admin".into(), "888".into(), "admin".into()])
            .values_panic(["g".into(), "admin".into(), "8881".into(), "admin".into()])
            .values_panic(["g".into(), "admin".into(), "9528".into(), "admin".into()])
            // 用户test绑定到角色9528
            .values_panic(["g".into(), "test".into(), "9528".into(), "admin".into()])
            .to_owned();
```

- [ ] **步骤 4：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/data/migration/src/m20260701_000001_create_casbin_rule.rs
git commit -m "feat(migration): casbin 策略数据填充 domain=admin"
```

---

### 任务 4：修改 CasbinService — 增加 domain 参数

**文件：**
- 修改：`backend/server/service/src/casbin_service.rs`

- [ ] **步骤 1：修改 get_policy_by_role 方法签名和查询**

找到第 121-128 行：

```rust
    /// 获取角色的权限策略
    pub async fn get_policy_by_role(db: &DatabaseConnection, role: &str) -> Result<Vec<casbin_rule::Model>, ServiceError> {
        CasbinRule::find()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .all(db)
            .await
            .map_err(Into::into)
    }
```

替换为：

```rust
    /// 获取角色的权限策略（按域过滤）
    pub async fn get_policy_by_role(db: &DatabaseConnection, role: &str, domain: &str) -> Result<Vec<casbin_rule::Model>, ServiceError> {
        CasbinRule::find()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .filter(casbin_rule::Column::V3.eq(domain))
            .all(db)
            .await
            .map_err(Into::into)
    }
```

- [ ] **步骤 2：修改 update_role_policies 方法签名**

找到第 141 行：

```rust
    pub async fn update_role_policies(db: &DatabaseConnection, enforcer: &Arc<RwLock<CachedEnforcer>>, role: &str, policies: Vec<(String, String)>) -> Result<(), ServiceError> {
```

替换为：

```rust
    pub async fn update_role_policies(db: &DatabaseConnection, enforcer: &Arc<RwLock<CachedEnforcer>>, role: &str, domain: &str, policies: Vec<(String, String)>) -> Result<(), ServiceError> {
```

- [ ] **步骤 3：修改 update_role_policies 内部删除逻辑**

找到第 145-149 行：

```rust
        // 删除现有策略
        CasbinRule::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .exec(&txn)
            .await?;
```

替换为（增加 domain 过滤）：

```rust
        // 删除现有策略（限定域）
        CasbinRule::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .filter(casbin_rule::Column::V3.eq(domain))
            .exec(&txn)
            .await?;
```

- [ ] **步骤 4：修改 update_role_policies 内部插入逻辑**

找到第 152-162 行：

```rust
        // 添加新策略
        for (obj, act) in policies {
            let active_model = casbin_rule::ActiveModel {
                id: Set(0),
                ptype: Set(Some("p".to_string())),
                v0: Set(Some(role.to_string())),
                v1: Set(Some(obj)),
                v2: Set(Some(act)),
                v3: Set(None),
                v4: Set(None),
                v5: Set(None),
            };
            active_model.insert(&txn).await?;
        }
```

替换为（v3 填入 domain）：

```rust
        // 添加新策略
        for (obj, act) in policies {
            let active_model = casbin_rule::ActiveModel {
                id: Set(0),
                ptype: Set(Some("p".to_string())),
                v0: Set(Some(role.to_string())),
                v1: Set(Some(obj)),
                v2: Set(Some(act)),
                v3: Set(Some(domain.to_string())),
                v4: Set(None),
                v5: Set(None),
            };
            active_model.insert(&txn).await?;
        }
```

- [ ] **步骤 5：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/service/src/casbin_service.rs
git commit -m "feat(service): CasbinService 增加 domain 参数"
```

---

### 任务 5：修改 Casbin API — 增加 domain 路径参数

**文件：**
- 修改：`backend/server/api/src/casbin_api.rs`

- [ ] **步骤 1：修改 get_policies_by_role 端点**

找到第 122-132 行：

```rust
#[utoipa::path(
    get,
    path = "/api/casbin/role/{role}",
    params(("role" = String, Path, description = "角色名称")),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_policies_by_role(State(state): State<AppState>, Path(role): Path<String>) -> Result<impl IntoResponse, AppError> {
    let policies = CasbinService::get_policy_by_role(&state.db, &role).await?;
    Ok(R::ok(policies))
}
```

替换为：

```rust
#[utoipa::path(
    get,
    path = "/api/casbin/role/{role}/domain/{domain}",
    params(
        ("role" = String, Path, description = "角色名称"),
        ("domain" = String, Path, description = "域名称"),
    ),
    responses((status = 200, description = "成功", body = R<Vec<casbin_rule::Model>>)),
    tag = "Casbin策略"
)]
pub async fn get_policies_by_role(State(state): State<AppState>, Path((role, domain)): Path<(String, String)>) -> Result<impl IntoResponse, AppError> {
    let policies = CasbinService::get_policy_by_role(&state.db, &role, &domain).await?;
    Ok(R::ok(policies))
}
```

- [ ] **步骤 2：修改 update_role_policies 端点**

找到第 102-120 行：

```rust
#[utoipa::path(
    put,
    path = "/api/casbin/role/{role}",
    request_body = UpdateRolePoliciesRequest,
    params(("role" = String, Path, description = "角色名称")),
    responses((status = 200, description = "权限策略更新成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn update_role_policies(State(state): State<AppState>, Path(role): Path<String>, Json(req): Json<UpdateRolePoliciesRequest>) -> Result<impl IntoResponse, AppError> {
    let policies: Vec<(String, String)> = req.casbin_infos
        .into_iter()
        .map(|info| (info.path, info.method))
        .collect();

    CasbinService::update_role_policies(&state.db, &state.enforcer, &role, policies)
        .await?;

    Ok(R::ok(()))
}
```

替换为：

```rust
#[utoipa::path(
    put,
    path = "/api/casbin/role/{role}/domain/{domain}",
    request_body = UpdateRolePoliciesRequest,
    params(
        ("role" = String, Path, description = "角色名称"),
        ("domain" = String, Path, description = "域名称"),
    ),
    responses((status = 200, description = "权限策略更新成功", body = R<serde_json::Value>)),
    tag = "Casbin策略"
)]
pub async fn update_role_policies(State(state): State<AppState>, Path((role, domain)): Path<(String, String)>, Json(req): Json<UpdateRolePoliciesRequest>) -> Result<impl IntoResponse, AppError> {
    let policies: Vec<(String, String)> = req.casbin_infos
        .into_iter()
        .map(|info| (info.path, info.method))
        .collect();

    CasbinService::update_role_policies(&state.db, &state.enforcer, &role, &domain, policies)
        .await?;

    Ok(R::ok(()))
}
```

- [ ] **步骤 3：修改路由注册**

找到第 154-155 行：

```rust
        .route("/api/casbin/role/{role}", get(get_policies_by_role))
        .route("/api/casbin/role/{role}", put(update_role_policies))
```

替换为：

```rust
        .route("/api/casbin/role/{role}/domain/{domain}", get(get_policies_by_role))
        .route("/api/casbin/role/{role}/domain/{domain}", put(update_role_policies))
```

- [ ] **步骤 4：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/api/src/casbin_api.rs
git commit -m "feat(api): casbin role 策略接口增加 domain 路径参数"
```

---

### 任务 6：修改路由分组 — 拆分 admin_routes 和 wechat_routes

**文件：**
- 修改：`backend/server/api/src/lib.rs`

- [ ] **步骤 1：将 protected_routes 改名为 admin_routes**

找到第 129 行：

```rust
pub fn protected_routes() -> Router<AppState> {
```

替换为：

```rust
pub fn admin_routes() -> Router<AppState> {
```

- [ ] **步骤 2：在 admin_routes 后添加 wechat_routes 函数**

在第 146 行 `}` 之后（`admin_routes` 函数结束）添加：

```rust

pub fn wechat_routes() -> Router<AppState> {
    Router::new()
        // 微信小程序 API 路由将在后续任务中添加
        // 当前为空壳，仅用于绑定 wechat 域的 AuthLayer
}
```

- [ ] **步骤 3：更新迁移策略数据中的 casbin 路由路径**

迁移文件中现有的 casbin role 路由路径需更新。但这属于任务 3 中已处理的迁移文件。此步骤无需额外操作。

- [ ] **步骤 4：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/api/src/lib.rs
git commit -m "feat(api): 拆分 protected_routes → admin_routes + wechat_routes"
```

---

### 任务 7：修改 main.rs — 两个 AuthLayer 实例

**文件：**
- 修改：`backend/server/gateway/src/main.rs`

- [ ] **步骤 1：替换 auth_layer 创建和路由组装**

找到第 83 行：

```rust
    let auth_layer = AuthLayer::new(enforcer, redis);
```

替换为：

```rust
    let admin_auth = AuthLayer::new(enforcer.clone(), redis.clone(), "admin".into());
    let wechat_auth = AuthLayer::new(enforcer.clone(), redis.clone(), "wechat".into());
```

- [ ] **步骤 2：替换路由组装部分**

找到第 106-119 行：

```rust
    let app = Router::new()
        .merge(api::public_routes().with_state(state.clone()))
        .merge(api::swagger_routes())
        .merge(
            api::protected_routes()
                .with_state(state.clone())
                .layer(auth_layer),
        )
        .layer(GovernorLayer::new(governor_config))
        .layer(body_limit)
        .layer(cors)
        .layer(compression)
        .layer(timeout)
        .layer(trace);
```

替换为：

```rust
    let app = Router::new()
        .merge(api::public_routes().with_state(state.clone()))
        .merge(api::swagger_routes())
        .merge(
            api::admin_routes()
                .with_state(state.clone())
                .layer(admin_auth),
        )
        .merge(
            api::wechat_routes()
                .with_state(state.clone())
                .layer(wechat_auth),
        )
        .layer(GovernorLayer::new(governor_config))
        .layer(body_limit)
        .layer(cors)
        .layer(compression)
        .layer(timeout)
        .layer(trace);
```

- [ ] **步骤 3：编译验证后端**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo check 2>&1 | tail -5
```

预期：编译通过，无错误。

- [ ] **步骤 4：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/gateway/src/main.rs
git commit -m "feat(gateway): 两个 AuthLayer 实例分别绑定 admin/wechat 域"
```

---

### 任务 8：修改 sys_role_service — delete 时增加 domain 过滤

**文件：**
- 修改：`backend/server/service/src/sys_role_service.rs`

- [ ] **步骤 1：修改 delete 方法中的 casbin 策略清理**

找到第 91-97 行：

```rust
        // 清理 Casbin 策略中该角色的权限
        use model::dao::casbin_rule;
        casbin_rule::Entity::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(id.to_string()))
            .exec(&txn)
            .await?;
```

替换为（增加 V3 域过滤 — 清理所有域中该角色的策略）：

```rust
        // 清理 Casbin 策略中该角色的权限（所有域）
        use model::dao::casbin_rule;
        casbin_rule::Entity::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(id.to_string()))
            .exec(&txn)
            .await?;
        // 清理角色绑定（所有域）
        casbin_rule::Entity::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("g"))
            .filter(casbin_rule::Column::V1.eq(id.to_string()))
            .exec(&txn)
            .await?;
```

注意：这里不加 V3 过滤是因为删除角色时要清理所有域中的相关策略。

- [ ] **步骤 2：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/service/src/sys_role_service.rs
git commit -m "feat(service): 角色删除时清理所有域的 casbin 策略"
```

---

### 任务 9：更新迁移文件中的 casbin role 路由路径

**文件：**
- 修改：`backend/server/data/migration/src/m20260701_000001_create_casbin_rule.rs`

- [ ] **步骤 1：更新 casbin role 相关的 p 策略路径**

在任务 3 中已修改了迁移文件的 columns 和 values。现在需要更新 casbin role 的路由路径，因为 API 端点从 `/api/casbin/role/:role` 变为 `/api/casbin/role/:role/domain/:domain`。

找到以下两行（在 p 策略 values_panic 中）：

```rust
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role".into(), "GET".into(), "admin".into()])
```

和

```rust
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role".into(), "PUT".into(), "admin".into()])
```

替换为：

```rust
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role/domain/:domain".into(), "GET".into(), "admin".into()])
```

和

```rust
            .values_panic(["p".into(), "888".into(), "/api/casbin/role/:role/domain/:domain".into(), "PUT".into(), "admin".into()])
```

- [ ] **步骤 2：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add backend/server/data/migration/src/m20260701_000001_create_casbin_rule.rs
git commit -m "fix(migration): 更新 casbin role 路由路径匹配 domain 参数"
```

---

### 任务 10：后端整体编译 + clippy 验证

- [ ] **步骤 1：编译检查**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo check 2>&1 | tail -10
```

预期：编译通过，无错误。如果有错误，根据错误信息修复。

- [ ] **步骤 2：Clippy 检查**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo clippy --all-targets -- -D warnings 2>&1 | tail -20
```

预期：零 warning。如果有 warning，修复后重新运行。

- [ ] **步骤 3：启动后端验证**

确保 Docker 基础设施运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
docker compose up -d
```

启动后端：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo run 2>&1 | tail -20
```

预期：服务正常启动，监听 0.0.0.0:8888，迁移执行成功（fresh 模式重建表并填充 domain=admin 的策略数据）。

- [ ] **步骤 4：验证 API 正常**

```bash
curl -s http://localhost:8888/health | head -5
```

预期：`{"status":"ok"}`

---

### 任务 11：前端 i18n — 新增域相关翻译 key

**文件：**
- 修改：`web/src/i18n/mod.rs`

- [ ] **步骤 1：在 TKey 枚举中新增域相关 key**

找到第 66 行：

```rust
    CasbinManage, AddRule, EditRule, RuleType, RuleSub, RuleObj, RuleAct,
    SearchRulePlaceholder, RuleSubPlaceholder, RuleObjPlaceholder, RuleActPlaceholder, CasbinHelp,
```

替换为：

```rust
    CasbinManage, AddRule, EditRule, RuleType, RuleSub, RuleObj, RuleAct, RuleDomain,
    SearchRulePlaceholder, RuleSubPlaceholder, RuleObjPlaceholder, RuleActPlaceholder, RuleDomainPlaceholder, CasbinHelp,
```

- [ ] **步骤 2：在 t_zh 函数中新增翻译**

找到第 151-157 行：

```rust
        TKey::CasbinManage => "权限管理", TKey::AddRule => "+ 新增规则", TKey::EditRule => "编辑规则",
        TKey::RuleType => "类型", TKey::RuleSub => "主体", TKey::RuleObj => "对象", TKey::RuleAct => "动作",
        TKey::SearchRulePlaceholder => "搜索类型/主体/对象/动作",
        TKey::RuleSubPlaceholder => "角色ID (如 888) 或用户名",
        TKey::RuleObjPlaceholder => "API路径 (如 /api/user/list)",
        TKey::RuleActPlaceholder => "HTTP方法 (GET/POST/PUT/DELETE)",
        TKey::CasbinHelp => "策略类型 p = 权限策略 (主体→对象→动作)，g = 角色绑定 (用户→角色)。主体填写角色ID，对象填写API路径。",
```

替换为：

```rust
        TKey::CasbinManage => "权限管理", TKey::AddRule => "+ 新增规则", TKey::EditRule => "编辑规则",
        TKey::RuleType => "类型", TKey::RuleSub => "主体", TKey::RuleObj => "对象", TKey::RuleAct => "动作", TKey::RuleDomain => "域",
        TKey::SearchRulePlaceholder => "搜索类型/主体/对象/动作",
        TKey::RuleSubPlaceholder => "角色ID (如 888) 或用户名",
        TKey::RuleObjPlaceholder => "API路径 (如 /api/user/list)",
        TKey::RuleActPlaceholder => "HTTP方法 (GET/POST/PUT/DELETE)",
        TKey::RuleDomainPlaceholder => "域名称 (如 admin/wechat/openapi)",
        TKey::CasbinHelp => "策略类型 p = 权限策略 (主体→对象→动作→域)，g = 角色绑定 (用户→角色→域)。主体填写角色ID，对象填写API路径，域标识子系统分区。",
```

- [ ] **步骤 3：在 t_en 函数中新增翻译**

找到第 226-232 行：

```rust
        TKey::CasbinManage => "Permission Management", TKey::AddRule => "+ Add Rule", TKey::EditRule => "Edit Rule",
        TKey::RuleType => "Type", TKey::RuleSub => "Subject", TKey::RuleObj => "Object", TKey::RuleAct => "Action",
        TKey::SearchRulePlaceholder => "Search type/subject/object/action",
        TKey::RuleSubPlaceholder => "Role ID (e.g. 888) or username",
        TKey::RuleObjPlaceholder => "API path (e.g. /api/user/list)",
        TKey::RuleActPlaceholder => "HTTP method (GET/POST/PUT/DELETE)",
        TKey::CasbinHelp => "Policy type p = permission policy (subject→object→action), g = role binding (user→role). Subject is role ID, Object is API path.",
```

替换为：

```rust
        TKey::CasbinManage => "Permission Management", TKey::AddRule => "+ Add Rule", TKey::EditRule => "Edit Rule",
        TKey::RuleType => "Type", TKey::RuleSub => "Subject", TKey::RuleObj => "Object", TKey::RuleAct => "Action", TKey::RuleDomain => "Domain",
        TKey::SearchRulePlaceholder => "Search type/subject/object/action",
        TKey::RuleSubPlaceholder => "Role ID (e.g. 888) or username",
        TKey::RuleObjPlaceholder => "API path (e.g. /api/user/list)",
        TKey::RuleActPlaceholder => "HTTP method (GET/POST/PUT/DELETE)",
        TKey::RuleDomainPlaceholder => "Domain name (e.g. admin/wechat/openapi)",
        TKey::CasbinHelp => "Policy type p = permission policy (subject→object→action→domain), g = role binding (user→role→domain). Subject is role ID, Object is API path, Domain identifies the subsystem partition.",
```

- [ ] **步骤 4：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add web/src/i18n/mod.rs
git commit -m "feat(i18n): 新增 RuleDomain 等域相关翻译 key"
```

---

### 任务 12：前端权限管理页面 — 增加域列和域输入框

**文件：**
- 修改：`web/src/components/casbin_manage.rs`

- [ ] **步骤 1：修改 CasbinRow 结构体增加 v3 字段**

找到第 29-38 行：

```rust
struct CasbinRow {
    id: u64,
    ptype: String,
    ptype_color: String,
    v0: String,
    v1: String,
    v2: String,
    v2_color: String,
    original: CasbinRule,
}
```

替换为：

```rust
struct CasbinRow {
    id: u64,
    ptype: String,
    ptype_color: String,
    v0: String,
    v1: String,
    v2: String,
    v2_color: String,
    v3: String,
    original: CasbinRule,
}
```

- [ ] **步骤 2：修改表格行预计算逻辑**

找到第 156-169 行：

```rust
    let rows: Vec<CasbinRow> = rules().into_iter().map(|item| {
        let ptype_str = item.ptype.clone().unwrap_or_default();
        let v2_str = item.v2.clone().unwrap_or_default();
        CasbinRow {
            id: item.id,
            ptype_color: ptype_color(&ptype_str),
            ptype: ptype_str,
            v0: item.v0.clone().unwrap_or_default(),
            v1: item.v1.clone().unwrap_or_default(),
            v2_color: method_color(&v2_str),
            v2: v2_str,
            original: item,
        }
    }).collect();
```

替换为：

```rust
    let rows: Vec<CasbinRow> = rules().into_iter().map(|item| {
        let ptype_str = item.ptype.clone().unwrap_or_default();
        let v2_str = item.v2.clone().unwrap_or_default();
        let v3_str = item.v3.clone().unwrap_or_default();
        CasbinRow {
            id: item.id,
            ptype_color: ptype_color(&ptype_str),
            ptype: ptype_str,
            v0: item.v0.clone().unwrap_or_default(),
            v1: item.v1.clone().unwrap_or_default(),
            v2_color: method_color(&v2_str),
            v2: v2_str,
            v3: v3_str,
            original: item,
        }
    }).collect();
```

- [ ] **步骤 3：修改表头增加"域"列**

找到第 218-225 行：

```rust
                        tr {
                            th { style: "{th_s}", "ID" }
                            th { style: "{th_s}", "{t(TKey::RuleType)}" }
                            th { style: "{th_s}", "主体 (角色/用户)" }
                            th { style: "{th_s}", "对象 (路径)" }
                            th { style: "{th_s}", "动作 (方法)" }
                            th { style: "{th_s}", "{t(TKey::Action)}" }
                        }
```

替换为：

```rust
                        tr {
                            th { style: "{th_s}", "ID" }
                            th { style: "{th_s}", "{t(TKey::RuleType)}" }
                            th { style: "{th_s}", "主体 (角色/用户)" }
                            th { style: "{th_s}", "对象 (路径)" }
                            th { style: "{th_s}", "动作 (方法)" }
                            th { style: "{th_s}", "{t(TKey::RuleDomain)}" }
                            th { style: "{th_s}", "{t(TKey::Action)}" }
                        }
```

- [ ] **步骤 4：修改 loading 和 empty 行的 colspan**

找到第 229 行：

```rust
                            tr { td { colspan: "6", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::Loading)}" } }
```

替换为：

```rust
                            tr { td { colspan: "7", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::Loading)}" } }
```

找到第 231 行：

```rust
                            tr { td { colspan: "6", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::NoData)}" } }
```

替换为：

```rust
                            tr { td { colspan: "7", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::NoData)}" } }
```

- [ ] **步骤 5：修改表格行增加域单元格**

找到第 256-257 行（在 v2 单元格的 `}` 之后，`td` 操作列之前）：

```rust
                                    }
                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
```

在 `}` 和 `td {` 之间插入域列单元格：

```rust
                                    }
                                    td {
                                        style: "{td_s} font-family: monospace;",
                                        if row.v3.is_empty() {
                                            span { style: "color: var(--el-text-color-placeholder);", "-" }
                                        } else {
                                            span {
                                                style: "display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 12px; font-weight: 600; color: #909399; background: #9093991a;",
                                                "{row.v3}"
                                            }
                                        }
                                    }
                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
```

- [ ] **步骤 6：修改对话框中 V3 标签为"域"**

找到第 360-368 行：

```rust
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "V3" }
                            Input {
                                value: Some(form_v3()),
                                placeholder: Some(t(TKey::RuleSubPlaceholder)),
                                on_change: move |e: Event<FormData>| { form_v3.set(e.data().value()); }
                            }
                        }
```

替换为：

```rust
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::RuleDomain)}" }
                            Input {
                                value: Some(form_v3()),
                                placeholder: Some(t(TKey::RuleDomainPlaceholder)),
                                on_change: move |e: Event<FormData>| { form_v3.set(e.data().value()); }
                            }
                        }
```

- [ ] **步骤 7：修改新增按钮初始化 form_v3 默认值**

找到第 182-186 行：

```rust
                    form_ptype.set(String::from("p"));
                    form_v0.set(String::new());
                    form_v1.set(String::new());
                    form_v2.set(String::new());
                    form_v3.set(String::new());
```

替换为（新增时默认填 admin）：

```rust
                    form_ptype.set(String::from("p"));
                    form_v0.set(String::new());
                    form_v1.set(String::new());
                    form_v2.set(String::new());
                    form_v3.set(String::from("admin"));
```

- [ ] **步骤 8：Commit**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add web/src/components/casbin_manage.rs
git commit -m "feat(web): 权限管理页面增加域列和域输入框"
```

---

### 任务 13：前端编译验证

- [ ] **步骤 1：前端编译检查**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/web
cargo check 2>&1 | tail -10
```

预期：编译通过，无错误。如果有错误，根据错误信息修复。

- [ ] **步骤 2：前端 clippy 检查**

运行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/web
cargo clippy --all-targets -- -D warnings 2>&1 | tail -20
```

预期：零 warning。如果有 warning，修复后重新运行。

---

### 任务 14：手动集成验证

- [ ] **步骤 1：确保后端运行**

后端已在任务 10 步骤 3 启动。如果未运行，执行：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin/backend
cargo run &
```

- [ ] **步骤 2：清除 Redis 菜单缓存**

```bash
redis-cli DEL user_menus:admin
```

- [ ] **步骤 3：启动前端**

```bash
cd /Users/pauljohn/rust/axum-dixous-admin/web
dx serve &
```

- [ ] **步骤 4：浏览器验证**

打开浏览器 `http://localhost:8080`，使用 admin 账号登录：

1. 登录成功 → 说明 admin 域 enforce 通过
2. 侧边栏显示"权限管理"菜单 → 说明菜单缓存正常
3. 进入权限管理页面 → 表格应显示 7 列（含"域"列），所有策略的域列显示 `admin`
4. 点击"新增规则" → 对话框中"域"输入框默认值为 `admin`
5. 编辑现有规则 → "域"输入框显示当前策略的域值

- [ ] **步骤 5：最终 Commit（如有修复）**

如果有任何修复，提交后打 tag：
```bash
cd /Users/pauljohn/rust/axum-dixous-admin
git add -A
git commit -m "feat: Casbin 域支持完整实现"
```
