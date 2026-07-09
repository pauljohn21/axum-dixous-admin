# AGENTS.md — axum-dixous-admin

> 全栈 Rust 后台管理系统：后端 Axum + SeaORM + Casbin + MySQL，前端 Dioxus 0.7 + Element Plus 组件，微信小程序 TypeScript 原生。

## 项目概览

```
axum-dixous-admin/
├── backend/                  # 后端工作空间
│   ├── compose.yml           # Docker Compose (MySQL 8 + Redis 7 + Adminer + Backend)
│   ├── Cargo.toml            # workspace 根 (含 [profile.release] 体积优化)
│   ├── mysql.sql             # 数据库初始 SQL
│   └── server/
│       ├── config.yml        # 后端配置 (编译时 include_str! 内嵌)
│       ├── Dockerfile        # 两阶段构建 (依赖缓存 + 非 root + healthcheck)
│       ├── .cargo/config.toml # Cargo 镜像配置 (rsproxy.cn 国内加速)
│       ├── .dockerignore
│       ├── gateway/          # 二进制入口 (main.rs)
│       ├── api/              # HTTP 路由 + OpenAPI 定义
│       ├── service/          # 业务逻辑层
│       ├── auth-layer/       # JWT 验证 + Casbin 鉴权中间件
│       ├── casbin-adapter/   # SeaORM Casbin 适配器
│       ├── casbin/           # rbac_model.conf
│       ├── data/
│       │   ├── migration/    # SeaORM 迁移脚本
│       │   └── model/        # 实体 (dao) + DTO (dto)
│       ├── utils/            # 配置、DB、JWT、密码、日志等工具
│       └── shell/            # sea-orm-cli 辅助脚本
├── web/                      # 前端 Dioxus 工作空间 (独立 git 仓库)
│   ├── compose.yaml          # Docker Compose (Nginx)
│   ├── nginx.conf            # Nginx 配置 (静态文件 + API 代理 + 缓存)
│   ├── Cargo.toml            # 含 [profile.release] WASM 最小化配置
│   ├── Dioxus.toml
│   ├── crates/
│   │   ├── route-guard/      # 路由守卫库 (类似 Vue Router beforeEach)
│   │   ├── element-icons/    # Element Plus 图标 (137+)
│   │   ├── dioxus-i18n/      # 国际化库
│   │   └── slider-captcha/   # 滑块验证码组件
│   ├── src/
│   │   ├── main.rs           # 入口
│   │   ├── app/              # 根组件 App
│   │   ├── router/           # 路由定义 + 守卫配置
│   │   ├── components/       # 页面组件 (登录/仪表盘/用户管理等)
│   │   ├── api/              # 后端 API 调用封装
│   │   ├── http/             # HTTP 客户端 (gloo-net)
│   │   ├── models/           # 前端数据模型
│   │   ├── config/           # 常量 (BASE_URL, storage keys)
│   │   ├── storage/          # localStorage 封装
│   │   ├── i18n/             # 国际化 (zh-CN / en-US)
│   │   ├── theme/            # 主题切换 (亮色/暗色)
│   │   └── icons/            # Element Plus 图标 (137+)
│   └── AGENTS.md             # Dioxus 0.7 专项指南 (保留)
├── axum-dixous-admin-wx/     # 微信小程序 (TypeScript 原生)
│   └── miniprogram/
│       ├── app.ts / app.json / app.wxss
│       ├── pages/            # dashboard / login / profile / users
│       └── utils/            # api.ts / auth.ts / config.ts / request.ts
├── .catpaw/skills/           # Agent Skills
│   ├── axum-crud/            # 全栈 CRUD 模块生成器
│   └── docker-deploy/        # Docker 部署指南
└── AGENTS.md                 # 本文件
```

## 技术栈

