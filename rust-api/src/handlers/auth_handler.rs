use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    store::{
        models::auth::{Account, AccountBody},
        pgstore,
    },
    AppState,
};

#[post("login")]
pub async fn log_in(
    state: Data<AppState>,
    body: Json<AccountBody>,
) -> impl Responder {
    let account = Account {
        id: 0,
        username: body.username.to_string(),
        email: body.email.to_string(),
        password: body.email.to_string(),
    };

    match pgstore::auth::log_in(account, &state).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(String::from(err.to_string()))),
    }
}

#[post("signup")]
pub async fn sign_up(
    state: Data<AppState>,
    body: Json<AccountBody>,
) -> impl Responder {
    let account = Account {
        id: 0,
        username: body.username.to_string(),
        email: body.email.to_string(),
        password: body.email.to_string(),
    };

    match pgstore::account::create(account, &state).await {
        Ok(acc) => HttpResponse::Ok().json(acc),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(String::from(err.to_string()))),
    }
}
