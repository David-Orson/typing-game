use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web::{web, web::Data, App};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

use actix_web::{get, HttpResponse, Result};

mod handlers;
mod store;
use store::pgstore;

pub struct AppState {
    db: Pool<Postgres>,
}

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = pgstore::open().await;

    let app = move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(Cors::permissive())
            .service(
                web::scope("/test").configure(handlers::test_routes),
            )
            .service(
                web::scope("/auth").configure(handlers::auth_routes),
            )
            .service(
                web::scope("/account")
                    .configure(handlers::account_routes),
            )
            .service(health)
    };

    println!("listening on :8086");

    HttpServer::new(app).bind(("0.0.0.0", 8086))?.run().await
}
