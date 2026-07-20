# 项目整体优化路线图设计文档

> 日期：2026-07-20
> 状态：📋 路线图已批准，待逐阶段实施
> 范围：axum-dixous-admin 全栈优化（前端 + 后端 + 微信小程序 + DevOps）
> 目标：提升可维护性 — 代码整洁、测试完善、开发效率高
> 组织方式：按维度纵向切分，6 个阶段递进实施

## 1. 背景与现状分析

### 1.1 项目概况

axum-dixous-admin 是一个全栈 Rust 后台管理系统：
- **后端**：Axum 0.8 + SeaORM 1 + Casbin 2 + MySQL 8 + Redis 7
- **前端**：Dioxus 0.7 + dioxus-element-plug 0.3 (Element Plus 组件)
- **微信小程序**：TypeScript 原生
- **代码生成器**：Rust CLI，YAML → 全栈 CRUD 代码

### 1.2 后端优化已完成（2026-07-20）

后端已完成 P1-P3 三个阶段的优化（详见 `2026-07-20-backend-optimization-design.md`）：
- P1：配置环境变量覆盖 + AppState 共享状态 + 统一错误层级
- P2：中间件链 + Redis JWT 黑名单 + 优雅关闭
- P3：Service Trait 抽象 + 测试套件（7 个 Mock 测试）

### 1.3 剩余问题分析

| # | 维度 | 问题 | 影响 |
|---|------|------|------|
| 1 | 前端代码质量 | `generator_manage.rs` 1355 行、44 个 Signal，单组件承担 9 个职责 | 难以维护、无法复用 |
| 2 | 后端遗留 | `db_conn!()` 宏定义但零调用，AuthLayer 仍用全局 Redis，`set_enforcer()` 全局注入 | 代码冗余、可测试性差 |
| 3 | 测试覆盖 | 仅 7 个 Mock 测试，无真实 DB 集成测试、无 API 端到端测试 | 重构无安全网 |
| 4 | 开发效率 | 无 CI/CD，无构建缓存分析，生成器功能有限 | 手动操作多、效率低 |
| 5 | 性能 | 前端 WASM 未优化，后端无热点缓存 | 加载慢、DB 压力大 |
| 6 | 安全 | CORS `very_permissive`，无限流，JWT 密钥在 config.yml | 不适合生产 |
| 7 | 微信小程序 | 功能少于 Web 端，代码质量待提升 | 用户体验不一致 |

## 2. 优化目标

- **代码整洁**：前端文件 < 400 行，后端无废弃代码，clippy 零 warning
- **测试完善**：后端测试覆盖率 > 60%，测试可独立运行，CI 自动化
- **开发效率高**：CI/CD 自动化，构建缓存优化，生成器增强
- **性能达标**：WASM 体积减小 30%+，热点缓存上线
- **安全就绪**：CORS 白名单、限流、输入校验、依赖审计
- **功能完整**：小程序功能对齐 Web 端核心功能

## 3. 路线图总览

### 3.1 阶段依赖

```
Phase 1: 代码质量基础
  │  前端大文件拆分 + 后端遗留清理 + 一致性规范
  ▼
Phase 2: 测试体系
  │  后端集成测试 + API 端到端 + 前端纯函数测试
  ▼
Phase 3: 开发效率
  │  CI/CD 流水线 + 构建缓存 + 生成器增强 + 文档
  ▼
Phase 4: 性能优化
  │  WASM 包体积 + 懒加载 + 后端热点缓存 + 查询优化
  ▼
Phase 5: 安全加固
  │  CORS 收紧 + 限流 + JWT 管理 + 输入校验 + 依赖审计
  ▼
Phase 6: 微信小程序
     功能对齐 + 代码质量 + UI 打磨
```

### 3.2 各阶段预估

