use std::sync::Arc;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use crate::{log, types::{AppState, GeneratorNameParams}};

pub async fn get_generator(
    request: HttpRequest,
    query: web::Query<GeneratorNameParams>,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    match &state.generators.lock() {
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
                HttpResponse::NotFound().body("Generator not found.")
            }
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().body("Unable to return generator settings.")
        }
    }
}

