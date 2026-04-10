use std::sync::Mutex;
use serde::Deserialize;
use sqlx::PgPool;
use angelspeech::prelude::*;

pub struct AppState {
    pub generators: Mutex<Vec<TextGenerator>>,
    pub database: PgPool
}

#[derive(Deserialize)]
pub struct GeneratorNameParams {
    pub generator: String
}

#[derive(Deserialize, Clone)]
pub struct WordParams {
    pub generator: String,
    pub min: u8,
    pub max: u8,
    pub bias: f32,
    pub text_length: u8,
    pub text_type: TextType
}

impl Into<TextParams> for WordParams {
    fn into(self) -> TextParams {
        TextParams::new(
            self.min,
            self.max,
            self.bias,
            self.text_length,
            self.text_type
        )
    }
}

