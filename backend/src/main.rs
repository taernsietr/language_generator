use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use chrono::Local;
use std::sync::Mutex;

// mod language;
mod text_generator;
mod pattern;
mod routes;
mod helpers;

use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server_address = dotenv::var("SERVER_ADDR").unwrap_or_else(|_| "[::1]:8080".to_string());
    let state = web::Data::new(AppState {
        generators: Mutex::new(load_generators()),
        default_generators: dotenv::var("DEFAULT_SETTINGS").unwrap().split(", ").map(|a| a.to_string()).collect(),
    });

    println!("[{}] [SERVER]: Server up! Open your preferred browser and access 「http://{}」!", Local::now().format(DF), server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(state.clone())
            .service(
                web::scope("/generators")
                    .route("", web::get().to(get_available_generators))
                    .route("/text", web::get().to(random_text))
                    .route("/settings", web::get().to(get_generator_settings))
                    .route("/save", web::post().to(save_generator))
            )
    })
    .bind(server_address)?
    .run()
    .await
}

