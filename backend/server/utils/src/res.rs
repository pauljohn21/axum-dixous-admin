use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct R<T: Serialize + ToSchema> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize + ToSchema> R<T> {
    pub fn ok(data: T) -> Self {
        R { code: 200, message: "success".into(), data: Some(data) }
    }

    pub fn fail(code: u16, message: impl Into<String>) -> Self {
        R { code, message: message.into(), data: None }
    }
}

impl<T: Serialize + ToSchema> IntoResponse for R<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
