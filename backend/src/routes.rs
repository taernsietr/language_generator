use actix_web::{web, Responder, HttpRequest, HttpResponse};
use bimap::BiMap;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::Deserialize;

use crate::text_generator::TextGenerator;
use crate::log;

pub struct AppState {
    pub generators: Mutex<HashMap<String, TextGenerator>>,
    pub default_generators: Vec<String>,
    pub conversion_table: BiMap<String, String>,
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
    let response = match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(request, format!("Text requested for [{}], which wasn't found.", &query.generator));
            HttpResponse::Ok().body(format!("Generator [{}] not found.", query.generator))
        },
        Some(gen) => { 
            log(request, format!("Generating text with [{}], length {}, with words of {} to {} syllables", &query.generator, query.text_length, query.min, query.max));
            HttpResponse::Ok().body(gen.random_text(query.min, query.max, query.text_length)) 
        } 
    };
    response
}

pub async fn get_available_generators(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    log(request, "Returning available generators".to_string());

    let mut response = format!("{{ \"generators\": [{}] }}", state.generators.lock().unwrap().values().map(|x| format!("\"{}\", ", x.get_name())).collect::<String>());
    let end = response.find(", ]").unwrap();
    response.replace_range(end..=end+1, "");

    HttpResponse::Ok().body(
        response
    )
}

// If the requested generator isn't found, assume the request used a freshly created name on the
// frontend and return an empty JSON
pub async fn get_generator_settings(request: HttpRequest, query: web::Query<GenParams>, state: web::Data<AppState>) -> impl Responder {
    let response = match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(request, format!("[{}] not found, returning empty settings", query.generator));
            HttpResponse::Ok().body( TextGenerator::new_empty(query.generator.clone()).get()) 
        },
        Some(gen) => { 
            log(request, format!("Returning settings for [{}]", query.generator));
            HttpResponse::Ok().body(gen.get()) 
        }
    };
    response
}

pub async fn save_generator(request: HttpRequest, req_body: String, state: web::Data<AppState>) -> impl Responder {
    let new_generator = serde_json::from_str::<TextGenerator>(&req_body).expect("Failed to read JSON data");
    let name = &new_generator.get_name(); // dirty workaround, is there a cleaner solution?
 
    if state.default_generators.contains(name) {
        log(request, "Received settings for a new generator, but a reserved name was given.".to_string());
        HttpResponse::NoContent().body(format!("[{}] is a default generator name and cannot be used. Please, choose a different name!", name))
    }
    else if state.generators.lock().unwrap().contains_key(name) {
        log(request, format!("Updating settings for [{}]", name));
        new_generator.save();
        *state.generators.lock().unwrap().get_mut(name).unwrap() = new_generator;
        HttpResponse::Ok().body("Generator settings updated!")
    }
    else {
        log(request, format!("Received settings for new generator: [{}]", name));
        new_generator.save();
        state.generators
            .lock()
            .unwrap()
            .insert(new_generator.get_name(), new_generator);
        HttpResponse::Ok().body("Settings saved!")
    }
}

pub async fn random_generator(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    log(request, "Returning random generator".to_string());

    let random_generator = TextGenerator::new_fully_random();
    state.generators
        .lock()
        .unwrap()
        .insert(random_generator.get_name(), random_generator);
    HttpResponse::Ok().body("New random generator created!")
}

pub async fn convert_xsampa_to_ipa(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(crate::convert::xsampa_to_ipa("SaJ\\_<bbaH\\".to_string(), &state.conversion_table))
}

pub async fn convert_ipa_to_xsampa(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(crate::convert::ipa_to_xsampa("θøːɬɯɾ".to_string(), &state.conversion_table))
}
