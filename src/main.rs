use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use std::fs::read_dir;
use std::path::PathBuf;
use std::env;

// mod language;
// mod utilities;
mod simple_generator;
mod routes;
mod helpers;

use crate::simple_generator::SimpleGenerator;
use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut setting_files = Vec::<PathBuf>::new();
    for file in read_dir(format!("{}/settings/", env::current_dir().unwrap().display())).unwrap() {
        setting_files.push(file.unwrap().path());
    }

    let state = web::Data::new(AppState {
        generator: Mutex::new(SimpleGenerator::load_str("default-settings.json")),
        generators: Mutex::new(load_generators(setting_files)),
    });

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

