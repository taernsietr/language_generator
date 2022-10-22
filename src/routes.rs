use actix_files::NamedFile;
use actix_web::{web, Responder, Error, HttpRequest, HttpResponse};
use chrono::Local;
use std::sync::Mutex;
use serde::Deserialize;

use crate::simple_generator::SimpleGenerator;

const DF: &str = "%H:%M:%S";

pub struct AppState {
    pub generator: Mutex<SimpleGenerator>,
    pub generators: Mutex<Vec<SimpleGenerator>>,
}

#[derive(Deserialize)]
pub struct WordParams {
    min: u8,
    max: u8,
    text_length: u8,
}

pub async fn index(request: HttpRequest) -> Result<NamedFile, Error> {
    println!("[API]: {} [{:?}] Attempting to serve index.html",
        Local::now().format(DF), 
        request.peer_addr().unwrap());
    Ok(NamedFile::open("static/index.html")?)
}

pub async fn random_word(request: HttpRequest, info: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    println!("[API]: {} [{:?}] Generating word with generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(state.generator.lock().unwrap().random_word(info.min, info.max))
}

pub async fn random_text(request: HttpRequest, info: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    println!("[API]: {} [{:?}] Generating text ({}) with generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        info.text_length,
        state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(state.generator.lock().unwrap().random_text(info.min, info.max, info.text_length))
}

pub async fn get_settings(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("[API]: {} [{:?}] Returning settings for generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        state.generator.lock().unwrap().get_name());
    web::Json(serde_json::from_str::<SimpleGenerator>(&state.generator.lock().unwrap().get_generator_setup()).expect("Failed to read JSON data"))
}

pub async fn new_settings(request: HttpRequest, req_body: String, state: web::Data<AppState>) -> impl Responder {
    *state.generator.lock().unwrap() = serde_json::from_str::<SimpleGenerator>(&req_body).expect("Failed to read JSON data");
    println!("[API]: {} [{:?}] Received settings for generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(format!("[API]: {} [{}] New settings received", Local::now().format(DF), request.peer_addr().unwrap()))
}
