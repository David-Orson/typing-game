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
    let typed = body.typed.to_string();
    let test = body.test.to_string();

    let wpm: i32 = typed.chars().count() as i32 * 12 / 15;
    let mut accuracy: f32 = 100.0;

    let mut errors = 0;
    let mut space_errors = 0;

    let typed_split: Vec<&str> = typed.split(" ").collect();
    let test_split: Vec<&str> = test.split(" ").collect();

    for (i, word) in typed_split.iter().enumerate() {
        for (j, c) in word.chars().enumerate() {
            if c != test_split[i].chars().nth(j).unwrap() {
                errors += 1;
            }
        }
        if i != typed_split.len() - 1 {
            errors += test_split[i].len() - word.len();
            space_errors += test_split[i].len() - word.len();
        }
    }

    if typed.len() > 0 {
        accuracy = 100.0
            - (errors as f32
                / (typed.replace(" ", "").len() as f32
                    + space_errors as f32))
                * 100.0;
    }

    let test = Test {
        id: 0,
        account_id: body.account,
        test,
        typed,
        wpm,
        accuracy,
    };

    match pgstore::test::create(test, &state).await {
        Ok(_) => HttpResponse::Ok().body("created test"),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(err.to_string())),
    }
}
