use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;

pub mod auth_handler;

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    let app = move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(auth_handler::auth_routes)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8085))?.run().await
}
