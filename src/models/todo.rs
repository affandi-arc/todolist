use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,  // <-- TAMBAHKAN INI (field yang hilang!)
    pub judul: String,
    pub isi: String,
    pub tanggal: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodo {
    pub judul: String,
    pub isi: String,
    pub tanggal: NaiveDate,
}