| 阶段 | 主题 | 预估工作量 | 核心产出 |
|------|------|-----------|----------|
| Phase 1 | 代码质量基础 | 2-3 天 | 前端文件全部 < 400 行，后端无 deprecated 代码 |
| Phase 2 | 测试体系 | 2-3 天 | 后端测试覆盖率 > 60%，测试可独立运行 |
| Phase 3 | 开发效率 | 2 天 | CI/CD 自动化，生成器增强 |
| Phase 4 | 性能优化 | 2-3 天 | WASM 体积减小 30%+，热点缓存上线 |
| Phase 5 | 安全加固 | 2 天 | 限流 + CORS + 校验 + 依赖审计 |
| Phase 6 | 微信小程序 | 2-3 天 | 功能对齐 Web 端核心功能 |

### 3.3 依赖关系说明

- **Phase 1 → Phase 2**：代码整洁后才能写有意义的测试（测试稳定代码，而非正在大改的代码）
- **Phase 2 → Phase 3**：CI/CD 需要测试体系就绪才能发挥价值
- **Phase 3 → Phase 4/5**：性能和安全优化需要 CI 保障不回退
- **Phase 4 ↔ Phase 5**：可适当并行，但安全建议在性能之后（先跑对，再跑快，再锁安全）
- **Phase 6 独立**：小程序可任何时候开始，放最后是因为它的优化不影响主项目

## 4. Phase 1 — 代码质量基础

### 4.1 前端大文件拆分

**核心问题：** `generator_manage.rs` 有 1355 行、44 个 Signal，一个组件承担了 9 个职责。

**拆分方案：** 按职责拆为 1 个主页面 + 7 个子组件：

```
components/
├── generator_manage.rs          # 主页面：编排 + 状态管理 (~200 行)
└── generator/                   # 新建子目录
    ├── mod.rs                   # 模块导出
    ├── config_form.rs           # 模块配置卡片
    ├── field_list.rs            # 字段列表表格
    ├── field_edit_dialog.rs     # 字段编辑对话框
    ├── code_preview_dialog.rs   # 代码预览对话框
    ├── template_dialog.rs       # 字段模板对话框
    └── db_import_dialog.rs      # 从数据库创建对话框
```

**子组件通信模式：** 使用 Dioxus `Signal` props 传递状态，父组件持有所有 Signal，子组件通过 props 读写。配置预览对话框等纯展示组件只接收只读 props。

**其他文件评估：**
- `menu_item.rs` (471 行) — 递归菜单树，逻辑紧凑，暂不拆分
- `user_manage.rs` (429 行) — 临界值，暂保留，后续视情况拆分

### 4.2 后端遗留清理

| # | 问题 | 现状 | 改造 |
|---|------|------|------|
| 1 | `db_conn!()` 宏 | 定义在 `utils/src/lib.rs`，全项目零调用（后端优化已全部迁移） | 直接删除宏定义 |
| 2 | AuthLayer Redis 全局访问 | `middleware.rs:103` 调用 `DB::redis_connection()` 全局获取 | AuthLayer 构造器注入 Redis `ConnectionManager`，与 enforcer 一致 |
| 3 | `service::enforcer::set_enforcer()` 全局注入 | `main.rs:33` 全局设置 + `casbin_service` 通过全局获取 | casbin_service 改为通过参数接收 enforcer，移除全局注入 |
| 4 | `sys_user_service` 微信配置 | `sys_user_service.rs:227-228` 直接读 `CONFIG.wechat` | 通过参数传入 wechat 配置，提升可测试性 |

**保留不动的全局变量（合理使用）：**
- `CONFIG` — 仅在启动期（`main.rs`、`db.rs`、`migration/lib.rs`、`level.rs`）使用，启动后通过 AppState 传递
- `DB::db_connection()` / `DB::redis_connection()` — 仅在 `main.rs` 启动期调用一次，创建 AppState

### 4.3 代码一致性规范

- `cargo clippy` 全量修复（零 warning 目标）
- `cargo fmt` 统一格式化
- 后端 Service 函数签名统一：`pub async fn xxx(db: &DatabaseConnection, ...) -> Result<T, ServiceError>`
- 前端组件统一：`#[component] pub fn Xxx() -> Element` + Signal 状态管理

### 4.4 Phase 1 验收标准

