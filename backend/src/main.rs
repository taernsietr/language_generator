use actix_web::{web, App, HttpServer};
use chrono::Local;
use std::sync::Mutex;

// mod language;
// mod utilities;
mod simple_generator;
mod routes;
mod helpers;

use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        generators: Mutex::new(load_generators())
    });

    println!("[{}] [SERVER]: Server up! Open your preferred browser and access 「http://127.0.0.1:8080」!", Local::now().format(DF));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/word", web::get().to(random_word))
                    .route("/text", web::get().to(random_text))
                    .route("/generators", web::get().to(get_available_generators))
                    .route("/settings", web::get().to(get_generator_settings))
                    .route("/new", web::post().to(save_settings))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

