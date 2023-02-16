use actix_files::NamedFile;
use actix_web::{web, Responder, Error, HttpRequest, HttpResponse};
use std::sync::Mutex;
use std::collections::HashMap;
use serde::Deserialize;

use crate::simple_generator::SimpleGenerator;
use crate::log;

pub struct AppState {
    pub generators: Mutex<HashMap<String, SimpleGenerator>>,
    pub default_generators: Vec<String>,
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
    
pub async fn random_word(request: HttpRequest, query: web::Query<WordParams>, state: web::Data<AppState>) -> impl Responder {
    log(request, format!("Generating word with generator [{}]", &query.generator));

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
    log(request, format!("Generating text with generator [{}], length {}", &query.generator, query.text_length));
    
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
    log(request, "Returning available generators".to_string());

    let mut response = format!("{{ \"generators\": [{}] }}", state.generators.lock().unwrap().values().map(|x| format!("\"{}\", ", x.get_name())).collect::<String>());
    let end = response.find(", ]").unwrap();
    response.replace_range(end..=end+1, "");

    HttpResponse::Ok().body(
        response
    )
}

pub async fn get_generator_settings(request: HttpRequest, query: web::Query<GenParams>, state: web::Data<AppState>) -> impl Responder {
    log(request, format!("Returning settings for generator [{}]", query.generator));

    HttpResponse::Ok().body(
        state.generators
            .lock()
            .unwrap()
            .get(&query.generator)
            .unwrap()
            .get()
    )
}

pub async fn save_new_generator(request: HttpRequest, req_body: String, state: web::Data<AppState>) -> impl Responder {
    let new_generator = serde_json::from_str::<SimpleGenerator>(&req_body).expect("Failed to read JSON data");
    let name = &new_generator.get_name(); // dirty workaround, is there a cleaner solution?
                                          //
    if !state.default_generators.contains(name) {
        log(request, format!("Received settings for new generator [{}]", name));
        new_generator.save();
        state.generators
            .lock()
            .unwrap()
            .insert(new_generator.get_name(), new_generator);
        HttpResponse::Ok().body("Settings saved!")
    }
    else {
        log(request, "Received settings for a new generator, but a reserved name was given.".to_string());
        HttpResponse::NoContent().body(format!("[{}] is a default generator name and cannot be used. Please, choose a different name!", name))
    }
}

pub async fn update_generator(request: HttpRequest, req_body: String, state: web::Data<AppState>) -> impl Responder {
    let new_generator = serde_json::from_str::<SimpleGenerator>(&req_body).expect("Failed to read JSON data");
    let name = &new_generator.get_name(); // dirty workaround, is there a cleaner solution?

    if !state.default_generators.contains(name) {
        log(request, format!("Updating settings for generator [{}]", name));
        new_generator.save();
        *state.generators.lock().unwrap().get_mut(name).unwrap() = new_generator;

        HttpResponse::Ok().body("Generator settings updated!")
    }
    else {
        log(request, "Received settings for updating a generator, but a reserved name was given.".to_string());
        HttpResponse::NoContent().body(format!("[{}] is a default generator name and cannot be used. Please, choose a different name!", name))
    }
}

pub async fn dbg(query: web::Query<GenParams>) -> impl Responder {
    dbg!("ué");
    HttpResponse::Ok().body(format!("[{}]", query.generator))
}
