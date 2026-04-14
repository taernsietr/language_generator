use std::sync::Mutex;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::TextGenerator;
use crate::log;

pub async fn get_generators(
    request: HttpRequest,
    generators: web::Data<Mutex<Vec<TextGenerator>>>
) -> impl Responder {
    log(&request, "Requested available generator names".to_string());

    match &generators.lock() {
        Ok(guard) => {
            let generators = &guard.iter()
                .map(|generator| { generator.get_name() })
                .collect::<Vec<String>>();
            if generators.is_empty() {
                HttpResponse::NoContent().finish()
            }
            else {
                let data = serde_json::to_string(generators)
                    .expect("Unable to parse generator names");
                HttpResponse::Ok().body(data)
            }
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().body("Mutex acquisition failed. Unable to parse generator names.")
        }
    }
}

