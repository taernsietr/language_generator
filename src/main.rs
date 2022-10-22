use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use std::fs::read_dir;
use std::path::Path;

// mod language;
mod simple_generator;
mod routes;
// mod utilities;

// use crate::language::Language;
use crate::simple_generator::SimpleGenerator;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        generator: Mutex::new(SimpleGenerator::load("default-settings.json")),
    });

    let mut setting_files = Vec::new();
    for file in read_dir("/home/tsrodr/Run/language_generator/src/settings/").unwrap() {
        setting_files.push(file.unwrap().file_name());
    }

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/word", web::get().to(random_word))
                    .route("/text", web::get().to(random_text))
                    .route("/current", web::get().to(get_settings))
                    .route("/new", web::post().to(new_settings))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

