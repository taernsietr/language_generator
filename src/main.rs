use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod language;
mod phonology;
mod grammar;
mod simple_generator;
// mod utilities;

// use crate::language::Language;
use crate::simple_generator::SimpleGenerator;

struct AppState {
    generator: Mutex<SimpleGenerator>,
}
   
async fn random_word(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_word(6, false)))
}

async fn random_text(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_text(50)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        generator: Mutex::new(SimpleGenerator::load("default.yaml")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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

