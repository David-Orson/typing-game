use actix_web::web::Data;
use bcrypt::{hash, verify};
use sqlx::{query, query_as, Postgres};
use std::io;

use crate::{
    store::models::auth::{Account, Token},
    AppState,
};

pub async fn log_in(
    account: Account,
    state: &Data<AppState>,
) -> Result<Token, io::Error> {
    let fetched_acc: Account;

    match query_as::<_, Account>(
        "SELECT * FROM account WHERE email = $1",
    )
    .bind(account.email.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => fetched_acc = data,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }

    match verify(account.password, &fetched_acc.password) {
        Ok(_) => (),
        Err(err) => {
            println!("{:?} passwords do not match", err)
        }
    }

    let random_bytes: Vec<u8> =
        (0..72).map(|_| rand::random::<u8>()).collect();
    let token_hash: String;

    match hash(random_bytes, 4) {
        Ok(result) => token_hash = result,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    };

    let token_clone = token_hash.clone();
    let username_clone = fetched_acc.username.clone();

    match query::<Postgres>(
        "INSERT INTO token (
			token,
			username,
			account_id
		) VALUES (
			$1,
			$2,
			$3
		)
		;",
    )
    .bind(token_clone)
    .bind(fetched_acc.username)
    .bind(fetched_acc.id)
    .execute(&state.db)
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }

    let token = Token {
        hash: token_hash,
        account_id: fetched_acc.id,
        username: username_clone,
    };

    Ok(token)
}
