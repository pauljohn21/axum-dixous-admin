# Phase 2 — 测试体系实现计划

> 日期：2026-07-20
> 设计文档：`docs/superpowers/specs/2026-07-20-phase2-test-system-design.md`
> 前置条件：Phase 1 已完成

## 任务清单

### 任务 1：创建测试辅助模块 `tests/common/mod.rs`

**目标：** 提供 `setup_test_state()`、`setup_test_db()`、`setup_test_redis()`、`setup_test_enforcer()`、`insert_test_user()` 等辅助函数。

**步骤：**
1. 创建 `backend/tests/common/mod.rs`
2. 实现 `setup_test_db()` — 连接 `scm_test` 数据库 + `Migrator::fresh()`
3. 实现 `setup_test_redis()` — 连接 Redis DB index 1 + `FLUSHDB`
4. 实现 `setup_test_enforcer(db)` — 创建 Casbin enforcer
5. 实现 `setup_test_state()` — 组装完整 AppState
6. 实现 `insert_test_user(db)` — 插入测试用户 fixtures

**验证：** `cargo build --tests` 编译通过

**提交：** `test: 创建测试辅助模块（任务 1/8）`

---

### 任务 2：更新 `Cargo.toml` dev-dependencies

**目标：** 追加 `http-body-util` 和 `tower` 的 `util` feature。

**步骤：**
1. 编辑 `backend/Cargo.toml`，`[dev-dependencies]` 追加 `http-body-util`
2. `tower` 追加 `features = ["util"]`

**验证：** `cargo build --tests` 编译通过

**提交：** `chore: 更新 dev-dependencies（任务 2/8）`

---

### 任务 3：L2 Mock 测试扩展 — Role/Menu/Api Service Mock

**目标：** 在 `api_tests.rs` 中添加 RoleService、MenuService、ApiService 的 Mock 实现和测试。

**步骤：**
1. 在 `api_tests.rs` 添加 `MockRoleService`、`MockMenuService`、`MockApiService`
2. 添加 `test_mock_role_service_crud` 测试
3. 添加 `test_mock_menu_service_crud` 测试
4. 添加 `test_mock_api_service_crud` 测试

**验证：** `cargo test --test api_tests` 全部通过

**提交：** `test: 扩展 Mock 测试到 Role/Menu/Api Service（任务 3/8）`

---

### 任务 4：L3 集成测试 — User Service

**目标：** 创建 `tests/integration/user_service_test.rs`，用真实 DB 测试用户服务。

**步骤：**
1. 创建 `tests/integration/mod.rs` 声明子模块
2. 创建 `tests/integration/user_service_test.rs`
3. 实现 `test_user_login_success` — 正确密码登录
4. 实现 `test_user_login_wrong_password` — 错误密码
5. 实现 `test_user_crud_lifecycle` — insert → get → update → delete
6. 实现 `test_user_list_with_keyword` — 关键词搜索
7. 实现 `test_user_change_password` — 改密后新密码登录

**验证：** `cargo test --test integration` 通过（需 MySQL + Redis）

**提交：** `test: User Service 集成测试（任务 4/8）`

---

### 任务 5：L3 集成测试 — Role Service + Casbin

**目标：** 创建 Role Service 集成测试和 Casbin 权限测试。

**步骤：**
1. 创建 `tests/integration/role_service_test.rs`
2. 实现 `test_role_crud_lifecycle`
3. 实现 `test_role_delete_cleans_associations` — 删除角色时清理关联
4. 创建 `tests/integration/casbin_test.rs`
5. 实现 `test_casbin_enforce_allow` — 有权限通过
6. 实现 `test_casbin_enforce_deny` — 无权限拒绝
7. 实现 `test_casbin_reload_policy` — 策略重载生效

**验证：** `cargo test --test integration` 通过

**提交：** `test: Role Service + Casbin 集成测试（任务 5/8）`

---

### 任务 6：L4 HTTP 路由测试 — Auth + User API

**目标：** 创建 `tests/api/auth_api_test.rs` 和 `tests/api/user_api_test.rs`。

**步骤：**
1. 创建 `tests/api/mod.rs` 声明子模块
2. 创建 `tests/api/auth_api_test.rs`
3. 实现 `test_health_route` — GET /health
4. 实现 `test_login_route_success` — POST /api/user/login
5. 实现 `test_login_route_wrong_password` — 错误密码返回 401
6. 创建 `tests/api/user_api_test.rs`
7. 实现 `test_get_user_info_no_token` — 无 token 返回 401
8. 实现 `test_get_user_info_route` — 带 JWT 返回用户信息

**验证：** `cargo test --test api` 通过

**提交：** `test: HTTP 路由测试 — Auth + User API（任务 6/8）`

---

### 任务 7：前端纯函数测试

**目标：** 在 `web/src/http/mod.rs` 和 `web/src/models/generator.rs` 中添加 `#[cfg(test)]` 模块。

**步骤：**
1. 在 `web/src/http/mod.rs` 添加 `#[cfg(test)]` 模块，测试 `build_page_query`
2. 在 `web/src/models/generator.rs` 添加 `#[cfg(test)]` 模块，测试 `config_to_json` / `json_to_config` 往返

**验证：** `cd web && cargo test` 通过

**提交：** `test: 前端纯函数测试 — HTTP 层 + Model 序列化（任务 7/8）`

---

### 任务 8：最终验证 + Clippy

**目标：** 全量验证，确保零 warning。

**步骤：**
1. `cargo test --manifest-path backend/Cargo.toml` — 全部通过
2. `cargo clippy --manifest-path backend/Cargo.toml --all-targets` — 零 warning
3. `cd web && cargo test` — 前端测试通过
4. 确认测试总数 > 20

**提交：** `test: Phase 2 最终验证（任务 8/8）`

---

## 执行顺序

```
任务 1 (common/mod.rs) ──→ 任务 4 (User 集成) ──→ 任务 6 (HTTP 路由)
                       │                      │
任务 2 (Cargo.toml) ───┘──→ 任务 5 (Role+Casbin)
                       │
任务 3 (Mock 扩展) ────┘──→ 任务 7 (前端) ──→ 任务 8 (验证)
```

任务 1 和 2 是基础，必须先做。任务 3 独立。任务 4-6 依赖 1+2。任务 7 独立。任务 8 最后。
