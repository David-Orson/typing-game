use crate::AppState;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AccountBody {
    pub email: String,
    pub password: String,
}

#[post("login")]
pub async fn log_in(state: Data<AppState>, body: Json<AccountBody>) -> impl Responder {
    match sqlx::query_as::<_, Account>(
        "INSERT INTO account (username, email, password) VALUES ('bob', $1, $2) RETURNING id, username, email, password"
    )
        .bind(body.email.to_string())
        .bind(body.password.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(err) => HttpResponse::InternalServerError().json(String::from(String::from(err.to_string())))
    }
}
