use actix_web::{web, App, HttpServer};
use chrono::Local;
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

const DF: &str = "%H:%M:%S";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut setting_files = Vec::<PathBuf>::new();
    let mut setting_names = Vec::<String>::new();

    for file in read_dir(format!("{}/settings/", env::current_dir().unwrap().display())).unwrap() {
        setting_names.push(file.as_ref().unwrap().path().into_os_string().into_string().unwrap());
        setting_files.push(file.unwrap().path());
    }

    let state = web::Data::new(AppState {
        generator: Mutex::new(SimpleGenerator::load_str("default-settings.json")),
        generators: Mutex::new(load_generators(setting_names, setting_files)),
    });

    println!("[SERVER]: {} Server up! Open your preferred browser and access 「http://127.0.0.1:8080」!", Local::now().format(DF));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/word", web::get().to(random_word))
                    .route("/text", web::get().to(random_text))
                    .route("/current", web::get().to(current_settings))
                    .route("/new", web::post().to(save_settings))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

