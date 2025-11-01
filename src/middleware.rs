use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::response::IntoResponse;
use std::sync::Arc;
use crate::error::AppError;
use crate::auth::decode_jwt;
use crate::AppState;

// Struct untuk menyimpan ID user yang sudah diautentikasi
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
}

// Fungsi middleware
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>, // Kita akan update AppState nanti
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {

    // 1. Ambil token dari header 'Authorization'
    let token = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        });

    let token = token.ok_or(AppError::Unauthorized)?;

    // 2. Dekode dan validasi token
    let claims = decode_jwt(&token)?;
    let user_id = claims.sub;

    // 3. (Opsional tapi bagus) Cek apakah user masih ada di DB
    // Ini mencegah token lama dari user yang sudah dihapus
    state.user_service.find_by_id(user_id).await?; // Kita akan buat UserService

    // 4. Sisipkan ID user ke dalam 'request extension'
    // agar handler bisa mengaksesnya
    req.extensions_mut().insert(AuthUser { id: user_id });

    // Lanjutkan ke handler
    Ok(next.run(req).await)
}