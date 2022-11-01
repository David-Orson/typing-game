use actix_web::{post, web, HttpResponse, Responder};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(log_in));
}

#[post("login")]
pub async fn log_in() -> impl Responder {
    HttpResponse::Ok().body("login route")
}
