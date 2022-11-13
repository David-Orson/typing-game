use actix_web::{
    post,
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

#[post("finish")]
pub async fn finish(
    state: Data<AppState>,
    body: Json<TestBody>,
) -> impl Responder {
    let test = Test {
        id: 0,
        account: body.account,
        test: body.test.to_string(),
        typed: body.typed.to_string(),
    };

    match pgstore::test::create(test, &state).await {
        Ok(_) => HttpResponse::Ok().body("created test"),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(err.to_string())),
    }
}
