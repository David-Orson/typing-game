use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Test {
    pub id: i32,
    pub account: i32,
    pub test: String,
    pub typed: String,
}

#[derive(Deserialize)]
pub struct TestBody {
    pub account: i32,
    pub test: String,
    pub typed: String,
}