- [ ] `generator_manage.rs` 主文件 < 250 行，子组件各 < 300 行
- [ ] `db_conn!()` 宏已删除，编译通过
- [ ] AuthLayer 通过构造器接收 Redis，无全局访问
- [ ] `service::enforcer::set_enforcer()` 已移除
- [ ] `cargo clippy` 零 warning
- [ ] 现有 7 个测试全部通过

### 4.5 Phase 1 文件变更清单

| 操作 | 文件 |
|------|------|
| 删除 | `utils/src/lib.rs` — 移除 `db_conn!()` 宏 |
| 改造 | `auth-layer/src/middleware.rs` — AuthLayer 构造器注入 Redis |
| 改造 | `gateway/src/main.rs` — 传入 Redis 给 AuthLayer |
| 改造 | `service/src/enforcer.rs` — 移除 `set_enforcer()` |
| 改造 | `service/src/casbin_service.rs` — 通过参数接收 enforcer |
| 改造 | `service/src/sys_user_service.rs` — 微信配置参数化 |
| 改造 | `api/src/user_api.rs` — 传入 wechat 配置 |
| 新增 | `web/src/components/generator/mod.rs` |
| 新增 | `web/src/components/generator/config_form.rs` |
| 新增 | `web/src/components/generator/field_list.rs` |
| 新增 | `web/src/components/generator/field_edit_dialog.rs` |
| 新增 | `web/src/components/generator/code_preview_dialog.rs` |
| 新增 | `web/src/components/generator/template_dialog.rs` |
| 新增 | `web/src/components/generator/db_import_dialog.rs` |
| 改造 | `web/src/components/generator_manage.rs` — 精简为编排逻辑 |
| 改造 | `web/src/components/mod.rs` — 注册 generator 子模块 |

## 5. Phase 2 — 测试体系

### 5.1 测试分层

```
tests/
├── api_tests.rs              # 现有 Mock 测试（保留）
├── common/
│   └── mod.rs                # 测试辅助：创建 AppState、测试 DB 初始化、fixtures
├── integration/
│   ├── user_service_test.rs  # 真实 DB Service 测试
│   ├── role_service_test.rs
│   ├── menu_service_test.rs
│   └── casbin_test.rs        # 权限规则测试
└── api/
    ├── user_api_test.rs       # HTTP 路由测试 (tower::ServiceExt::oneshot)
    └── auth_api_test.rs       # 登录/登出/JWT 黑名单测试
```

**测试分层说明：**

| 层级 | 名称 | 说明 | 工具 |
|------|------|------|------|
| L1 | 单元测试 | 纯函数测试（如 `build_page_query`、DTO 序列化） | `#[test]` |
| L2 | Mock 测试 | Trait + Mock 实现，不碰 DB | 现有的 7 个测试 |
| L3 | 集成测试 | 真实 DB + Service 层调用 | SeaORM + 测试数据库 |
| L4 | HTTP 路由测试 | `Router::oneshot(request)` 完整路由链（含中间件） | `tower::ServiceExt` |

> **注：** 本项目是纯 Rust 全栈应用，前端编译为 WASM 客户端渲染，无需浏览器自动化测试。L4 的 `tower::ServiceExt::oneshot` 在内存中模拟 HTTP 请求，不启动真实服务器，属于 HTTP 层集成测试，不是 E2E 测试。

### 5.2 测试 DB 策略

独立测试数据库 `scm_test`：
- 通过 `ADMIN_DB_DATABASE=scm_test` 环境变量指定
- 每个测试前 `Migrator::fresh()` 重建表结构 + 插入种子数据
- 与开发环境共用 MySQL 实例

### 5.3 测试辅助模块

`common/mod.rs` 提供：
- `create_test_state()` — 构建完整 AppState（含 DB + Redis + Enforcer）
- `insert_test_user()` — 插入测试用户 fixtures
- `seed_test_data()` — 批量插入种子数据

### 5.4 优先测试覆盖

