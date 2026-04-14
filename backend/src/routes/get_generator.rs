use std::sync::Mutex;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::TextGenerator;
use crate::{log, types::GeneratorNameParams};

pub async fn get_generator(
    request: HttpRequest,
    query: web::Query<GeneratorNameParams>,
    generators: web::Data<Mutex<Vec<TextGenerator>>>
) -> impl Responder {
    match &generators.lock() {
        Ok(guard) => {
            if let Some(result) = guard.iter()
                .find(|generator| generator.get_name() == query.generator) {
                    log(&request, format!("Returning settings for [{}]", &query.generator));
                    let data = serde_json::to_string(result)
                        .expect("Unable to parse generator names");
                    HttpResponse::Ok().body(data)
                }
            else {
                log(&request, format!("[{}] not found", &query.generator));
                HttpResponse::NotFound().finish()
            }
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().finish()
        }
    }
}

