use actix_web::{get, web, web::Data, HttpResponse, Responder};

use crate::{store::pgstore, AppState};

#[get("{id}")]
pub async fn get(
    state: Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    match pgstore::account::get(*id as i32, &state).await {
        Ok(acc) => HttpResponse::Ok().json(acc),
        Err(err) => HttpResponse::InternalServerError()
            .json(String::from(String::from(err.to_string()))),
    }
}
