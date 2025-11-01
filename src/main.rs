use crate::database::{create_pool, run_migrations};
use crate::repositories::{
    todo_repository::MySqlTodoRepository, TodoRepository,
    user_repository::MySqlUserRepository, UserRepository, // <-- Impor User Repo
};
use crate::routes::create_router;
use crate::services::{
    todo_service::TodoServiceImpl, TodoService,
    user_service::UserServiceImpl, UserService, // <-- Impor User Service
};
use std::net::SocketAddr;
use std::sync::Arc;

// Deklarasikan semua modul baru
pub mod auth; // <--
pub mod database;
pub mod error;
pub mod handlers;
pub mod middleware; // <--
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

pub struct AppState {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>, // <-- Tambahkan UserService
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = create_pool().await.expect("Gagal membuat DB pool");
    run_migrations(&pool).await.expect("Gagal menjalankan migrasi");
    println!("Database terhubung dan tabel siap.");

    // --- Set up Dependency Injection (DI) ---

    // Repositories
    let todo_repo: Arc<dyn TodoRepository> = Arc::new(MySqlTodoRepository::new(pool.clone()));
    let user_repo: Arc<dyn UserRepository> = Arc::new(MySqlUserRepository::new(pool.clone()));

    // Services
    let todo_service: Arc<dyn TodoService> = Arc::new(TodoServiceImpl::new(todo_repo));
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo));

    // --- Buat AppState ---
    let app_state = Arc::new(AppState {
        todo_service,
        user_service, // <-- Masukkan ke state
    });

    let app = create_router(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8180));
    println!("Mendengarkan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}