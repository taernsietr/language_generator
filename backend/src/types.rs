use serde::Deserialize;
use std::{
    sync::Mutex,
    collections::HashMap,
    path::PathBuf,
};
use angelspeech::prelude::TextGenerator;

pub struct AppState {
    pub settings: PathBuf,
    pub generators: Mutex<HashMap<String, TextGenerator>>,
    pub default_generators: String,
    // pub conversion_table: Vec<(String, String)>,
}

#[derive(Deserialize)]
pub struct WordParams {
    pub generator: String,
    pub min: u8,
    pub max: u8,
    pub bias: f32,
    pub text_length: u8,
}

#[derive(Deserialize)]
pub struct GenParams {
    pub generator: String,
}
    
