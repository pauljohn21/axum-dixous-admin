# Phase 3 — 开发效率实现计划

> 日期：2026-07-20
> 设计文档：`docs/superpowers/specs/2026-07-20-phase3-dev-efficiency-design.md`
> 前置条件：Phase 1、Phase 2 已完成

## 任务清单

### 任务 1：创建 GitHub Actions CI 流水线

**目标：** PR 自动运行 clippy + test + build。

**步骤：**
1. 创建 `.github/workflows/ci.yml`
2. 配置后端检查 Job（MySQL + Redis 服务容器 + clippy + test + build）
3. 配置前端检查 Job（WASM target + dioxus CLI + test + build）
4. 配置 Docker 构建 Job（仅 push to main 时触发）

**验证：** YAML 语法正确，推送后 CI 触发

**提交：** `ci: 创建 GitHub Actions CI 流水线（任务 1/5）`

---

### 任务 2：实现代码生成器 — 后端模板

**目标：** 在 `generator_code_service.rs` 中实现后端 5 个文件的代码生成。

**步骤：**
1. 在 `service/Cargo.toml` 添加 `GeneratorConfig` 反序列化支持（从 `model` 或内联定义）
2. 实现 `gen_migration(config)` — 生成 SeaORM 迁移脚本
3. 实现 `gen_dao(config)` — 生成 SeaORM 实体
4. 实现 `gen_dto(config)` — 生成 DTO (Insert/Update/Query)
5. 实现 `gen_service(config)` — 生成 Service 层 CRUD
6. 实现 `gen_api(config)` — 生成 API 路由 + OpenAPI

**验证：** `cargo build` 编译通过，`preview_code` 返回完整代码内容

**提交：** `feat: 实现代码生成器后端模板（任务 2/5）`

---

### 任务 3：实现代码生成器 — 前端模板

**目标：** 实现前端 3 个文件的代码生成。

**步骤：**
1. 实现 `gen_model(config)` — 生成前端数据模型
2. 实现 `gen_api_call(config)` — 生成 API 调用封装
3. 实现 `gen_component(config)` — 生成管理页面组件骨架

**验证：** `cargo build` 编译通过，生成的前端代码语法正确

**提交：** `feat: 实现代码生成器前端模板（任务 3/5）`

---

### 任务 4：更新 AGENTS.md

**目标：** 反映 Phase 1-2 优化成果。

**步骤：**
1. 添加测试体系部分（测试分层、运行方式、辅助模块）
2. 更新「添加新模块步骤」反映 Service 参数注入
3. 更新「关键约定」反映全局状态清除（移除 `db_conn!()` 相关内容）
4. 添加 CI/CD 说明
5. 更新代码生成器部分

**验证：** 文档内容与代码实际状态一致

**提交：** `docs: 更新 AGENTS.md 反映 Phase 1-3 优化成果（任务 4/5）`

---

### 任务 5：最终验证

**目标：** 全量验证 CI + 代码生成器。

**步骤：**
1. `cargo clippy --manifest-path backend/Cargo.toml --all-targets` — 零 warning
2. `cargo test --manifest-path backend/Cargo.toml` — 全部通过
3. `cd web/crates/dioxus-i18n && cargo test` — 前端测试通过
4. 手动验证代码生成器输出（通过 API 或单元测试）
5. 验证 CI YAML 语法

**提交：** `test: Phase 3 最终验证（任务 5/5）`

---

## 执行顺序

```
任务 1 (CI/CD) ──────────────────────────────────────→ 任务 5 (验证)
                                                        ↑
任务 2 (后端模板) ──→ 任务 3 (前端模板) ──→ 任务 4 (文档) ─┘
```

任务 1 独立。任务 2-3 有依赖。任务 4 独立。任务 5 最后。
