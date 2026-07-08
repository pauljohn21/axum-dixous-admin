# ============================================================
# Dockerfile — axum-dixous-admin 后端 (gateway)
# ============================================================
# 构建: docker build -t axum-admin-backend -f server/Dockerfile server/
# 两阶段构建: builder → runtime
# 依赖缓存: 先拷 Cargo.toml + 占位源码编译依赖，再拷真实源码
# ============================================================

# ==================== Stage 1: 编译 ====================
FROM rust:1.96-slim-bookworm AS builder

# 安装编译依赖: curl (utoipa-swagger-ui 编译时下载 Swagger UI)
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# GitHub 代理加速 (utoipa-swagger-ui 编译时下载 Swagger UI 资源)
ENV SWAGGER_UI_DOWNLOAD_URL=https://gh-proxy.com/https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.14.zip

WORKDIR /app

# ---------- 1. 拷贝 cargo 镜像配置 + manifest 文件，缓存依赖层 ----------
# 镜像配置 (rsproxy.cn 国内加速)，必须在编译依赖之前拷入
COPY .cargo/ .cargo/

# 各子 crate 的 Cargo.toml
COPY api/Cargo.toml api/Cargo.toml
COPY auth-layer/Cargo.toml auth-layer/Cargo.toml
COPY casbin-adapter/Cargo.toml casbin-adapter/Cargo.toml
COPY data/migration/Cargo.toml data/migration/Cargo.toml
COPY data/model/Cargo.toml data/model/Cargo.toml
COPY gateway/Cargo.toml gateway/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY utils/Cargo.toml utils/Cargo.toml

# ---------- 2. 创建占位源码，编译依赖 ----------
RUN mkdir -p api/src auth-layer/src casbin-adapter/src \
    data/migration/src data/model/src gateway/src service/src utils/src && \
    echo "pub fn lib() {}" > api/src/lib.rs && \
    echo "pub fn lib() {}" > auth-layer/src/lib.rs && \
    echo "pub fn lib() {}" > casbin-adapter/src/lib.rs && \
    echo "pub fn lib() {}" > data/migration/src/lib.rs && \
    echo "pub fn lib() {}" > data/model/src/lib.rs && \
    echo "fn main() {}" > gateway/src/main.rs && \
    echo "pub fn lib() {}" > service/src/lib.rs && \
    echo "pub fn lib() {}" > utils/src/lib.rs

# 编译依赖 (此层被 Docker 缓存，改代码不会重编)
RUN cargo build --release --locked 2>/dev/null || true

# ---------- 3. 拷入真实源码，编译业务代码 ----------
# 删除占位文件，拷入真实代码
RUN find . -name "*.rs" -delete
COPY . .

# 替换 config.yml 中的地址为 Docker 服务名 (include_str! 编译期内嵌)
# datasource.host: localhost → mysql (第一个)
# cache.host:      localhost → redis  (第二个)
RUN sed -i '0,/host: localhost/s/host: localhost/host: mysql/' config.yml && \
    sed -i 's/host: localhost/host: redis/g' config.yml

# 重新编译 (依赖已缓存，只编译业务代码)
RUN cargo build --release

# ==================== Stage 2: 运行时 ====================
FROM debian:bookworm-slim AS runtime

# 安装最小运行时依赖 (curl 用于 healthcheck)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r app && useradd -r -g app -d /app app

WORKDIR /app

# 从构建阶段复制二进制
COPY --from=builder /app/target/release/gateway /app/gateway

# 切换非 root 用户
USER app

EXPOSE 8888

# 健康检查 (后端 /health 端点)
HEALTHCHECK --interval=10s --timeout=3s --retries=3 --start-period=10s \
    CMD curl -sf http://localhost:8888/health || exit 1

CMD ["./gateway"]
