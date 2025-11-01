use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, MySql, Pool};
use std::env;

// Tipe alias untuk pool kita
pub type DbPool = Pool<MySql>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus di-set di environment atau .env");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INT AUTO_INCREMENT PRIMARY KEY,
            judul VARCHAR(255) NOT NULL,
            isi TEXT,
            tanggal DATE NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await?;

    Ok(())
}