| 层级 | 技术 | 版本 |
|------|------|------|
| 后端 Web 框架 | Axum | 0.8 |
| ORM | SeaORM | 1 |
| 数据库 | MySQL | 8 |
| 缓存 | Redis | 7 |
| 权限 | Casbin | 2 (RBAC, keyMatch2) |
| JWT | jsonwebtoken | 9 |
| API 文档 | utoipa + Swagger UI | 5 / 9 |
| 前端框架 | Dioxus | 0.7 |
| UI 组件 | dioxus-element-plug | 0.2 |
| HTTP 客户端 | gloo-net | 0.7 |
| 前端路由守卫 | route-guard (本地 crate) | - |
| 微信小程序 | TypeScript + WXML/WXSS | 原生 |
| Rust edition | 2024 | rust-version 1.96 |

## 开发环境

### 启动基础设施

```bash
cd backend
docker compose up -d        # MySQL + Redis + Adminer
```

- Adminer: http://localhost:8090
- MySQL: localhost:3306 (root / root123456 / scm)
- Redis: localhost:6379

### 启动后端

```bash
cd backend
cargo run                   # 默认运行 gateway，监听 0.0.0.0:8888
```

- Swagger UI: http://localhost:8888/
- 健康检查: http://localhost:8888/health

### 启动前端

```bash
cd web
dx serve                    # 默认 web 平台，热重载
```

前端默认连接 `http://localhost:8888`（见 `web/src/config/mod.rs`）。

## 后端架构

### 分层结构

```
gateway (main.rs)           → 启动入口，组装路由和中间件
  ├── api (HTTP 层)          → 路由定义 + OpenAPI 注解 + 请求处理
  ├── service (业务层)        → 业务逻辑，调用 model + utils
  ├── model (数据层)
  │   ├── dao/              → SeaORM 实体 (由 sea-orm-cli 生成)
  │   └── dto/              → 请求/响应数据传输对象
  ├── auth-layer (中间件)     → JWT 验证 + Casbin enforce
  ├── casbin-adapter         → SeaORM 实现的 Casbin Adapter
  ├── migration (迁移)        → SeaORM 迁移脚本
  └── utils (工具)           → 配置、DB 连接、JWT、密码、日志
```

### 关键约定

#### 配置

- 配置文件: `backend/server/config.yml`
- 通过 `include_str!` 在编译期内嵌，运行时用 `once_cell::Lazy` 全局持有
- 访问方式: `utils::prelude::CONFIG.datasource.host`
- **Docker 构建注意**: `config.yml` 中的 `localhost` 需在 Dockerfile 中用 `sed` 替换为 Docker 服务名 (`mysql`/`redis`)

#### 数据库连接

- 使用 `db_conn!()` 宏获取连接: `let db = db_conn!();`
- 宏展开为 `&utils::prelude::DB::db_connection().await`

#### 统一响应

- 所有 API 返回 `R<T>` 结构: `{ code, message, data }`
- 成功: `R::ok(data)` → code=200
- 失败: `R::fail(code, message)`

#### 分页

- 请求: `PageRequest { page, page_size, keyword }` (均为 Option)
- 响应: `PageResponse<T> { list, total, page, page_size }`

#### 鉴权流程

1. 登录: `POST /api/user/login` → 返回 JWT token
2. 受保护路由: `AuthLayer` 中间件拦截
   - 验证 JWT → 提取 `Username` 注入 request extension
   - Casbin enforce(sub=user, obj=path, act=method) → 通过/拒绝
3. 前端: `Authorization: Bearer {token}` 请求头

#### API 路由规范

- 公开路由: `/api/user/login`, `/health`
- 受保护路由: `/api/{resource}` (CRUD)
- 路由示例: `/api/user/{id}` GET/PUT/DELETE

#### 数据迁移

- 迁移模式由 `config.yml` 中 `migration` 字段控制: `fresh` / `up` / `down` / `reset`
- **当前开发环境使用 `fresh`**: 每次重启服务会 drop 所有表并重新执行全部迁移，因此修改现有迁移文件（如添加初始数据）后直接重启即可生效，无需新建迁移文件
- 新建迁移: `cd backend/server/shell && sh migrate_table.sh`
- 生成实体: `cd backend/server/shell && sh gen_entity.sh`

