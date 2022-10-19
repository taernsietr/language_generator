use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod language;
mod simple_generator;
mod routes;
// mod utilities;

// use crate::language::Language;
use crate::simple_generator::SimpleGenerator;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        generator: Mutex::new(SimpleGenerator::load("default.yaml")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/word", web::get().to(random_word))
                    .route("/text", web::get().to(random_text))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

