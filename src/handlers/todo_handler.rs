use crate::error::AppError;
use crate::models::{CreateTodo, Todo};
use crate::AppState;
use crate::middleware::AuthUser; // <-- Impor AuthUser
use axum::{
    extract::{Path, State, Extension}, // <-- Tambahkan Extension
    http::StatusCode,
    Json,
};
use std::sync::Arc;

// GET /todos
pub async fn get_all_todos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthUser>, // <-- Ambil user dari middleware
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = state.todo_service.get_all_todos(user.id).await?; // <-- Masukkan user.id
    Ok(Json(todos))
}

// GET /todos/:id
pub async fn get_todo_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user): Extension<AuthUser>, // <-- Ambil user
) -> Result<Json<Todo>, AppError> {
    let todo = state.todo_service.get_todo_by_id(id, user.id).await?; // <-- Masukkan user.id
    Ok(Json(todo))
}

// POST /todos
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthUser>, // <-- Ambil user
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let new_todo = state.todo_service.create_todo(payload, user.id).await?; // <-- Masukkan user.id
    Ok(Json(new_todo))
}

// PUT /todos/:id
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user): Extension<AuthUser>, // <-- Ambil user
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let updated_todo = state.todo_service.update_todo(id, payload, user.id).await?; // <-- Masukkan user.id
    Ok(Json(updated_todo))
}

// DELETE /todos/:id
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user): Extension<AuthUser>, // <-- Ambil user
) -> Result<StatusCode, AppError> {
    state.todo_service.delete_todo(id, user.id).await?; // <-- Masukkan user.id
    Ok(StatusCode::NO_CONTENT)
}