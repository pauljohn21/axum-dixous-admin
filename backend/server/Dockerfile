# 使用官方Rust镜像作为构建阶段
FROM rust:1.84-slim-bookworm AS builder

# 设置工作目录
WORKDIR /usr/src/admin



# 复制项目源代码
COPY . .

# 构建项目
RUN cargo build --release

# 使用Debian镜像作为运行阶段
FROM debian:bookworm-slim

# 设置工作目录
WORKDIR /usr/src

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /usr/src/admin/target/release/gateway .

# 暴露端口
EXPOSE 8888

# 运行二进制文件
CMD ["./gateway"]