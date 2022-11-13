use actix_web::web;

pub mod auth_handler;
pub mod test_handler;

use auth_handler::{log_in, sign_up};
use test_handler::finish;

pub fn test_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(finish);
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up).service(log_in);
}
