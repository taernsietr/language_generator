use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod language;
mod phonology;
mod grammar;
mod simple_generator;
// mod utilities;

// use crate::language::Language;
use crate::simple_generator::SimpleGenerator;

async fn random_word() -> impl Responder {
    HttpResponse::Ok().body("word")
}

async fn random_text() -> impl Responder {
    HttpResponse::Ok().body("text")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/word", web::get().to(random_word))
            .route("/text", web::get().to(random_text))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    // let defaults = SimpleGenerator::load("default.yaml");

    // let word = warp::path("word").map(|| SimpleGenerator::load("default.yaml").random_word(5, false));
    // let text = warp::path("text").map(|| SimpleGenerator::load("default.yaml").random_text(50));
    // let root = warp::get()
    //     .and(warp::path::end())
    //     .and(warp::fs::file("index.html"));
   
}

