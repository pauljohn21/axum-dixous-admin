# Phase 3 — 开发效率详细设计

> 日期：2026-07-20
> 状态：📋 设计完成，待实施
> 前置条件：Phase 1（代码质量）、Phase 2（测试体系）已完成

## 1. 现状分析

### 1.1 CI/CD 现状

- **无 CI/CD 配置**：项目中无 `.github/workflows/` 或 `.gitee/workflows/`
- 所有检查（clippy、test、build）依赖手动执行
- 无自动化构建和部署流程

### 1.2 代码生成器现状

| 文件 | 状态 |
|------|------|
| `backend/server/service/src/generator_code_service.rs` | **占位代码** — `preview_code` 只返回 TODO 注释 |
| `backend/server/service/src/generator_history_service.rs` | 已实现 — 历史 CRUD + 回滚 + 数据库导入 |
| `backend/server/api/src/generator_api.rs` | 已实现 — 路由和 OpenAPI 注解完整 |
| `web/src/components/generator_manage.rs` | 已实现 — Phase 1 拆分为 1 主 + 7 子组件 |
| `web/src/models/generator.rs` | 已实现 — 完整的 GeneratorConfig / GeneratorField 模型 |

**核心缺口**：`GeneratorCodeService::preview_code` 未实现实际代码生成逻辑，只返回占位内容。

### 1.3 构建配置现状

| 配置 | 状态 |
|------|------|
| 后端 `[profile.release]` | ✅ 已优化 (lto, codegen-units=1, opt-level="z", panic=abort, strip) |
| 前端 `[profile.release]` | ✅ 已优化 (opt-level="z", lto, codegen-units=1, panic=abort, strip) |
| Dockerfile 两阶段构建 | ✅ 依赖缓存 + 非 root + healthcheck |
| `.cargo/config.toml` 镜像 | ✅ rsproxy.cn 国内加速 |

### 1.4 文档现状

| 文档 | 状态 |
|------|------|
| `AGENTS.md` | 需更新 — 未反映 Phase 1-2 优化成果（测试体系、全局状态清除等） |
| `.catpaw/skills/axum-crud/SKILL.md` | 已有 — CRUD 模块生成器 skill |
| `.catpaw/skills/docker-deploy/SKILL.md` | 已有 — Docker 部署指南 |

## 2. 目标

1. **CI/CD 自动化**：PR 自动运行 clippy + test + build，保障代码质量不回退
2. **生成器增强**：实现完整的代码生成逻辑，从占位代码升级为可生成实际可用的 CRUD 代码骨架
3. **文档更新**：AGENTS.md 反映 Phase 1-2 优化后的架构

## 3. CI/CD 流水线设计

### 3.1 触发条件

```yaml
on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]
```

### 3.2 Job 结构

```
ci (workflow)
├── backend-check    — 后端 clippy + test + build
├── frontend-check   — 前端 clippy + test + build
└── docker-build     — 仅 push to main 时触发
```

### 3.3 后端检查 Job

```yaml
backend-check:
  runs-on: ubuntu-latest
  services:
    mysql:
      image: mysql:8
      env:
        MYSQL_ROOT_PASSWORD: root123456
        MYSQL_DATABASE: scm_test
      ports: ['3306:3306']
      options: >-
        --health-cmd="mysqladmin ping -h localhost"
        --health-interval=10s
        --health-timeout=5s
        --health-retries=5
    redis:
      image: redis:7
      ports: ['6379:6379']
      options: >-
        --health-cmd="redis-cli ping"
        --health-interval=10s
        --health-timeout=5s
        --health-retries=5
  steps:
    - uses: actions/checkout@v4
      with:
        submodules: true
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
      with:
        workspaces: backend
    - name: Install cargo components
      run: rustup component add clippy
    - name: Clippy
      run: cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings
    - name: Test
      run: cargo test --manifest-path backend/Cargo.toml
      env:
        ADMIN_DB_HOST: localhost
        ADMIN_DB_DATABASE: scm_test
        ADMIN_DB_USERNAME: root
        ADMIN_DB_PASSWORD: root123456
        ADMIN_REDIS_HOST: localhost
    - name: Build
      run: cargo build --manifest-path backend/Cargo.toml --release
```

### 3.4 前端检查 Job

