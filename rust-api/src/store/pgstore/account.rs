use actix_web::web::Data;
use bcrypt::hash;
use std::io;

use crate::{store::models::auth::Account, AppState};

pub async fn get(
    id: i32,
    state: &Data<AppState>,
) -> Result<Account, io::Error> {
    match sqlx::query_as::<_, Account>(
        "SELECT * FROM account WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(acc) => Ok(acc),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }
}

pub async fn create(
    account: Account,
    state: &Data<AppState>,
) -> Result<Account, io::Error> {
    let hashed_pass: String;

    match hash(account.password, 4) {
        Ok(result) => hashed_pass = result,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    };

    match sqlx::query_as::<_, Account>(
        "INSERT INTO account (
            username, 
            email, 
            password,
            pr
        ) VALUES (
            $1,
            $2,
            $3,
            0
        ) 
        RETURNING 
            id, 
            username,
            email,
            password,
            pr
        ",
    )
    .bind(account.username)
    .bind(account.email)
    .bind(hashed_pass)
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => return Ok(data),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }
}

pub async fn update_pr(
    account: Account,
    state: &Data<AppState>,
) -> Result<String, io::Error> {
    match sqlx::query(
        "UPDATE 
            account 
        SET 
            pr = $1
        WHERE
            id = $2
        ",
    )
    .bind(account.pr)
    .bind(account.id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(String::from("Updated Pr")),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }
}