#### 添加新模块步骤

1. 创建迁移: `sea migrate generate create_xxx`
2. 生成实体: 运行 `gen_entity.sh` (或手动在 `model/src/dao/` 创建)
3. 创建 DTO: 在 `model/src/dto/` 添加 `xxx_dto.rs`
4. 创建 Service: 在 `service/src/` 添加 `xxx_service.rs`
5. 创建 API: 在 `api/src/` 添加 `xxx_api.rs`，注册路由和 OpenAPI
6. 注册模块: 在各 `lib.rs` 中 `pub mod xxx;`
7. 合并路由: 在 `api/src/lib.rs` 的 `protected_routes()` 中 `.merge(xxx_api::routes())`

## 前端架构

### 模块说明

| 模块 | 职责 |
|------|------|
| `main.rs` | 入口，初始化 i18n + 主题，启动 Dioxus |
| `app/` | 根组件 App，注入全局样式 + Router |
| `router/` | 路由枚举定义 + RouteGuard 配置 |
| `components/` | 页面组件 (Login, Dashboard, UserManage 等) |
| `api/` | 按模块封装后端 API 调用 |
| `http/` | 底层 HTTP 封装 (get/post/put/delete + 401 处理) |
| `models/` | 前端数据模型，与后端 DTO 对应 |
| `config/` | 常量: `BASE_URL`, `TOKEN_KEY`, `USERNAME_KEY` |
| `storage/` | localStorage 读写 (token/username) |
| `i18n/` | 国际化: `t(TKey::xxx)` 翻译函数 |
| `theme/` | 主题切换: 亮色/暗色，CSS 变量，持久化到 localStorage |
| `icons/` | Element Plus 图标组件 (137+) |

### 关键约定

#### HTTP 请求

- 统一通过 `http::get/post/put/delete` 调用
- 自动携带 `Authorization: Bearer {token}` 请求头
- 401 自动清除 token (前端路由守卫随后重定向到登录页)
- 返回 `Result<T, String>`，成功为反序列化后的 `T`，失败为错误消息

#### 路由守卫

- 使用 `route-guard` crate (本地 `crates/route-guard/`)
- 未登录访问受保护路由 → 自动跳转 `/login`
- 已登录访问 `/login` → 自动跳转 `/`
- 守卫在路由变化后、组件渲染前执行，无页面闪烁

#### 国际化

- `t(TKey::xxx)` 在渲染期调用，自动订阅 locale Signal
- `set_locale(Locale::EnUS)` 切换语言并持久化到 localStorage
- 新增翻译: 在 `i18n/mod.rs` 的 `TKey` 枚举添加变体，然后在 `t_zh` / `t_en` 添加对应翻译

#### 主题

- 亮色/暗色切换，基于 CSS 变量 (`var(--el-xxx)`)
- `theme::init_theme()` 初始化，`theme::toggle_theme()` 切换
- 持久化到 localStorage (`admin_theme` key)
- 所有组件颜色必须使用 `var(--el-xxx)` CSS 变量，禁止硬编码 hex 值

#### UI 组件

- 使用 `dioxus-element-plug` 提供 Element Plus 组件
- 导入: `use dioxus_element_plug::prelude::*;`
- 可用组件: Button, Input, Table, Pagination, Dialog, Form, Select, Tag, Card 等
- 图标: `use crate::icons::element::xxx;` (如 `Search`, `Plus`, `Edit`, `Delete`)

#### 添加新页面步骤

1. 在 `components/` 创建 `xxx_manage.rs`，实现 `#[component] fn XxxManage() -> Element`
2. 在 `components/mod.rs` 添加 `pub mod xxx_manage;`
3. 在 `router/mod.rs` 的 `Route` 枚举添加 `#[route("/xxx")] XxxManage {}`
4. 在 `admin_layout.rs` 的侧边栏菜单添加导航链接
5. 在 `i18n/mod.rs` 添加相关翻译 key
6. 如需调用后端: 在 `api/` 添加对应模块，在 `models/` 添加数据模型

