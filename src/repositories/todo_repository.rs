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
    async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
        let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)
    }

    async fn get_by_id(&self, id: i32) -> Result<Todo, AppError> {
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Todo dengan id {} tidak ditemukan", id)))?;
        Ok(todo)
    }

    async fn create(&self, payload: CreateTodo) -> Result<Todo, AppError> {
        let result = sqlx::query("INSERT INTO todos (judul, isi, tanggal) VALUES (?, ?, ?)")
            .bind(&payload.judul)
            .bind(&payload.isi)
            .bind(payload.tanggal)
            .execute(&self.pool)
            .await?;

        let new_id = result.last_insert_id() as i32;
        self.get_by_id(new_id).await
    }

    async fn update(&self, id: i32, payload: CreateTodo) -> Result<Todo, AppError> {
        // Pastikan item ada
        self.get_by_id(id).await?;

        sqlx::query("UPDATE todos SET judul = ?, isi = ?, tanggal = ? WHERE id = ?")
            .bind(&payload.judul)
            .bind(&payload.isi)
            .bind(payload.tanggal)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.get_by_id(id).await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound(format!("Todo dengan id {} tidak ditemukan", id)))
        } else {
            Ok(())
        }
    }
}