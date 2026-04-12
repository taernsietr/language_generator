use std::{ops::Deref, sync::Mutex};
use actix_web::web::Data;
use sqlx::PgPool;
use angelspeech::prelude::TextGenerator;

use crate::types::*;

pub async fn initialize_shared_data() -> Data<AppState> {
    let environment = dotenvy::var("ENVIRONMENT")
        .unwrap_or("DEVELOPMENT".into());

    let database_address = match environment.deref() {
        "TESTING" => dotenvy::var("DATABASE_URL")
            .expect("Couldn't find the environment variable for the Postgres database."),
        "DEVELOPMENT" | _ => "postgresql://angelspeech:scatman@[::1]:30010/angelspeech".into()
    };

    let database = PgPool::connect(&database_address)
        .await
        .expect("Couldn't connect to database.");

    _ = sqlx::query("CREATE TABLE IF NOT EXISTS gen ID uuid DEFAULT gen_random_uuid() PRIMARY KEY;")
        .execute(&database);

    let data: Vec<TextGenerator> = sqlx::query_as("SELECT * FROM gen;")
        .fetch_all(&database)
        .await
        .expect("Failed to retrieve generator data.");

    Data::from(
        std::sync::Arc::new(
            AppState {
                generators: Mutex::new(data),
                database
            }
        )
    )
}

