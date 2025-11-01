use super::UserRepository;
use crate::database::DbPool;
use crate::error::AppError;
use crate::models::{CreateUser, User};
use async_trait::async_trait;

pub struct MySqlUserRepository {
    pool: DbPool,
}

impl MySqlUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {

    async fn create(&self, payload: &CreateUser, password_hash: &str) -> Result<User, AppError> {
        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)"
        )
            .bind(&payload.username)
            .bind(&payload.email)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;

        let new_id = result.last_insert_id() as i32;
        self.find_by_id(new_id).await
    }

    async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ? OR email = ?"
        )
            .bind(username_or_email)
            .bind(username_or_email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: i32) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User dengan id {} tidak ditemukan", id)))?;
        Ok(user)
    }
}