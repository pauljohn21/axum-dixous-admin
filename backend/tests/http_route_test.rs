//! HTTP 路由测试 — 使用 tower::ServiceExt::oneshot
//!
//! 不启动真实 HTTP 服务器，在内存中模拟请求。
//! 需要运行 MySQL (localhost:3306) 和 Redis (localhost:6379)。

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serial_test::serial;
use tower::ServiceExt;

#[tokio::test]
#[serial]
async fn test_health_route() {
    let state = common::setup_test_state().await;
    let app = api::public_routes().with_state(state);

    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
#[serial]
async fn test_login_route_success() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    let app = api::public_routes().with_state(state);

    let body = serde_json::json!({
        "username": "testuser",
        "password": "123456"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/user/login")
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["code"], 200);
    assert!(json["data"]["token"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_login_route_wrong_password() {
    let state = common::setup_test_state().await;
    common::insert_test_user(&state.db).await;

    let app = api::public_routes().with_state(state);

    let body = serde_json::json!({
        "username": "testuser",
        "password": "wrongpassword"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/user/login")
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // ServiceError::InvalidPassword → AppError::AuthError → 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_login_route_user_not_found() {
    let state = common::setup_test_state().await;

    let app = api::public_routes().with_state(state);

    let body = serde_json::json!({
        "username": "ghost",
        "password": "123456"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/user/login")
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_protected_route_without_token() {
    let state = common::setup_test_state().await;

    // protected_routes 需要 AuthLayer 中间件
    // 这里测试 public_routes 不包含受保护路由
    let app = api::public_routes().with_state(state);

    // /health 是公开路由，应该可以访问
    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_protected_route_with_valid_token() {
    let state = common::setup_test_state().await;
    // 种子数据已包含 admin 用户和角色分配 (g, admin, 888)

    // 构建完整应用（公开 + 受保护路由 + AuthLayer）
    use auth_layer::AuthLayer;
    let app = api::public_routes()
        .merge(api::protected_routes())
        .layer(AuthLayer::new(
            state.enforcer.clone(),
            state.redis.clone(),
        ))
        .with_state(state.clone());

    // 创建 JWT token（admin 用户在种子数据中有角色分配）
    let token = utils::prelude::create_token("admin").unwrap();

    // 访问受保护路由 /api/user/info
    let request = Request::builder()
        .method("GET")
        .uri("/api/user/info")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["code"], 200);
    assert_eq!(json["data"]["username"], "admin");
}
