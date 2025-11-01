use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[A-Z])(?=.*\W).{8,}$")
        .expect("Gagal meng-compile regex password");
}

// validasi password
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    if !PASSWORD_REGEX.is_match(password) {
        let mut err = validator::ValidationError::new("password_policy");
        err.message = Some("Password harus memiliki minimal 8 karakter, 1 huruf kapital, dan 1 simbol.".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Jangan pernah kirim hash password ke klien
    pub password_hash: String,
}

// Struct untuk Registrasi (dengan validasi)
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: String,

    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(custom = "validate_password_strength")]
    pub password: String,
}

// Struct untuk Login
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username_or_email: String,
    pub password: String,
}

// Struct untuk respons Login (mengirim token)
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}