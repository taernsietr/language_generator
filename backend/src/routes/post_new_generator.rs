use std::sync::Mutex;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::TextGenerator;
use crate::log;

pub async fn post_new_generator(
    request: HttpRequest,
    req_body: String,
    generators: web::Data<Mutex<Vec<TextGenerator>>>
) -> impl Responder {
    match serde_json::from_str::<TextGenerator>(&req_body) {
        Ok(new) => {
            match &generators.lock() {
                Ok(guard) => {
                    if !guard
                        .iter()
                        .any(|generator| generator.get_name() == new.get_name()) {
                            log(&request, format!("Generator [{}] created.", new.get_name()));
                            generators.lock().as_mut().unwrap().push(new);
                            HttpResponse::Ok().body("Generator created!")
                    }
                    else {
                        log(&request, format!("A generator with name [{}] already exists.", &new.get_name()));
                        HttpResponse::Conflict().body("A generator with that name already exists.")
                    }
                }
                Err(_poisoned) => {
                    eprintln!("Mutex acquisition failed.");
                    HttpResponse::InternalServerError().body("Unable to return generator settings.")
                }
            }
        },
        Err(_) => {
            log(&request, String::from("Failed to parse generator data."));
            HttpResponse::BadRequest().body("Couldn't parse generator.")
        }
    }
}

