use crate::handlers::todo_handler;
use crate::AppState;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};

pub fn create_router(app_state: Arc<AppState>) -> Router {

    let cors = CorsLayer::new()
        .allow_origin(Any) // Izinkan semua origin (untuk development)
        .allow_methods(Any) // Izinkan semua metode (GET, POST, dll)
        .allow_headers(Any); // Izinkan semua header

    Router::new()
        .route("/todos", get(todo_handler::get_all_todos))
        .route("/todos", post(todo_handler::create_todo))
        .route("/todos/:id", get(todo_handler::get_todo_by_id))
        .route("/todos/:id", put(todo_handler::update_todo))
        .route("/todos/:id", delete(todo_handler::delete_todo))
        .layer(cors)
        .with_state(app_state)
}