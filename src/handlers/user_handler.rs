use crate::error::AppError;
use crate::models::{CreateUser, LoginUser, LoginResponse, User};
use crate::AppState;
use axum::{extract::State, Json};
use std::sync::Arc;

// POST /register
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.register(payload).await?;
    Ok(Json(user))
}

// POST /login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = state.user_service.login(payload).await?;
    Ok(Json(response))
}