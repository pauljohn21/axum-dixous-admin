# Phase 5 — 安全加固实现计划

> 日期：2026-07-20
> 设计文档：`docs/superpowers/specs/2026-07-20-phase5-security-design.md`
> 前置条件：Phase 1-4 已完成

## 任务清单

### 任务 1：CORS 收紧 + 请求体限制

**目标：** CORS 从 `very_permissive` 改为环境变量驱动的白名单模式，添加请求体大小限制。

**步骤：**
1. 在 `gateway/src/main.rs` 实现 `build_cors_layer()` 函数
2. 替换 `CorsLayer::very_permissive()` 为 `build_cors_layer()`
3. 添加 `DefaultBodyLimit::max(2MB)` 中间件
4. 在 `config.rs` 添加 `ADMIN_CORS_ORIGINS` 环境变量读取（可选，也可直接在 main.rs 读取）

**验证：** `cargo build` 编译通过

**提交：** `security: CORS 收紧为白名单模式 + 请求体大小限制（任务 1/5）`

---

### 任务 2：限流中间件

**目标：** 添加全局限流 100 次/分钟，登录/注册限流 5 次/分钟。

**步骤：**
1. 在 workspace `Cargo.toml` 添加 `tower-governor` 依赖
2. 在 `gateway/Cargo.toml` 引入 `tower-governor`
3. 在 `main.rs` 配置全局限流
4. 将登录/注册路由单独分组，应用更严格限流

**验证：** `cargo build` 编译通过

**提交：** `security: 添加 tower-governor 限流中间件（任务 2/5）`

---

### 任务 3：JWT 密钥安全 + 输入校验

**目标：** JWT 密钥从 config.yml 移除明文，添加用户输入校验。

**步骤：**
1. 修改 `config.yml`：jwt.secret 改为空字符串
2. 在 `main.rs` 添加启动时 JWT 密钥检查警告
3. 在 `SysUserService` 添加 `validate_username/password/email/phone` 函数
4. 在 `insert` 和 `login` 方法中调用校验

**验证：** `cargo test` 通过，clippy 零 warning

**提交：** `security: JWT密钥环境变量注入 + 用户输入校验（任务 3/5）`

---

### 任务 4：CI 集成 cargo audit

**目标：** CI 流水线添加依赖审计步骤。

**步骤：**
1. 在 `.github/workflows/ci.yml` 的 `backend-check` job 添加 cargo-audit 安装和运行

**验证：** YAML 语法正确

**提交：** `security: CI 集成 cargo audit 依赖审计（任务 4/5）`

---

### 任务 5：最终验证

**目标：** 全量验证安全加固。

**步骤：**
1. `cargo clippy --all-targets -- -D warnings`
2. `cargo test`
3. `cd web/crates/dioxus-i18n && cargo test`

**提交：** `test: Phase 5 最终验证（任务 5/5）`

---

## 执行顺序

```
任务 1 (CORS+Body限制) ──→ 任务 5 (验证)
任务 2 (限流) ────────────→
任务 3 (JWT+校验) ────────→
任务 4 (CI audit) ────────→
```

任务 1-4 互相独立，可按顺序执行。任务 5 最后。
