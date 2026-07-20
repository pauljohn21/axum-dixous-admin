# Phase 5 — 安全加固详细设计

> 日期：2026-07-20
> 状态：📋 设计完成，待实施
> 前置条件：Phase 1-4 已完成

## 1. 现状分析

### 1.1 CORS 配置

**当前**: `CorsLayer::very_permissive()` — 允许所有来源、所有方法、所有请求头
**风险**: 任意网站可发起跨域请求，存在 CSRF 风险

### 1.2 限流

**当前**: 无任何限流中间件
**风险**: 暴力破解登录接口、DDoS 攻击

### 1.3 JWT 密钥

**当前**: `config.yml` 中明文存储 `"your-256-bit-secret-key-change-in-production"`
**风险**: 密钥泄露在代码仓库中

### 1.4 输入校验

**当前**: DTO 无任何校验（`SysUserInsertDTO` 的 username/password/phone/email 无格式验证）
**风险**: SQL 注入（SeaORM 已参数化，风险低）、XSS（前端需防范）、垃圾数据

### 1.5 依赖审计

**当前**: 无 `cargo audit` 集成
**风险**: 使用含已知漏洞的依赖

## 2. 目标

1. **CORS 白名单**: 开发环境宽松，生产环境白名单
2. **限流中间件**: 登录 5 次/分钟，全局 100 次/分钟
3. **JWT 密钥安全**: 通过环境变量注入，config.yml 移除明文
4. **输入校验**: 用户注册/更新添加邮箱、手机号、密码长度校验
5. **依赖审计**: CI 集成 `cargo audit`

## 3. CORS 收紧设计

### 3.1 方案

通过环境变量 `ADMIN_CORS_ORIGINS` 配置允许的来源（逗号分隔），未配置时默认开发环境宽松。

```rust
fn build_cors_layer() -> CorsLayer {
    let origins: Vec<&str> = std::env::var("ADMIN_CORS_ORIGINS")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .collect();

    if origins.is_empty() {
        // 开发环境：宽松配置
        CorsLayer::very_permissive()
    } else {
        // 生产环境：白名单
        let allowed: Vec<HeaderValue> = origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(allowed)
            .allow_methods([
                Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS,
            ])
            .allow_headers([
                header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT,
            ])
            .allow_credentials(true)
            .max_age(Duration::from_secs(3600))
    }
}
```

### 3.2 配置方式

```bash
# 生产环境
export ADMIN_CORS_ORIGINS="https://admin.example.com,https://app.example.com"

# 开发环境（不设置 = 宽松）
# 无需设置
```

## 4. 限流中间件设计

### 4.1 方案

使用 `tower-governor` crate 实现基于 IP 的限流。

| 路由 | 限流 |
|------|------|
| `/api/user/login` | 5 次/分钟 |
| `/api/user/register` | 3 次/分钟 |
| 全局其他路由 | 100 次/分钟 |

### 4.2 实现

在 `gateway/Cargo.toml` 添加 `tower-governor` 依赖：

```toml
tower-governor = "0.4"
```

在 `main.rs` 中配置：

```rust
use tower_governor::{GovernorLayer, GovernorConfig};

// 全局限流: 100 次/分钟
let global_limiter = GovernorConfig {
    per_second: 100,
    burst_size: 100,
    ..Default::default()
};

// 登录限流: 5 次/分钟（通过单独路由层应用）
let login_limiter = GovernorConfig {
    per_second: 5,
    burst_size: 5,
    ..Default::default()
};
```

由于 `tower-governor` 的限流是按路由组应用的，需要将登录/注册路由单独分组并应用更严格的限流。

## 5. JWT 密钥安全设计

### 5.1 方案

- `config.yml` 中的 `jwt.secret` 改为占位符 `""`（空字符串）
- 启动时检查 `ADMIN_JWT_SECRET` 环境变量，若为空则使用 config.yml 的值（兼容开发环境）
- 日志中不输出密钥值

### 5.2 变更

`config.yml`:
```yaml
jwt:
  secret: ""  # 通过 ADMIN_JWT_SECRET 环境变量设置
  expire_hours: 24
```

`config.rs` 的 `load()` 方法已有环境变量覆盖逻辑，无需修改代码。只需更新 `config.yml` 和 Dockerfile。

### 5.3 启动校验

在 `main.rs` 添加启动时检查：

```rust
if CONFIG.jwt.secret.is_empty() {
    tracing::warn!("JWT 密钥未设置！请配置 ADMIN_JWT_SECRET 环境变量");
}
```

## 6. 输入校验设计

### 6.1 方案

在 Service 层添加校验逻辑（不引入 `validator` crate，保持依赖简洁）。

### 6.2 校验规则

| 字段 | 规则 |
|------|------|
| username | 3-20 字符，仅字母数字下划线 |
| password | 最少 6 字符 |
| email | 基本格式校验（含 `@`） |
| phone | 11 位数字（中国手机号） |

### 6.3 实现

在 `SysUserService::insert` 和 `login` 中添加校验：

```rust
fn validate_username(username: &str) -> Result<(), ServiceError> {
    if username.len() < 3 || username.len() > 20 {
        return Err(ServiceError::BadRequest("用户名长度需 3-20 字符".into()));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ServiceError::BadRequest("用户名仅允许字母数字下划线".into()));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ServiceError> {
    if password.len() < 6 {
        return Err(ServiceError::BadRequest("密码至少 6 位".into()));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), ServiceError> {
    if !email.is_empty() && !email.contains('@') {
        return Err(ServiceError::BadRequest("邮箱格式不正确".into()));
    }
    Ok(())
}

fn validate_phone(phone: &str) -> Result<(), ServiceError> {
    if !phone.is_empty() && (phone.len() != 11 || !phone.chars().all(|c| c.is_ascii_digit())) {
        return Err(ServiceError::BadRequest("手机号格式不正确".into()));
    }
    Ok(())
}
```

## 7. 依赖审计设计

### 7.1 CI 集成

在 `.github/workflows/ci.yml` 的 `backend-check` job 中添加：

```yaml
- name: Install cargo-audit
  run: cargo install cargo-audit || true
- name: Audit dependencies
  run: cargo audit --deny warnings
```

### 7.2 本地使用

```bash
cargo install cargo-audit
cargo audit
```

## 8. 请求体大小限制

### 8.1 方案

在 `main.rs` 添加 `DefaultBodyLimit`：

```rust
use axum::extract::DefaultBodyLimit;

// 限制请求体 2MB
.layer(DefaultBodyLimit::max(2 * 1024 * 1024))
```

## 9. 验收标准

- [ ] CORS 白名单模式：生产环境通过 `ADMIN_CORS_ORIGINS` 配置
- [ ] 限流中间件：登录 5 次/分钟，全局 100 次/分钟
- [ ] JWT 密钥：config.yml 移除明文，通过环境变量注入
- [ ] 输入校验：username/password/email/phone 校验生效
- [ ] 请求体大小限制：2MB
- [ ] CI 集成 `cargo audit`
- [ ] `cargo clippy` 零 warning
- [ ] `cargo test` 全部通过
