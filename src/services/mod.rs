use crate::models::{CreateTodo, Todo};
use crate::error::AppError;
use async_trait::async_trait;

pub mod todo_service;
pub mod user_service;

use crate::models::{CreateUser, LoginUser, LoginResponse, User};

#[async_trait]
pub trait TodoService: Send + Sync {
    // Tambahkan user_id
    async fn get_all_todos(&self, user_id: i32) -> Result<Vec<Todo>, AppError>;
    async fn get_todo_by_id(&self, id: i32, user_id: i32) -> Result<Todo, AppError>;
    async fn create_todo(&self, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError>;
    async fn update_todo(&self, id: i32, payload: CreateTodo, user_id: i32) -> Result<Todo, AppError>;
    async fn delete_todo(&self, id: i32, user_id: i32) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn register(&self, payload: CreateUser) -> Result<User, AppError>;
    async fn login(&self, payload: LoginUser) -> Result<LoginResponse, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<User, AppError>;
}