use std::sync::Mutex;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::TextGenerator;
use crate::log;

pub async fn get_random_generator(
    request: HttpRequest,
    generators: web::Data<Mutex<Vec<TextGenerator>>>
) -> impl Responder {
    log(&request, "Returning random generator".to_string());

    let random_generator = TextGenerator::random();
    let name = random_generator.get_name();

    match generators.lock() {
        Ok(mut guard) => {
            guard.push(random_generator);
            HttpResponse::Ok().body(
                serde_json::to_string(&name).expect("Unable to parse generator name.")
            )
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().body("Unable to parse generator names.")
        }
    }
}

