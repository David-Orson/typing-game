use actix_web::web::Data;
use bcrypt::hash;
use std::io;

use crate::{store::models::auth::Account, AppState};

pub async fn create(account: Account, state: &Data<AppState>) -> Result<String, io::Error> {
    let hashed_pass: String;

    match hash(account.password, 4) {
        Ok(result) => hashed_pass = result,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
    };

    match sqlx::query(
        "INSERT INTO account (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, password"
    ).bind(account.username)
        .bind(account.email)
        .bind(hashed_pass)
        .execute(&state.db)
        .await
    {
        Ok(_) => return Ok(String::from("hi")),
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}