## 微信小程序架构

### 模块说明

| 文件/目录 | 职责 |
|-----------|------|
| `utils/config.ts` | 常量: `BASE_URL`, `TOKEN_KEY`, `USERNAME_KEY` |
| `utils/auth.ts` | token 读写 (wx.Storage) |
| `utils/request.ts` | HTTP 封装: `request/get/post/put/del` + 401 处理 |
| `utils/api.ts` | API 接口定义: `api.login()`, `api.userList()` 等 |
| `pages/login/` | 登录页 (账号密码 + 微信登录) |
| `pages/dashboard/` | 仪表盘 |
| `pages/users/` | 用户管理列表 |
| `pages/profile/` | 个人信息 + 修改密码 |

### 关键约定

- 与 Web 端共用同一套后端 API (`http://localhost:8888/api/*`)
- `request.ts` 对应 Web 端的 `http/mod.rs`，逻辑一致 (401 清 token + 跳登录)
- `api.ts` 对应 Web 端的 `api/` 各模块
- 401 自动 `wx.reLaunch` 跳转登录页

## Dioxus 0.7 专项指南

详见 `web/AGENTS.md`，包含 RSX 语法、组件、Signal 状态管理、路由、全栈等完整文档。

## Docker 部署

详见 `.catpaw/skills/docker-deploy/SKILL.md`，包含完整部署指南。

### 快速部署

```bash
# 后端 (MySQL + Redis + Adminer + Backend)
cd backend && docker compose up -d

# 前端 (先构建 WASM，再启动 Nginx)
cd web && dx build --release && docker compose up -d
```

### 后端 Dockerfile 要点

- 两阶段构建: `rust:1.96-slim-bookworm` (builder) → `debian:bookworm-slim` (runtime)
- 依赖缓存: 先拷 `.cargo/` + 各 `Cargo.toml`，创建占位源码编译依赖，再拷真实源码
- 国内加速: `.cargo/config.toml` (rsproxy.cn) + `SWAGGER_UI_DOWNLOAD_URL` (gh-proxy.com)
- config.yml 替换: `sed` 将 `localhost` → `mysql`/`redis` (Docker 服务名)
- 运行时: 非 root 用户 (`app`) + 健康检查 (`/health`)

### 前端构建要点

- WASM 最小化: `web/Cargo.toml` 中 `[profile.release]` (opt-level="z", lto, codegen-units=1, panic=abort, strip)
- 构建产物: `web/target/dx/web/release/web/public/`
- Nginx: WASM MIME 类型 + 带哈希资源长期缓存 + `/api/` 反向代理 + SPA 路由回退

### 端口映射

| 服务 | 宿主机端口 | 说明 |
|------|-----------|------|
| MySQL | 3306 | 数据库 |
| Redis | 6379 | 缓存 |
| Adminer | 8090 | 数据库管理 |
| Backend | 8888 | API + Swagger UI |
| Nginx | 80 | 前端 + API 代理 |

## Agent Skills

项目内置两个 Skill，位于 `.catpaw/skills/`：

| Skill | 触发词 | 用途 |
|-------|--------|------|
| `axum-crud` | "新增模块", "CRUD", "增删改查" | 生成全栈 CRUD 模块 (migration → dao → dto → service → api + 前端) |
| `docker-deploy` | "部署", "Docker", "docker compose", "Nginx", "上线" | Docker 全栈部署指南 |

## 编码规范

- Rust edition 2024, rust-version 1.96
- 后端: 中文注释，模块级 `//!` 文档注释
- 前端: 中文注释，组件使用 `#[component]` 宏
- 微信小程序: TypeScript，中文注释
- 错误处理: 后端用 `anyhow::Result` + `AppError`，前端用 `Result<T, String>`
- 序列化: 后端 `serde` derive，前端 `serde` + `serde_json`
- 命名: 后端 snake_case，组件 PascalCase，小程序 camelCase
- CSS 颜色: 必须使用 `var(--el-xxx)` CSS 变量，禁止硬编码 hex 值
