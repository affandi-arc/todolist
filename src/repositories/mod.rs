use crate::models::{CreateTodo, Todo};
use crate::error::AppError;
use async_trait::async_trait;

pub mod todo_repository;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Todo>, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Todo, AppError>;
    async fn create(&self, payload: CreateTodo) -> Result<Todo, AppError>;
    async fn update(&self, id: i32, payload: CreateTodo) -> Result<Todo, AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}