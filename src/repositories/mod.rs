use crate::models::{CreateTodo, Todo};
use crate::error::AppError;
use async_trait::async_trait;
pub mod user_repository;

pub mod todo_repository;

use crate::models::{User, CreateUser};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    // TAMBAHKAN user_id ke SEMUA fungsi
    async fn get_all(&self, user_id: i32) -> Result<Vec<Todo>, AppError>;
    async fn get_by_id(&self, id: i32, user_id: i32) -> Result<Todo, AppError>;
    async fn create(&self, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError>;
    async fn update(&self, id: i32, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError>;
    async fn delete(&self, id: i32, user_id: i32) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, payload: &CreateUser, password_hash: &str) -> Result<User, AppError>;
    async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<User, AppError>;
}