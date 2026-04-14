use angelspeech::prelude::TextGenerator;
use sqlx::{Execute, PgPool};
use crate::log::log_query;

const BOOTSTRAP_SQL: [&str; 3] = [
    "DROP SCHEMA angelspeech CASCADE;",
    "CREATE SCHEMA angelspeech;",
    "CREATE TABLE angelspeech.generators ( name VARCHAR(128) PRIMARY KEY, categories JSONB NOT NULL, patterns JSONB NOT NULL, ruleset JSONB NOT NULL );",
];

const INSERT_GENERATOR: &str = "
    INSERT INTO angelspeech.generators (name, categories, patterns, ruleset) 
    VALUES ($1::TEXT, $2::JSONB, $3::JSONB, $4::JSONB);";

pub async fn prime_db(pool: &PgPool) {
    for query in BOOTSTRAP_SQL {
        log_query(query);
        match sqlx::query(query).execute(pool).await {
            Ok(_) => println!("Success!"),
            Err(e) => println!("Error!\n{}", e)
        }
    };

    let example = TextGenerator::random();
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

