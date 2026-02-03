use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::sync::Arc;
use crate::{log, types::{AppState, GenParams}};

pub async fn get_generator_settings(
    request: HttpRequest,
    query: web::Query<GenParams>,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
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

