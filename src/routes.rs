use crate::handlers::todo_handler;
use crate::AppState;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/todos", get(todo_handler::get_all_todos))
        .route("/todos", post(todo_handler::create_todo))
        .route("/todos/:id", get(todo_handler::get_todo_by_id))
        .route("/todos/:id", put(todo_handler::update_todo))
        .route("/todos/:id", delete(todo_handler::delete_todo))
        .with_state(app_state)
}