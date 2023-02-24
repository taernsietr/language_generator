use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use chrono::Local;
use std::sync::Mutex;

// mod language;
mod simple_generator;
mod routes;
mod helpers;

use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        generators: Mutex::new(load_generators()),
        default_generators: dotenv::var("DEFAULT_SETTINGS").unwrap().split(", ").map(|a| a.to_string()).collect(),
    });

    println!("[{}] [SERVER]: Server up! Open your preferred browser and access 「http://127.0.0.1:8080」!", Local::now().format(DF));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(state.clone())
            .service(
                web::scope("/sg")
                    .route("/randtext", web::get().to(random_text))
                    .route("/generators", web::get().to(get_available_generators))
                    .route("/settings", web::get().to(get_generator_settings))
                    .route("/save", web::post().to(save_generator))
            )
    })
    .bind(("[::]", 8080))?
    .run()
    .await
}

