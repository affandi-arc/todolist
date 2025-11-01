use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::error::AppError; // Kita akan perbarui 'error.rs' nanti

// Struct "Claims" untuk data di dalam JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Expiry
    pub iat: usize, // Issued At
    pub sub: i32,   // Subject (user_id)
}

// Hash password menggunakan Argon2
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("Gagal hash password".into()))?
        .to_string();

    Ok(password_hash)
}

// Verifikasi password
pub fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::InvalidLogin("Format hash tidak valid".into()))?;

    let argon2 = Argon2::default();
    let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Password salah
    }
}

// Dapatkan JWT_SECRET dari .env
fn get_jwt_secret() -> Result<String, AppError> {
    std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal("JWT_SECRET tidak di-set".into()))
}

// Buat JSON Web Token
pub fn create_jwt(user_id: i32) -> Result<String, AppError> {
    let secret = get_jwt_secret()?;
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::days(7); // Token valid 7 hari

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    )?;

    Ok(token)
}

// Verifikasi/Dekode JSON Web Token
pub fn decode_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret()?;
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    )?;

    Ok(token_data.claims)
}