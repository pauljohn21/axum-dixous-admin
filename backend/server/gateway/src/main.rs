use axum::Router;
use auth_layer::AuthLayer;
use casbin::{CachedEnforcer, CoreApi};
use tower_http::cors::CorsLayer;
use tracing::info;

use migration::Migrator;
use utils::prelude::{CONFIG, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Level::init();
    Migrator::migration_init().await;

    let db = utils::prelude::DB::db_connection().await;

    // 初始化 Casbin
    let model_str = include_str!("../../casbin/rbac_model.conf");
    let model = casbin::DefaultModel::from_str(model_str).await?;
    let adapter = casbin_adapter::SeaOrmAdapter::new(db.clone()).await?;
    let enforcer = std::sync::Arc::new(tokio::sync::RwLock::new(
        CachedEnforcer::new(model, adapter).await?,
    ));

    let auth_layer = AuthLayer::new(enforcer);

    // CORS
    let cors = CorsLayer::very_permissive();

    // 路由
    let app = Router::new()
        .merge(api::public_routes())
        .merge(api::swagger_routes())
        .merge(
            api::protected_routes()
                .layer(auth_layer),
        )
        .layer(cors);

    let addr = CONFIG.server.clone().addr();
    info!("服务启动于 {}", addr);
    info!("Swagger UI : {}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
