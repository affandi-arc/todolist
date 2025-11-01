// src/error.rs
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use jsonwebtoken::errors::Error as JwtError; // Impor error JWT
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors; // Impor error Validasi

#[derive(Debug, Error)]
pub enum AppError {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Item tidak ditemukan: {0}")]
    NotFound(String),

    // --- TAMBAHAN BARU ---

    #[error("Input tidak valid: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Error JWT: {0}")]
    Jwt(#[from] JwtError),

    #[error("Login gagal: {0}")]
    InvalidLogin(String),

    #[error("Otentikasi diperlukan")]
    Unauthorized,

    #[error("Duplikat entry: {0}")]
    DuplicateEntry(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(e) => {
                // Cek error duplikat MySQL (Error 1062)
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code() == Some("23000".into()) { // Kode SQL state untuk duplikat
                        return (StatusCode::CONFLICT, Json(json!({ "error": "Username atau email sudah digunakan." }))).into_response();
                    }
                }
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Token tidak valid atau kadaluarsa".into()),
            AppError::InvalidLogin(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Otentikasi diperlukan".into()),
            AppError::DuplicateEntry(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}