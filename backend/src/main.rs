use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use chrono::Local;
use std::sync::Mutex;

use bimap::BiHashMap;

mod helpers;
mod routes;
mod text_generator;
mod pattern;
mod convert;
// mod language;

use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server_address = dotenv::var("SERVER_ADDR").unwrap_or_else(|_| "[::1]:8080".to_string());
    let state = web::Data::new(AppState {
        generators: Mutex::new(load_generators()),
        default_generators: dotenv::var("DEFAULT_SETTINGS").unwrap().split(", ").map(|a| a.to_string()).collect(),
    });

    let address = format!("{}/resources/conversion_table.json", dotenv::var("SETTINGS").unwrap());
    let contents: String = std::fs::read_to_string(address).unwrap();
    let conversion_table: BiHashMap<String, String> = serde_json::from_str(&contents).unwrap();
    dbg!(conversion_table);

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
                    .route("/testing", web::get().to(random))
            )
    })
    .bind(server_address)?
    .run()
    .await
}

