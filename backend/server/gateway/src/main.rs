use std::sync::Arc;

use axum::Router;
use auth_layer::AuthLayer;
use casbin::{CachedEnforcer, CoreApi};
use tower_http::cors::CorsLayer;
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

    // 将 enforcer 注入 service 层，用于策略修改后刷新缓存
    service::enforcer::set_enforcer(enforcer.clone());

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

    // CORS
    let cors = CorsLayer::very_permissive();

    // 路由
    let app = Router::new()
        .merge(api::public_routes().with_state(state.clone()))
        .merge(api::swagger_routes())
        .merge(
            api::protected_routes()
                .with_state(state.clone())
                .layer(auth_layer),
        )
        .layer(cors);

    let addr = CONFIG.server.clone().addr();
    info!("服务启动于 {}", addr);
    info!("Swagger UI : {}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
