# Phase 4 — 性能优化实现计划

> 日期：2026-07-20
> 设计文档：`docs/superpowers/specs/2026-07-20-phase4-performance-design.md`
> 前置条件：Phase 1、2、3 已完成

## 任务清单

### 任务 1：创建缓存工具模块

**目标：** 在 `utils/src/cache.rs` 中实现通用的 Redis Cache-Aside 工具。

**步骤：**
1. 创建 `utils/src/cache.rs`，实现 `Cache` 结构体（get/set/del/del_pattern）
2. 在 `utils/src/lib.rs` 注册 `cache` 模块
3. 在 `utils/src/prelude.rs` 导出 `Cache`

**验证：** `cargo build` 编译通过

**提交：** `feat: 创建 Redis 缓存工具模块（任务 1/5）`

---

### 任务 2：Dashboard 查询并行化

**目标：** 将 `dashboard_stats` 的 4 个串行 count 查询改为 `tokio::try_join!` 并行。

**步骤：**
1. 修改 `service/src/sys_user_service.rs` 的 `dashboard_stats` 方法
2. 用 `tokio::try_join!` 并行执行 4 个 count

**验证：** `cargo test` 通过，clippy 零 warning

**提交：** `perf: Dashboard 统计查询并行化（任务 2/5）`

---

### 任务 3：菜单查询优化 + 缓存集成

**目标：** 优化 `get_menus_by_username` 消除 N+1，集成 Redis 缓存。

**步骤：**
1. 重构 `get_menus_by_username`：消除冗余的 `.one()` 查询，改用 `.all()`
2. 补全父菜单改为内存操作（从全量菜单中查找，无 DB 查询）
3. 添加 `redis: &mut ConnectionManager` 参数
4. 集成 Cache-Aside：先查缓存 → miss 查 DB → 回填缓存
5. 更新 API handler 传入 `state.redis`
6. 在菜单 CRUD 方法中添加缓存失效（`del_pattern("user_menus:*")`）

**验证：** `cargo test` 通过，clippy 零 warning

**提交：** `perf: 菜单查询优化 + Redis 缓存集成（任务 3/5）`

---

### 任务 4：字典缓存 + Dashboard 缓存

**目标：** 字典列表和 Dashboard 统计添加 Redis 缓存。

**步骤：**
1. 在 `SysDictionaryService::list` 添加缓存（key: `dict:list`, TTL 2h）
2. 在字典 CRUD 方法中添加缓存失效
3. 在 `dashboard_stats` 添加缓存（key: `dashboard:stats`, TTL 5min）
4. 在用户/角色/菜单/API CRUD 方法中添加 Dashboard 缓存失效

**验证：** `cargo test` 通过，clippy 零 warning

**提交：** `perf: 字典 + Dashboard 缓存集成（任务 4/5）`

---

### 任务 5：前端 WASM 依赖精简 + 最终验证

**目标：** 分析前端依赖，精简不必要的 features，全量验证。

**步骤：**
1. 检查 `web-sys` features 使用情况，移除未使用的
2. `cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings`
3. `cargo test --manifest-path backend/Cargo.toml`
4. `cd web/crates/dioxus-i18n && cargo test`

**提交：** `perf: 前端 WASM 依赖精简 + 最终验证（任务 5/5）`

---

## 执行顺序

```
任务 1 (缓存工具) ──→ 任务 3 (菜单缓存) ──→ 任务 5 (验证)
                  ──→ 任务 4 (字典+Dashboard缓存) ─┘
任务 2 (Dashboard并行) ──────────────────────────┘
```

任务 1 是基础。任务 2 独立。任务 3、4 依赖任务 1。任务 5 最后。
