mod helpers;
mod routes;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use chrono::Local;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use crate::helpers::*;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = dotenv::var("SERVER_ADDR").unwrap_or_else(|_| "[::1]:8080".to_string());

    let arc_data = web::Data::new(
        AppState {
            settings: Arc::new(PathBuf::from(dotenv::var("SETTINGS").unwrap())),
            generators: Arc::new(Mutex::new(load_generators(PathBuf::from(dotenv::var("SETTINGS").unwrap())))),
            default_generators: Arc::new(dotenv::var("DEFAULT_GENERATORS").unwrap().split(", ").map(|a| a.to_string()).collect()),
            conversion_table: Arc::new(serde_json::from_str(&std::fs::read_to_string(PathBuf::from(dotenv::var("RESOURCES").unwrap()).join("conversion_table.json")).unwrap()).unwrap())
        }
    );

    println!(
        "[{}] [SERVER]: Server up! Open your preferred browser and access 「http://{}」!",
        Local::now().format(DATE_FORMAT),
        &server_address
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(arc_data.clone())
            .service(
                web::scope("/generators")
                    .route("", web::get().to(get_available_generators))
                    .route("/words", web::get().to(random_words))
                    .route("/pseudotext", web::get().to(pseudotext))
                    .route("/settings", web::get().to(get_generator_settings))
                    .route("/save", web::post().to(save_generator))
                    .route("/random_generator", web::get().to(random_generator))
                    .route("/xsampa-ipa", web::post().to(convert_xsampa_to_ipa))
                    .route("/ipa-xsampa", web::post().to(convert_ipa_to_xsampa))
            )
            .service(
                web::scope("/ipa")
                    .route("", web::get().to(ipa_resources))
            )
    })
    .bind(server_address)?
    .run()
    .await
}

