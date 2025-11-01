use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// Struct untuk data di Database
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

// Struct untuk Registrasi (dengan validasi)
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: String,

    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

// Validasi kustom untuk password (TANPA REGEX LOOKAHEAD)
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if password.len() < 8 {
        let mut err = validator::ValidationError::new("password_length");
        err.message = Some("Password minimal 8 karakter".into());
        return Err(err);
    }

    if !has_uppercase {
        let mut err = validator::ValidationError::new("password_uppercase");
        err.message = Some("Password harus memiliki minimal 1 huruf kapital".into());
        return Err(err);
    }

    if !has_symbol {
        let mut err = validator::ValidationError::new("password_symbol");
        err.message = Some("Password harus memiliki minimal 1 simbol".into());
        return Err(err);
    }

    Ok(())
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