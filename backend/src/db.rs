use std::ops::Deref;
use std::sync::Mutex;
use actix_web::web::Data;
use sqlx::{Execute, PgPool};
use angelspeech::prelude::TextGenerator;
use crate::log::log_query;

const BOOTSTRAP_SQL: [&str; 3] = [
    "DROP SCHEMA angelspeech CASCADE;",
    "CREATE SCHEMA angelspeech;",
    "CREATE TABLE angelspeech.generators ( name VARCHAR(128) PRIMARY KEY, categories JSONB NOT NULL, patterns JSONB NOT NULL, ruleset JSONB NOT NULL );",
];

const INSERT_GENERATOR: &str = "
    INSERT INTO angelspeech.generators (name, categories, patterns, ruleset) 
    VALUES ($1::TEXT, $2::JSONB, $3::JSONB, $4::JSONB);";

async fn prime_db(pool: &PgPool) {
    for query in BOOTSTRAP_SQL {
        log_query(query);
        match sqlx::query(query).execute(pool).await {
            Ok(_) => println!("Success!"),
            Err(e) => println!("Error!\n{}", e)
        }
    };

    //let example = TextGenerator::random();
    let example = TextGenerator::new(
        "example",
        vec!(
            ("C", vec!("p", "t", "k", "m", "n", "s", "h")),
            ("V", vec!("i", "a", "u"))
        ),
        vec!(
            ("CV", "Any", "Default")
        ),
        vec!(
            ("np", "mp")
        )
    );
    let query = sqlx::query(INSERT_GENERATOR)
        .bind(example.get_name())
        .bind(serde_json::to_string(&example.categories).unwrap())
        .bind(serde_json::to_string(&example.patterns).unwrap())
        .bind(serde_json::to_string(&example.ruleset).unwrap());
    log_query(query.sql());



    let _ = query
        .execute(pool)
        .await;
}

pub async fn initialize_pool() -> Data<PgPool> {
    let environment = std::env::var("ENVIRONMENT")
        .unwrap_or(String::from("DEVELOPMENT"));

    let database_address = match environment.deref() {
        "TESTING" => std::env::var("DATABASE_URL")
            .expect("Couldn't find the environment variable for the Postgres database."),
        _ => String::from("postgresql://angelspeech:scatman@[::1]:30010/angelspeech")
    };

    let pool = PgPool::connect(&database_address)
        .await
        .expect("Couldn't connect to database.");

    prime_db(&pool).await;
    Data::new(pool)
}

// TODO: convert this to load all relevant data from the DB
pub async fn initialize_generators(pool: &PgPool) -> Data<Mutex<Vec<TextGenerator>>> {
    let data: Vec<TextGenerator> = sqlx::query_as("SELECT * FROM angelspeech.generators;")
        .fetch_all(pool)
        .await
        .expect("Failed to retrieve generator data.");

    //let generators: Vec<TextGenerator> = data.iter().map(|g| g.clone()).collect();

    Data::from(std::sync::Arc::new(Mutex::new(data)))
}

