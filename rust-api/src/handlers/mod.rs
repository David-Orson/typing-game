use actix_web::{post, web, HttpResponse, Responder};

pub mod auth_handler;

use auth_handler::log_in;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(log_in));
}