| 优先级 | 模块 | 测试内容 | 类型 |
|--------|------|----------|------|
| P0 | 用户登录 | 正确密码/错误密码/禁用用户 | 集成 |
| P0 | JWT 黑名单 | 登出后 token 失效 | 集成 |
| P0 | CRUD 基础 | User/Role/Menu 的增删改查 | 集成 |
| P1 | Casbin 权限 | 有权限/无权限/路由匹配 | 集成 |
| P1 | 分页查询 | 关键词搜索/分页边界 | 集成 |
| P2 | HTTP 路由 | 完整 HTTP 请求→响应（含中间件链） | 路由测试 |
| P2 | 错误转换 | ServiceError → HTTP 状态码 | 路由测试 |

### 5.5 Mock 测试扩展

为 RoleService、MenuService、ApiService 各添加 Mock 测试，与现有 MockUserService 模式一致。

### 5.6 前端测试策略

Dioxus 0.7 前端测试能力有限，采用务实策略：

| 策略 | 内容 | 可行性 |
|------|------|--------|
| API 层逻辑测试 | 测试 `build_page_query`、URL 构建等纯函数 | 高 |
| Model 序列化测试 | 测试 DTO 的 serde 序列化/反序列化 | 高 |
| 组件渲染测试 | Dioxus 0.7 尚无成熟方案 | 暂不做 |

> **不做 E2E 测试：** 本项目前端为 Dioxus 0.7 WASM 客户端渲染，无成熟浏览器自动化测试方案。投入产出比低，不纳入计划。

前端测试目录：
- `web/src/http/mod.rs` — 新增 `#[cfg(test)]` 模块，测试 `build_page_query` 等
- `web/src/models/*.rs` — 新增 `#[cfg(test)]` 模块，测试序列化

### 5.7 Phase 2 验收标准

- [ ] 后端集成测试覆盖 User/Role/Menu CRUD 核心路径
- [ ] JWT 黑名单登出测试通过
- [ ] Casbin 权限规则测试通过
- [ ] HTTP 路由测试至少覆盖登录 + 一个 CRUD 操作（`tower::ServiceExt::oneshot`）
- [ ] Mock 测试扩展到 4 个 Service（User/Role/Menu/Api）
- [ ] 前端纯函数测试覆盖 HTTP 层和 Model 序列化
- [ ] `cargo test` 全量通过，测试数 > 20

## 6. Phase 3 — 开发效率

### 6.1 CI/CD 流水线

- PR 触发：`cargo clippy` + `cargo test` + `cargo build`
- 合并触发：Docker 镜像构建 + 推送
- 前端：`dx build --release` + WASM 体积检查
- 依赖缓存：`Swatinem/rust-cache` 加速构建

### 6.2 构建优化

- 后端：Cargo 依赖缓存分层（Dockerfile 已有，检查增量构建效率）
- 前端：WASM 构建产物分析，识别大依赖
- 开发热重载：检查 `dx serve` 和 `cargo run` 的反馈速度

### 6.3 代码生成器增强

当前生成器只支持基础 CRUD，增强为：
- 支持关联关系（一对多/多对多）
- 支持自定义模板
- 生成测试代码骨架
- 预览界面优化（Phase 1 拆分后更易扩展）

### 6.4 文档完善

- API 文档：确保 utoipa 注解完整，Swagger UI 可用
- 开发文档：更新 AGENTS.md 反映优化后的架构
- 部署文档：Docker 部署指南更新

### 6.5 Phase 3 验收标准

- [ ] CI/CD 自动化运行，PR 检查 clippy + test + build
- [ ] `cargo test` 在 CI 中通过（含 MySQL + Redis 服务）
- [ ] 生成器支持关联关系生成
- [ ] AGENTS.md 更新反映优化后架构

## 7. Phase 4 — 性能优化

### 7.1 前端性能

- WASM 包体积分析（`twiggy` 工具），减小 30%+ 目标
- 路由级懒加载（Dioxus 0.7 动态组件）
- 首屏加载优化（骨架屏、延迟加载非关键组件）

### 7.2 后端热点缓存

| 缓存对象 | Redis Key | TTL | 失效时机 |
|----------|-----------|-----|----------|
| 用户菜单 | `user_menus:{username}` | 1h | 菜单/角色变更时删除 |
| 字典数据 | `dict:{type}` | 2h | 字典修改时删除 |
| 用户权限策略 | `user_perms:{username}` | 1h | 角色权限变更时删除 |

