---
name: docker-deploy
description: Deploy axum-dixous-admin full-stack via Docker — backend (Axum + MySQL + Redis) and frontend (Dioxus WASM + Nginx). Use when the user mentions "部署", "Docker", "docker build", "docker compose", "Nginx", "WASM构建", "发布", "上线", or when configuring deployment for the project.
---

# axum-dixous-admin Docker 部署

全栈 Docker 部署：后端 (Axum + MySQL + Redis + Adminer) + 前端 (Dioxus WASM + Nginx)。

## 架构总览

```
                    ┌─────────────────────────────────┐
                    │        backend/compose.yml       │
                    │  ┌────────┐ ┌───────┐ ┌───────┐ │
                    │  │ MySQL 8│ │Redis 7│ │Adminer│ │
                    │  └───┬────┘ └───┬───┘ └───────┘ │
                    │      │          │                │
                    │  ┌───┴──────────┴──────────┐     │
                    │  │  Backend (Axum :8888)   │     │
                    │  │  Dockerfile 两阶段构建   │     │
                    │  └─────────────────────────┘     │
                    └──────────────┬──────────────────┘
                                   │ :8888
                    ┌──────────────┴──────────────────┐
                    │       web/compose.yaml           │
                    │  ┌─────────────────────────┐     │
                    │  │  Nginx (:80)             │     │
                    │  │  ├─ / → SPA (WASM+JS)   │     │
                    │  │  └─ /api/ → proxy :8888 │     │
                    │  └─────────────────────────┘     │
                    └─────────────────────────────────┘
```

## 部署工作流

### 场景 A: 全栈部署 (推荐)

```bash
# 1. 启动后端 (MySQL + Redis + Adminer + Backend)
cd backend
docker compose up -d

# 2. 构建前端 WASM 产物
cd ../web
dx build --release

# 3. 启动前端 Nginx
docker compose up -d
```

### 场景 B: 仅后端

```bash
cd backend
docker compose up -d backend   # 自动拉起 MySQL + Redis
```

### 场景 C: 仅前端

```bash
cd web
dx build --release             # 先构建 WASM
docker compose up -d           # 启动 Nginx
```

## 关键文件

| 文件 | 作用 |
|------|------|
| `backend/compose.yml` | 后端编排: MySQL + Redis + Adminer + Backend |
| `backend/server/Dockerfile` | 后端两阶段构建: builder → runtime |
| `backend/server/.cargo/config.toml` | Cargo 镜像配置 (rsproxy.cn 加速) |
| `backend/server/config.yml` | 后端配置 (include_str! 编译期内嵌) |
| `web/compose.yaml` | 前端编排: Nginx |
| `web/nginx.conf` | Nginx 配置: 静态文件 + API 代理 + 缓存 |
| `web/Cargo.toml` | 前端依赖 + `[profile.release]` WASM 最小化 |

## 后端 Dockerfile 要点

### 1. 依赖缓存 (分层构建)

```dockerfile
# 先拷 .cargo/ + 各 Cargo.toml，创建占位源码编译依赖
COPY .cargo/ .cargo/
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
# ... 其他子 crate

# 占位源码 → 编译依赖 (Docker 缓存此层)
RUN mkdir -p api/src ... && echo "pub fn lib() {}" > ...
RUN cargo build --release --locked 2>/dev/null || true

# 拷入真实源码 → 只编译业务代码
COPY . .
RUN cargo build --release
```

### 2. 国内加速 (三个关键点)

| 加速项 | 配置 | 说明 |
|--------|------|------|
| Cargo crates | `.cargo/config.toml` → rsproxy.cn | 必须在编译前 COPY |
| Swagger UI | `ENV SWAGGER_UI_DOWNLOAD_URL=gh-proxy.com/...` | utoipa-swagger-ui build.rs |
| apt 包 | 默认 Debian 源 | 网络慢时可换 mirrors.aliyun.com |

