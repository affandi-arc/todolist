use crate::handlers::{todo_handler, user_handler};
use crate::middleware::auth_middleware;
use crate::AppState;
use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use http::{header, Method};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    // CORS Configuration - Support IntelliJ dan localhost
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|_, _| true)) // Allow semua origin
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(false);

    // Rute publik (tanpa auth middleware)
    let public_routes = Router::new()
        .route("/register", post(user_handler::register))
        .route("/login", post(user_handler::login));

    // Rute terproteksi (menggunakan auth middleware)
    let protected_routes = Router::new()
        .route("/todos", get(todo_handler::get_all_todos))
        .route("/todos", post(todo_handler::create_todo))
        .route("/todos/:id", get(todo_handler::get_todo_by_id))
        .route("/todos/:id", put(todo_handler::update_todo))
        .route("/todos/:id", delete(todo_handler::delete_todo))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // Gabungkan semua router
    Router::new()
        .nest("/auth", public_routes)
        .merge(protected_routes)
        .layer(cors) // CORS harus di akhir
        .with_state(app_state)
}