use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use deadpool_postgres::{Client, Pool};

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "account")] // singular 'user' is a keyword..
pub struct Account {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use crate::{handlers::auth_handler::errors::MyError, handlers::auth_handler::Account};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    pub async fn add_user(client: &Client, account: Account) -> Result<Account, MyError> {
        let _stmt = "
        INSERT INTO account
        (email, first_name, last_name, username)
        VALUES ($1, $2, $3, $4)
        RETURNING $table_fields;
        ";
        let _stmt = _stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(&stmt, &[&account.email, &account.username])
            .await?
            .iter()
            .map(|row| Account::from_row_ref(row).unwrap())
            .collect::<Vec<Account>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(log_in).service(sign_up));
}

#[post("log-in")]
pub async fn log_in() -> impl Responder {
    HttpResponse::Ok().body("log in route")
}

#[post("sign-up")]
pub async fn sign_up(account: web::Json<Account>, db_pool: web::Data<Pool>) -> impl Responder {
    // validate account

    // create account

    // get created account

    use crate::{
        handlers::auth_handler::db, handlers::auth_handler::errors::MyError,
        handlers::auth_handler::Account,
    };

    let account: Account = account.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, account).await?;

    // respond with account
    let res: Result<HttpResponse, Error> = Ok(HttpResponse::Ok().json(new_user));

    res
}
