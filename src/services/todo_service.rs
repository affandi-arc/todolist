use super::TodoService;
use crate::error::AppError;
use crate::models::{CreateTodo, Todo};
use crate::repositories::TodoRepository;
use async_trait::async_trait;
use std::sync::Arc;

// Implementasi service, bergantung pada Trait Repo
pub struct TodoServiceImpl {
    repo: Arc<dyn TodoRepository>,
}

// Konstruktor untuk DI
impl TodoServiceImpl {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, AppError> {
        // Di sini bisa ada logika bisnis, cth: logging
        self.repo.get_all().await
    }

    async fn get_todo_by_id(&self, id: i32) -> Result<Todo, AppError> {
        self.repo.get_by_id(id).await
    }

    async fn create_todo(&self, payload: CreateTodo) -> Result<Todo, AppError> {
        // Di sini bisa ada logika bisnis, cth: validasi
        self.repo.create(payload).await
    }

    async fn update_todo(&self, id: i32, payload: CreateTodo) -> Result<Todo, AppError> {
        self.repo.update(id, payload).await
    }

    async fn delete_todo(&self, id: i32) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
