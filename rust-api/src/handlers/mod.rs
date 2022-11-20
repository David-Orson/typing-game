use actix_web::web;

pub mod account_handler;
pub mod auth_handler;
pub mod test_handler;

use account_handler::get;
use auth_handler::{log_in, sign_up};
use test_handler::{finish, get_all_by_account};

pub fn test_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(finish).service(get_all_by_account);
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up).service(log_in);
}

pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
