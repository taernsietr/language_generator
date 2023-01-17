use actix_files::NamedFile;
use actix_web::{web, Responder, Error, HttpRequest, HttpResponse};
use chrono::Local;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::Deserialize;

use crate::simple_generator::SimpleGenerator;

const DF: &str = "%H:%M:%S";

pub struct AppState {
    pub generators: Mutex<HashMap<String, SimpleGenerator>>,
}

#[derive(Deserialize)]
pub struct WordParams {
    generator: String,
    min: u8,
    max: u8,
    text_length: u8,
}
    
fn log(fmt: &str, req: HttpRequest, text: String) {
    println!("[{}] [SERVER: {:?}]: {}",
        Local::now().format(fmt),
        req.peer_addr().unwrap(),
        text
    );
}

pub async fn index(request: HttpRequest) -> Result<NamedFile, Error> {
    log(DF, request, format!("Attempting to serve index.html"));
    Ok(NamedFile::open("static/index.html")?)
}

pub async fn random_word(request: HttpRequest, query: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    log(DF, request, format!("Generating word with generator [{}]", &query.generator));

    HttpResponse::Ok().body(
        state.generators
            .lock()
            .unwrap()
            .get(&query.generator)
            .unwrap()
            .random_word(query.min, query.max)
    )
}

pub async fn random_text(request: HttpRequest, query: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    log(DF, request, format!("Generating text with generator [{}], length {}", &query.generator, query.text_length));
    
    HttpResponse::Ok().body(
        state.generators
            .lock()
            .unwrap()
            .get(&query.generator)
            .unwrap()
            .random_text(query.min, query.max, query.text_length)
    )
}

pub async fn get_available_generators(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    log(DF, request, format!("Returning available generators"));
    
    HttpResponse::Ok().body(
        format!("{}", state.generators.lock().unwrap().values().map(|x| x.get_name() + "\n").collect::<String>())
    )
}

// TODO: if generator exists, update, otherwise, create new generator file
pub async fn save_settings(request: HttpRequest, query: web::Query<WordParams>, req_body: String, state: web::Data<AppState>) -> impl Responder {
    let new_generator = serde_json::from_str::<SimpleGenerator>(&req_body).expect("Failed to read JSON data");

    state.generators
        .lock()
        .unwrap()
        .insert(new_generator.get_name(), new_generator);
    
    log(DF, request, format!("Received settings for generator [{}]", &query.generator));
    
    HttpResponse::Ok().body(format!("Settings saved!"))
}
