use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use casbin::{CachedEnforcer, CoreApi};
use futures::future::BoxFuture;
use tower::Layer;
use utils::prelude::verify_token;
use redis::AsyncCommands;

use std::sync::Arc;
use tokio::sync::RwLock;

/// JWT 中提取的用户名，通过 request extension 传递给下游 handler
#[derive(Clone, Debug)]
pub struct Username(pub String);

#[derive(Clone)]
pub struct AuthLayer {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
    pub redis: redis::aio::ConnectionManager,
    pub domain: String,
}

impl AuthLayer {
    pub fn new(
        enforcer: Arc<RwLock<CachedEnforcer>>,
        redis: redis::aio::ConnectionManager,
        domain: String,
    ) -> Self {
        Self { enforcer, redis, domain }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            enforcer: self.enforcer.clone(),
            redis: self.redis.clone(),
            domain: self.domain.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    enforcer: Arc<RwLock<CachedEnforcer>>,
    redis: redis::aio::ConnectionManager,
    domain: String,
}

impl<S> tower::Service<Request> for AuthMiddleware<S>
where
    S: tower::Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());

        let path = req.uri().path().to_string();
        let method = req.method().clone();
        let enforcer = self.enforcer.clone();
        let redis = self.redis.clone();
        let domain = self.domain.clone();

        // 1. JWT 验证
        let token_info = if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                match verify_token(token) {
                    Ok(claims) => Some((claims.sub, token.to_string())),
                    Err(_) => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        // 2. 无 JWT → 401
        let (subject, token_str) = match token_info {
            Some(s) => s,
            None => {
                return Box::pin(async move { Ok(StatusCode::UNAUTHORIZED.into_response()) });
            }
        };

        // 将用户名注入 request extension，供下游 handler 使用
        req.extensions_mut().insert(Username(subject.clone()));

        // 3. Casbin 权限检查
        let action = method.as_str().to_string();
        let inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, inner);

        Box::pin(async move {
            // JWT 黑名单检查 — Redis O(1) 查询
            let mut redis = redis;
            let token_key = format!("jwt:blacklist:{}", token_str);
            let exists: bool = redis.exists(&token_key).await.unwrap_or(false);
            if exists {
                return Ok(StatusCode::UNAUTHORIZED.into_response());
            }

            let args = vec![subject, path, action, domain];
            let result = {
                let guard = enforcer.read().await;
                guard.enforce(args)
            };

            match result {
                Ok(true) => inner.call(req).await,
                Ok(false) => Ok(StatusCode::FORBIDDEN.into_response()),
                Err(e) => {
                    tracing::error!("Casbin enforce error: {}", e);
                    Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                }
            }
        })
    }
}
