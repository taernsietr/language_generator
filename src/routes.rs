use actix_files::NamedFile;
use actix_web::{web, Responder, Error, HttpRequest, HttpResponse};
use chrono::Local;
use std::sync::Mutex;

use crate::simple_generator::SimpleGenerator;

const DF: &str = "%H:%M:%S";

pub struct AppState {
    pub generator: Mutex<SimpleGenerator>,
}

pub async fn index(request: HttpRequest) -> Result<NamedFile, Error> {
    println!("[API]: {} [{:?}] Attempting to serve index.html",
        Local::now().format(DF), 
        request.peer_addr().unwrap());
    Ok(NamedFile::open("static/index.html")?)
}

// TODO: parse max word size and exact syllable number
pub async fn random_word(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("[API]: {} [{:?}] Generating {} word(s) with generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        "3",
        state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_word(3, 6, false)))
}

// TODO: parse text size
pub async fn random_text(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("[API]: {} [{:?}] Generating text ({}) with generator [{}]",
        Local::now().format(DF),
        request.peer_addr().unwrap(),
        "50",
        state.generator.lock().unwrap().get_name());
    HttpResponse::Ok().body(format!("{}", state.generator.lock().unwrap().random_text(3, 6, 50)))
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