缓存模式：Cache-Aside（先查 Redis，miss 则查 DB 并回填）。

### 7.3 数据库查询优化

- 审查 N+1 查询（特别是菜单树构建、用户角色关联查询）
- 添加必要索引（检查迁移脚本）
- 慢查询日志监控

### 7.4 Phase 4 验收标准

- [ ] WASM 体积减小 30%+
- [ ] 热点缓存命中率 > 80%
- [ ] 无 N+1 查询
- [ ] Dashboard 响应时间 < 100ms

## 8. Phase 5 — 安全加固

### 8.1 中间件安全

- CORS：从 `very_permissive` 收紧为白名单模式
- 限流：`tower-governor`，登录接口 5 次/分钟，全局 100 次/分钟
- 请求体大小限制

### 8.2 认证安全

- JWT 密钥：强制环境变量注入（`ADMIN_JWT_SECRET`），`config.yml` 中移除明文
- 密码策略：最小长度、复杂度校验
- Token 刷新机制（可选）

### 8.3 输入校验

- 后端 DTO 添加 `validator` 校验（邮箱格式、手机号、字符串长度）
- 前端表单校验对齐后端规则

### 8.4 依赖审计

- `cargo audit` 检查已知漏洞
- 定期更新依赖版本

### 8.5 Phase 5 验收标准

- [ ] CORS 白名单生效，非白名单来源被拒绝
- [ ] 限流中间件工作，超限返回 429
- [ ] JWT 密钥通过环境变量注入
- [ ] `cargo audit` 无高危漏洞
- [ ] 后端 DTO 校验覆盖所有用户输入字段

## 9. Phase 6 — 微信小程序

### 9.1 功能对齐

- 补齐 Web 端核心功能：角色管理、菜单管理、字典管理
- 代码生成器移动端适配（简化版）

### 9.2 代码质量

- 统一请求封装（对齐 Web 端 `http/mod.rs` 模式）
- TypeScript 类型完善
- 错误处理统一

### 9.3 UI 打磨

- 适配暗色模式
- 交互优化（加载状态、错误提示）

### 9.4 Phase 6 验收标准

- [ ] 小程序功能覆盖 Web 端核心 CRUD
- [ ] 请求封装统一，TypeScript 类型完整
- [ ] 暗色模式适配完成

## 10. 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| Phase 1 前端拆分引入 bug | 拆分后逐组件手动验证，Phase 2 测试兜底 |
| Phase 2 测试 DB 与开发 DB 冲突 | 使用独立数据库名 `scm_test` |
| Phase 4 缓存一致性问题 | Cache-Aside 模式 + 变更时主动删除缓存 |
| Phase 5 CORS 收紧影响开发 | 开发环境保留宽松配置，仅生产环境收紧 |
| Phase 6 小程序审核风险 | 提前了解微信审核规则 |

## 11. 实施策略

- **逐阶段实施**：每个阶段独立设计 → 计划 → 实现 → 验证
- **每阶段产出独立规格文档**：`docs/superpowers/specs/YYYY-MM-DD-phase{N}-design.md`
- **每阶段产出独立实现计划**：`docs/superpowers/plans/YYYY-MM-DD-phase{N}-plan.md`
- **阶段间不跳步**：前阶段验收通过后再开始下阶段
- **详细度说明**：Phase 1 和 Phase 2 已有足够细节，可直接进入实现计划；Phase 3-6 为概要级别，将在各自阶段启动时细化为详细规格

## 12. 不做的事情（YAGNI）

- 不引入 GraphQL（REST + Swagger 足够）
- 不引入微服务架构（单体应用满足需求）
- 不引入消息队列（当前无异步任务需求）
- 不引入 ELK 日志栈（tracing + 文件日志足够）
- 不引入 Kubernetes（Docker Compose 足够）
- 不做前端 SSR/SSG（WASM 客户端渲染足够）
- 不做 APM 性能监控（tracing + 基础指标足够）
