use super::TodoService;
use crate::error::AppError;
use crate::models::{CreateTodo, Todo};
use crate::repositories::TodoRepository;
use async_trait::async_trait;
use std::sync::Arc;

pub struct TodoServiceImpl {
    repo: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_all_todos(&self, user_id: i32) -> Result<Vec<Todo>, AppError> {
        self.repo.get_all(user_id).await
    }

    async fn get_todo_by_id(&self, id: i32, user_id: i32) -> Result<Todo, AppError> {
        self.repo.get_by_id(id, user_id).await
    }

    async fn create_todo(&self, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError> {
        self.repo.create(payload, user_id).await
    }

    async fn update_todo(&self, id: i32, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError> {
        self.repo.update(id, payload, user_id).await
    }

    async fn delete_todo(&self, id: i32, user_id: i32) -> Result<(), AppError> {
        self.repo.delete(id, user_id).await
    }
}
