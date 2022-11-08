use actix_web::web;

pub mod auth_handler;

use auth_handler::{log_in, sign_up};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(sign_up).service(log_in));
}
