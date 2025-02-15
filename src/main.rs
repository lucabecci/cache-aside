use actix_web::{App, HttpServer};
use crate::web::create_http::{create_event, get_event};

mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_event)
            .service(get_event)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}