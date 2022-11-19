use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web::{web, web::Data, App};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

mod handlers;
mod store;
use store::pgstore;

pub struct AppState {
    db: Pool<Postgres>,
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
    };

    HttpServer::new(app).bind(("127.0.0.1", 8086))?.run().await
}
