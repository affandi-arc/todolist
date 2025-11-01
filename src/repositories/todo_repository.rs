use super::TodoRepository;
use crate::database::DbPool;
use crate::error::AppError;
use crate::models::{CreateTodo, Todo};
use async_trait::async_trait;

pub struct MySqlTodoRepository {
    pool: DbPool,
}

impl MySqlTodoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for MySqlTodoRepository {
    async fn get_all(&self, user_id: i32) -> Result<Vec<Todo>, AppError> {
        // Tambahkan 'WHERE user_id = ?'
        let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)
    }

    async fn get_by_id(&self, id: i32, user_id: i32) -> Result<Todo, AppError> {
        // Tambahkan 'AND user_id = ?'
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Todo dengan id {} tidak ditemukan", id)))?;
        Ok(todo)
    }

    async fn create(&self, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError> {
        // Tambahkan 'user_id' ke INSERT
        let result = sqlx::query(
            "INSERT INTO todos (judul, isi, tanggal, user_id) VALUES (?, ?, ?, ?)"
        )
            .bind(&payload.judul)
            .bind(&payload.isi)
            .bind(payload.tanggal)
            .bind(user_id) // Bind user_id
            .execute(&self.pool)
            .await?;

        let new_id = result.last_insert_id() as i32;
        // Panggil get_by_id dengan user_id untuk keamanan
        self.get_by_id(new_id, user_id).await
    }

    async fn update(&self, id: i32, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError> {
        // Pastikan user ini pemilik item sebelum update
        self.get_by_id(id, user_id).await?;

        sqlx::query(
            // Tambahkan 'AND user_id = ?' untuk memastikan
            "UPDATE todos SET judul = ?, isi = ?, tanggal = ? WHERE id = ? AND user_id = ?"
        )
            .bind(&payload.judul)
            .bind(&payload.isi)
            .bind(payload.tanggal)
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        self.get_by_id(id, user_id).await
    }

    async fn delete(&self, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = sqlx::query(
            // Tambahkan 'AND user_id = ?'
            "DELETE FROM todos WHERE id = ? AND user_id = ?"
        )
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound(format!("Todo dengan id {} tidak ditemukan", id)))
        } else {
            Ok(())
        }
    }
}