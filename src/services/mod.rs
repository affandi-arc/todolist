use crate::models::{CreateTodo, Todo};
use crate::error::AppError;
use async_trait::async_trait;

pub mod todo_service;

// Interface (Trait) untuk TodoService
// Perhatikan, ini identik dengan Repo, tapi di sinilah
// logika bisnis akan ditambahkan nanti.
#[async_trait]
pub trait TodoService: Send + Sync {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, AppError>;
    async fn get_todo_by_id(&self, id: i32) -> Result<Todo, AppError>;
    async fn create_todo(&self, payload: CreateTodo) -> Result<Todo, AppError>;
    async fn update_todo(&self, id: i32, payload: CreateTodo) -> Result<Todo, AppError>;
    async fn delete_todo(&self, id: i32) -> Result<(), AppError>;
}