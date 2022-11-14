use actix_web::{
    get, post, web,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    store::{
        models::test::{Test, TestBody},
        pgstore,
    },
    AppState,
};

#[get("{id}")]
pub async fn get_all_by_account(
    state: Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    let account_id: u32 = *id;

    match pgstore::test::get_all_by_account(account_id, &state).await
    {
        Ok(tests) => HttpResponse::Ok().json(tests),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(err.to_string())),
    }
}

#[post("finish")]
pub async fn finish(
    state: Data<AppState>,
    body: Json<TestBody>,
) -> impl Responder {
    let test = Test {
        id: 0,
        account_id: body.account,
        test: body.test.to_string(),
        typed: body.typed.to_string(),
    };

    match pgstore::test::create(test, &state).await {
        Ok(_) => HttpResponse::Ok().body("created test"),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(err.to_string())),
    }
}
