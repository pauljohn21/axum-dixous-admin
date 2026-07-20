use std::fmt;

use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    AppError(#[from] axum::Error),
    #[error("认证失败: {0}")]
    AuthError(String),
    #[error("权限不足: {0}")]
    Forbidden(String),
    #[error("资源不存在: {0}")]
    NotFoundError(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::AuthError(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFoundError(_) => (StatusCode::NOT_FOUND, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = serde_json::json!({ "code": status.as_u16(), "message": message });
        (status, axum::Json(body)).into_response()
    }
}

/// Service 层领域错误类型
#[derive(Debug, Error)]
pub enum ServiceError {
    // ── 通用领域错误 ──
    #[error("资源不存在: {0}")]
    NotFound(String),
    #[error("认证失败: {0}")]
    Auth(String),
    #[error("权限不足: {0}")]
    Forbidden(String),
    #[error("参数错误: {0}")]
    BadRequest(String),

    // ── 用户领域特定 ──
    #[error("用户不存在")]
    UserNotFound,
    #[error("密码错误")]
    InvalidPassword,
    #[error("该微信号已绑定其他账号")]
    WechatAlreadyBound,
    #[error("微信登录失败: {0}")]
    WechatApi(String),

    // ── 基础设施错误 ──
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("JWT 错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

impl From<ServiceError> for AppError {
    fn from(e: ServiceError) -> Self {
        match &e {
            ServiceError::NotFound(_) | ServiceError::UserNotFound => {
                AppError::NotFoundError(e.to_string())
            }
            ServiceError::Auth(_)
            | ServiceError::InvalidPassword
            | ServiceError::WechatApi(_)
            | ServiceError::WechatAlreadyBound => AppError::AuthError(e.to_string()),
            ServiceError::Forbidden(_) => AppError::Forbidden(e.to_string()),
            ServiceError::BadRequest(_) => AppError::AuthError(e.to_string()),
            _ => AppError::Internal(e.to_string()),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ParseError(password_hash::Error);

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ParseError").field(&self.0.to_string()).finish()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Password verification errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerifyError {
    /// Password hash parsing errors.
    Parse(ParseError),

    /// Password is invalid.
    PasswordInvalid,
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{err}"),
            Self::PasswordInvalid => write!(f, "password is invalid"),
        }
    }
}

impl From<ParseError> for VerifyError {
    fn from(err: ParseError) -> VerifyError {
        VerifyError::Parse(err)
    }
}

impl std::error::Error for ParseError {}

impl std::error::Error for VerifyError {}
