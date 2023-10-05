use actix_web::{web, Responder, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
    path::PathBuf,
};

use angelspeech::{
    generator::text_generator::TextGenerator,
    utils::convert::{xsampa_to_ipa, ipa_to_xsampa}
};

use crate::log;

pub struct AppState {
    pub settings: PathBuf,
    pub generators: Arc<Mutex<HashMap<String, TextGenerator>>>,
    pub default_generators: Vec<String>, // change do Arc
    pub conversion_table: Vec<(String, String)>,
}

#[derive(Deserialize)]
pub struct WordParams {
    generator: String,
    min: u8,
    max: u8,
    text_length: u8,
}

#[derive(Deserialize)]
pub struct GenParams {
    generator: String,
}
    
pub async fn random_text(request: HttpRequest, query: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(&request, format!("Text requested for [{}], which wasn't found.", &query.generator));
            HttpResponse::NotFound().body(format!("Generator [{}] not found.", &query.generator))
        },
        Some(gen) => { 
            log(&request, format!("Generating text with [{}], length {}, with words of {} to {} syllables", &query.generator, &query.text_length, &query.min, &query.max));
            HttpResponse::Ok().body(gen.random_text(query.min, query.max, query.text_length)) 
        } 
    }
}

pub async fn get_available_generators(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    log(&request, "Requested available generator names".to_string());

    // TODO: refactor this
    let mut response = format!("{{ \"generators\": [{}] }}", state.generators.lock().unwrap().values().map(|x| format!("\"{}\", ", x.get_name())).collect::<String>());
    let end = response.find(", ]").unwrap();
    response.replace_range(end..=end+1, "");

    HttpResponse::Ok().body(
        response
    )
}

// TODO: refactor
// If the requested generator isn't found, assume the request used a freshly created name on the
// frontend and return an empty JSON
pub async fn get_generator_settings(request: HttpRequest, query: web::Query<GenParams>, state: web::Data<AppState>) -> impl Responder {
    match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(&request, format!("[{}] not found", &query.generator));
            HttpResponse::NotFound().body("Generator not found.")
        },
        Some(gen) => { 
            log(&request, format!("Returning settings for [{}]", &query.generator));
            HttpResponse::Ok().body(gen.as_json()) 
        }
    }
}

pub async fn save_generator(request: HttpRequest, req_body: String, state: web::Data<AppState>) -> impl Responder {
    let new_generator: TextGenerator = serde_json::from_str::<TextGenerator>(&req_body).expect("Failed to read JSON data");
    let name = &new_generator.get_name(); // dirty workaround, is there a cleaner solution?
 
    if state.default_generators.contains(name) {
        log(&request, "Received settings for a new generator, but a reserved name was given.".to_string());
        HttpResponse::NoContent().body(format!("[{}] is a default generator name and cannot be used. Please, choose a different name!", &name))
    }
    else if state.generators.lock().unwrap().contains_key(name) {
        log(&request, format!("Updating settings for [{}]", &name));
        new_generator.save_local(state.settings.clone());
        *state.generators.lock().unwrap().get_mut(name).unwrap() = new_generator;
        HttpResponse::Ok().body("Generator settings updated!")
    }
    else {
        log(&request, format!("Received settings for new generator: [{}]", &name));
        new_generator.save_local(state.settings.clone());
        state.generators
            .lock()
            .unwrap()
            .insert(new_generator.get_name(), new_generator);
        HttpResponse::Ok().body("Settings saved!")
    }
}

pub async fn random_generator(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    log(&request, "Returning random generator".to_string());

    let random_generator = TextGenerator::random();
    let name = random_generator.get_name();
    state.generators
        .lock()
        .unwrap()
        .insert(random_generator.get_name(), random_generator);
    HttpResponse::Ok().body(serde_json::to_string(&name).expect("Unable to parse generator name."))
}

pub async fn convert_xsampa_to_ipa(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(xsampa_to_ipa(req_body.to_string()))
}

pub async fn convert_ipa_to_xsampa(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(ipa_to_xsampa(req_body.to_string()))
}