```yaml
frontend-check:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
      with:
        submodules: true
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    - name: Install dioxus CLI
      run: cargo install dioxus-cli --version 0.7.1
    - uses: Swatinem/rust-cache@v2
      with:
        workspaces: web
    - name: Test
      run: cargo test --manifest-path web/crates/dioxus-i18n/Cargo.toml
    - name: Build
      run: cd web && dx build --release
```

### 3.5 Docker 构建 Job

```yaml
docker-build:
  needs: [backend-check, frontend-check]
  if: github.ref == 'refs/heads/main'
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
      with:
        submodules: true
    - name: Build backend image
      run: docker build -t axum-admin-backend -f backend/server/Dockerfile backend/server/
```

## 4. 代码生成器增强设计

### 4.1 当前问题

`GeneratorCodeService::preview_code` 只返回占位内容：
```rust
content: "// Migration code will be generated here\n// TODO: Implement full code generation"
```

### 4.2 增强方案

实现基于 `GeneratorConfig` 的完整代码生成，生成以下文件：

**后端（5 个文件）：**

| 文件 | 说明 |
|------|------|
| `m{timestamp}_create_{table}.rs` | SeaORM 迁移脚本 |
| `{resource}.rs` (dao) | SeaORM 实体 |
| `{resource}_dto.rs` | 请求/响应 DTO |
| `{resource}_service.rs` | Service 层 CRUD |
| `{resource}_api.rs` | API 路由 + OpenAPI |

**前端（3 个文件）：**

| 文件 | 说明 |
|------|------|
| `{resource}.rs` (model) | 前端数据模型 |
| `{resource}.rs` (api) | API 调用封装 |
| `{resource}_manage.rs` | 管理页面组件骨架 |

### 4.3 代码生成器实现

在 `generator_code_service.rs` 中实现模板生成函数：

```rust
impl GeneratorCodeService {
    pub async fn preview_code(data: PreviewCodeDTO) -> Result<PreviewCodeResponse, ServiceError> {
        let config: GeneratorConfig = serde_json::from_str(&data.config_json)
            .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

        let mut backend_files = Vec::new();
        let mut frontend_files = Vec::new();

        if config.generate_backend {
            backend_files.push(Self::gen_migration(&config));
            backend_files.push(Self::gen_dao(&config));
            backend_files.push(Self::gen_dto(&config));
            backend_files.push(Self::gen_service(&config));
            backend_files.push(Self::gen_api(&config));
        }

        if config.generate_frontend {
            frontend_files.push(Self::gen_model(&config));
            frontend_files.push(Self::gen_api_call(&config));
            frontend_files.push(Self::gen_component(&config));
        }

        Ok(PreviewCodeResponse { backend_files, frontend_files })
    }
}
```

### 4.4 类型映射

| GeneratorConfig 字段类型 | Rust 类型 | SeaORM 列类型 |
|--------------------------|-----------|---------------|
| string | String | string_null / string |
| text | String | text_null / text |
| i8 | i8 | tiny_integer_null |
| i32 | i32 | integer_null |
| i64 | i64 | big_integer_null |
| u64 | u64 | big_unsigned_null |
| f32 | f32 | float_null |
| f64 | f64 | double_null |
| bool | bool | boolean_null |
| date | String | date_null |
| datetime | String | timestamp_with_time_zone_null |
| json | serde_json::Value | json_null |

### 4.5 生成示例

以 `sys_category` 表（字段: id, name, sort_order, enable）为例，生成的 DAO：

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema, Eq)]
#[sea_orm(table_name = "sys_category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub sort_order: Option<i32>,
    pub enable: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

## 5. 文档更新

### 5.1 AGENTS.md 更新内容

- 添加测试体系部分（Phase 2 成果）
- 更新「添加新模块步骤」反映 Service 参数注入
- 更新「关键约定」反映全局状态清除
- 添加 CI/CD 说明

### 5.2 新增文档

- `docs/development.md` — 开发指南（环境搭建、测试运行、代码生成器使用）

## 6. 验收标准

- [ ] GitHub Actions CI 自动运行 clippy + test + build
- [ ] CI 在 PR 上显示检查状态
- [ ] 代码生成器能生成 8 个文件的完整内容
- [ ] 生成的代码可通过编译（手动验证）
- [ ] AGENTS.md 更新反映 Phase 1-2 优化成果
- [ ] 开发文档可用
