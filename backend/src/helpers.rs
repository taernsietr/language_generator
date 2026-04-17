use std::{ops::Deref, sync::Mutex};
use actix_web::web::Data;
use sqlx::PgPool;
use angelspeech::prelude::TextGenerator;

pub async fn initialize_pool() -> Data<PgPool> {
    let environment = std::env::var("ENVIRONMENT")
        .unwrap_or(String::from("DEVELOPMENT"));

    let database_address = match environment.deref() {
        "TESTING" => std::env::var("DATABASE_URL")
            .expect("Couldn't find the environment variable for the Postgres database."),
        _ => String::from("postgresql://angelspeech:scatman@[::1]:30010/angelspeech")
    };

    Data::new(
        PgPool::connect(&database_address)
            .await
            .expect("Couldn't connect to database.")
    )
}

// TODO: convert this to load all relevant data from the DB
pub async fn initialize_generators(pool: &PgPool) -> Data<Mutex<Vec<TextGenerator>>> {
    let data: Vec<TextGenerator> = sqlx::query_as("SELECT * FROM angelspeech.generators;")
        .fetch_all(pool)
        .await
        .expect("Failed to retrieve generator data.");

    Data::from(std::sync::Arc::new(Mutex::new(data)))
}

