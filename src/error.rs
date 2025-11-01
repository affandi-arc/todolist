use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Item tidak ditemukan: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}