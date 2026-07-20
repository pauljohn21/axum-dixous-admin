use std::sync::Arc;
use std::time::Duration;

use axum::http::StatusCode;
use axum::Router;
use auth_layer::AuthLayer;
use casbin::{CachedEnforcer, CoreApi};
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use migration::Migrator;
use utils::prelude::{AppState, CONFIG, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Level::init();
    Migrator::migration_init().await;

    let db = utils::prelude::DB::db_connection().await;

    // 初始化 Casbin
    let model_str = include_str!("../../casbin/rbac_model.conf");
    let model = casbin::DefaultModel::from_str(model_str).await?;
    let adapter = casbin_adapter::SeaOrmAdapter::new(db.clone()).await?;
    let enforcer = Arc::new(tokio::sync::RwLock::new(
        CachedEnforcer::new(model, adapter).await?,
    ));

    // 构建 HTTP 客户端（复用连接池）
    let http_client = reqwest::Client::new();

    // 初始化 Redis 连接管理器
    let redis = utils::prelude::DB::redis_connection().await.clone();

    // 构建应用共享状态
    let state = AppState {
        db: db.clone(),
        enforcer: enforcer.clone(),
        http_client,
        config: CONFIG.clone(),
        redis,
    };

    let auth_layer = AuthLayer::new(enforcer);

    // 中间件链
    let cors = CorsLayer::very_permissive();
    let compression = CompressionLayer::new();
    let timeout = TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30));
    let trace = TraceLayer::new_for_http();

    // 路由
    let app = Router::new()
        .merge(api::public_routes().with_state(state.clone()))
        .merge(api::swagger_routes())
        .merge(
            api::protected_routes()
                .with_state(state.clone())
                .layer(auth_layer),
        )
        .layer(cors)
        .layer(compression)
        .layer(timeout)
        .layer(trace);

    let addr = CONFIG.server.clone().addr();
    info!("服务启动于 {}", addr);
    info!("Swagger UI : {}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("服务已优雅关闭");
    Ok(())
}

/// 优雅关闭信号处理 — 捕获 Ctrl+C
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("收到关闭信号，正在优雅关闭...");
}
