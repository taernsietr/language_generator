use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::sync::Arc;
use angelspeech::prelude::TextGenerator;
use crate::{log, types::AppState};


pub async fn save_generator(
    request: HttpRequest,
    req_body: String,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    println!("DEBUG: {}", req_body);
    let new_generator: TextGenerator = serde_json::from_str::<TextGenerator>(&req_body).expect("Failed to read JSON data");
    let name = &new_generator.get_name();
 
    // TODO: handle this on the frontend
    if state.default_generators.contains(name) {
        log(&request, "Received settings for a new generator, but a reserved name was given.".to_string());
        HttpResponse::NoContent().body(format!("[{}] is a default generator name and cannot be used. Please, choose a different name!", &name))
    }
    else if state.generators.lock().unwrap().contains_key(name) {
        log(&request, format!("Updating settings for [{}]", &name));
        new_generator.save_local(state.settings.to_path_buf());
        *state.generators.lock().unwrap().get_mut(name).unwrap() = new_generator;
        HttpResponse::Ok().body("Generator settings updated!")
    }
    else {
        log(&request, format!("Received settings for new generator: [{}]", &name));
        new_generator.save_local(state.settings.to_path_buf());
        state.generators
            .lock()
            .unwrap()
            .insert(new_generator.get_name(), new_generator);
        HttpResponse::Ok().body("Settings saved!")
    }
}

