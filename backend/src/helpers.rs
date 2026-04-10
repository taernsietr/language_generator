use std::sync::{Arc, Mutex};
use actix_web::web;
use sqlx::PgPool;
use angelspeech::prelude::TextGenerator;

use crate::types::*;

pub async fn initialize_shared_data() -> web::Data<AppState> {
    let database_address = dotenvy::var("POSTGRES_HOST")
        .expect("Couldn't find the environment variable for the Postgres database.");

    let database = PgPool::connect(&database_address)
        .await
        .expect("Couldn't connect to database.");

    let data: Vec<TextGenerator> = sqlx::query_as("SELECT * FROM generators")
        .fetch_all(&database)
        .await
        .expect("Failed to retrieve generator data.");

    web::Data::from(
        Arc::new(AppState {
            generators: Mutex::new(data),
            database
        })
    )
}

