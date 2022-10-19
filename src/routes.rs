use actix_files::NamedFile;
use actix_web::{web, Responder, Error, HttpResponse};
use std::sync::Mutex;

use crate::simple_generator::SimpleGenerator;

pub struct AppState {
    pub generator: Mutex<SimpleGenerator>,
}

pub async fn index() -> Result<NamedFile, Error> {
    println!("[API] Attempting to serve index.html");
    Ok(NamedFile::open("static/index.html")?)
}

// TODO: parse max word size and exact syllable number
pub async fn random_word(state: web::Data<AppState>) -> impl Responder {
    println!("[API] Generating word with setup: {}", state.generator.lock().unwrap().get_name());
    println!("[API] Testing generator print: {}", state.generator.lock().unwrap().get_generator_setup());
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_word(6, false)))
}

// TODO: parse text size
pub async fn random_text(state: web::Data<AppState>) -> impl Responder {
    println!("[API] Generating text with setup: {}", state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_text(50)))
}

