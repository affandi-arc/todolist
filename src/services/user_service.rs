// src/services/user_service.rs

use super::UserService;
use crate::error::AppError;
use crate::models::{CreateUser, LoginUser, LoginResponse, User};
use crate::repositories::UserRepository;
use crate::auth::{hash_password, verify_password, create_jwt};
use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate; // <-- Ini adalah import yang Anda perlukan

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register(&self, payload: CreateUser) -> Result<User, AppError> {
        // 1. Validasi input (sekarang akan berhasil)
        payload.validate()?;

        // 2. Hash password
        let password_hash = hash_password(&payload.password)?;

        // 3. Simpan ke database
        let user = self.repo.create(&payload, &password_hash).await?;
        Ok(user)
    }

    async fn login(&self, payload: LoginUser) -> Result<LoginResponse, AppError> {
        // 1. Cari user
        let user = self.repo.find_by_username_or_email(&payload.username_or_email)
            .await?
            .ok_or_else(|| AppError::InvalidLogin("Username atau password salah.".into()))?;

        // 2. Verifikasi password (tanpa .await)
        let is_valid = verify_password(&user.password_hash, &payload.password)?;
        if !is_valid {
            return Err(AppError::InvalidLogin("Username atau password salah.".into()));
        }

        // 3. Buat JWT
        let token = create_jwt(user.id)?;
        Ok(LoginResponse { token })
    }

    async fn find_by_id(&self, id: i32) -> Result<User, AppError> {
        self.repo.find_by_id(id).await
    }
}