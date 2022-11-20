use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub pr: i32,
}

#[derive(Deserialize)]
pub struct AccountBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Token {
    pub hash: String,
    pub account_id: i32,
    pub username: String,
}
