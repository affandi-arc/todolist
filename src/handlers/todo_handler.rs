use crate::error::AppError;
use crate::models::{CreateTodo, Todo};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

// Handler hanya berinteraksi dengan Trait Service
// Tipe kembalian sekarang menggunakan AppError

// GET /todos
pub async fn get_all_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = state.todo_service.get_all_todos().await?;
    Ok(Json(todos))
}

// GET /todos/:id
pub async fn get_todo_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, AppError> {
    let todo = state.todo_service.get_todo_by_id(id).await?;
    Ok(Json(todo))
}

// POST /todos
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let new_todo = state.todo_service.create_todo(payload).await?;
    Ok(Json(new_todo))
}

// PUT /todos/:id
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let updated_todo = state.todo_service.update_todo(id, payload).await?;
    Ok(Json(updated_todo))
}

// DELETE /todos/:id
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    state.todo_service.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}