### 3. config.yml 地址替换 (include_str! 内嵌)

```dockerfile
# config.yml 通过 include_str! 编译期内嵌，需 sed 替换为 Docker 服务名
RUN sed -i '0,/host: localhost/s/host: localhost/host: mysql/' config.yml && \
    sed -i 's/host: localhost/host: redis/g' config.yml
```

| 配置项 | 本地开发 | Docker 容器 |
|--------|---------|------------|
| `datasource.host` | `localhost` | `mysql` |
| `cache.host` | `localhost` | `redis` |
| `server.host` | `0.0.0.0` | `0.0.0.0` (不变) |

### 4. 运行时安全

- 非 root 用户 (`app`) 运行
- 健康检查: `curl -sf http://localhost:8888/health`
- 最小镜像: `debian:bookworm-slim` + `curl` + `ca-certificates`

## 前端构建要点

### 1. WASM 最小化 (web/Cargo.toml)

```toml
[profile.release]
opt-level = "z"       # 极致体积优化
lto = true             # 链接时优化
codegen-units = 1      # 单代码生成单元
panic = "abort"        # 移除 unwinding
strip = true           # 移除调试符号
```

### 2. 构建产物

```bash
dx build --release
# 产物: web/target/dx/web/release/web/public/
#   ├── index.html
#   └── assets/
#       ├── web-dx<hash>.js          (~50KB)
#       ├── web_bg-dx<hash>.wasm     (~1.1MB)
#       └── favicon-dx<hash>.ico
```

### 3. Nginx 配置要点

| 功能 | 配置 |
|------|------|
| WASM MIME | `location ~* \-dx[a-z0-9]+\.wasm$` → `application/wasm` |
| 长期缓存 | 带哈希资源 → `expires 1y; Cache-Control: immutable` |
| API 代理 | `/api/` → `proxy_pass http://host.docker.internal:8888` |
| SPA 回退 | `try_files $uri $uri/ /index.html` |
| gzip | JS/WASM/HTML/JSON 压缩 |
| 健康检查 | `/health` → `200 {"status":"ok"}` |

## 常见问题

### Q: Docker 构建时 cargo 下载很慢
**A**: 确认 `.cargo/config.toml` 在 Dockerfile 中被先于 `cargo build` 拷入。检查 `COPY .cargo/ .cargo/` 在 `COPY . .` 之前。

### Q: utoipa-swagger-ui 编译失败 (curl not found / 下载超时)
**A**: Dockerfile builder 阶段需安装 `curl`，并设置 `SWAGGER_UI_DOWNLOAD_URL` 环境变量为 `gh-proxy.com` 代理。

### Q: 后端容器连不上 MySQL/Redis
**A**: `config.yml` 通过 `include_str!` 编译期内嵌，需在 Dockerfile 中用 `sed` 将 `localhost` 替换为 Docker 服务名 (`mysql`/`redis`)。

### Q: 前端 WASM 文件太大
**A**: 确认 `web/Cargo.toml` 中 `[profile.release]` 配置了 `opt-level = "z"` + `lto = true` + `codegen-units = 1` + `panic = "abort"` + `strip = true`。

### Q: Nginx 容器连不上后端 (Linux)
**A**: `web/compose.yaml` 中已配置 `extra_hosts: host.docker.internal:host-gateway`，确保 Linux 兼容。

### Q: Docker build 时 Rust 版本不对
**A**: `Cargo.toml` 中 `rust-version = "1.96"`，Dockerfile 必须用 `rust:1.96-slim-bookworm`。

## 端口映射

| 服务 | 容器端口 | 宿主机端口 | 说明 |
|------|---------|-----------|------|
| MySQL | 3306 | 3306 | 数据库 |
| Redis | 6379 | 6379 | 缓存 |
| Adminer | 8080 | 8090 | 数据库管理 |
| Backend | 8888 | 8888 | API + Swagger UI |
| Nginx | 80 | 80 | 前端 + API 代理 |
