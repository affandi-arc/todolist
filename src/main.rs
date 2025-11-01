use crate::database::{create_pool, run_migrations};
use crate::repositories::{todo_repository::MySqlTodoRepository, TodoRepository};
use crate::routes::create_router;
use crate::services::{todo_service::TodoServiceImpl, TodoService};
use std::net::SocketAddr;
use std::sync::Arc;

// Deklarasikan semua modul baru
pub mod database;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Struct AppState untuk menampung semua 'state'
// yang akan kita 'inject' ke handlers
pub struct AppState {
    pub todo_service: Arc<dyn TodoService>,
    // Anda bisa tambahkan state lain di sini, cth:
    // pub user_service: Arc<dyn UserService>,
}

#[tokio::main]
async fn main() {
    // 1. Load config
    dotenvy::dotenv().ok();

    // 2. Buat koneksi DB
    let pool = create_pool().await.expect("Gagal membuat DB pool");

    // 3. Jalankan migrasi
    run_migrations(&pool).await.expect("Gagal menjalankan migrasi");
    println!("Database terhubung dan tabel siap.");

    // 4. Set up Dependency Injection (DI)
    // Buat implementasi konkret
    let todo_repo: Arc<dyn TodoRepository> = Arc::new(MySqlTodoRepository::new(pool));
    // Inject repo ke service
    let todo_service: Arc<dyn TodoService> = Arc::new(TodoServiceImpl::new(todo_repo));

    // 5. Buat AppState
    let app_state = Arc::new(AppState {
        todo_service,
        // user_service: ...
    });

    // 6. Buat router
    let app = create_router(app_state);

    // 7. Jalankan Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8180));
    println!("Mendengarkan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}