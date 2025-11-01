use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub judul: String,
    pub isi:String,
    pub tanggal: NaiveDate,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CreateTodo {
    pub judul: String,
    pub isi: String,
    pub tanggal: NaiveDate,
}