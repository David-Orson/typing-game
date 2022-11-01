use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
