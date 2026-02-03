use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::sync::Arc;
use crate::{log, types::{WordParams, AppState}};

pub async fn pseudotext(
    request: HttpRequest,
    query: web::Query<WordParams>,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(&request, format!("Pseudotext requested for [{}], which wasn't found.", &query.generator));
            HttpResponse::NotFound().body(format!("Generator [{}] not found.", &query.generator))
        },
        Some(gen) => { 
            log(&request, format!("Generating pseudotext with [{}], length {}, with words of {} to {} syllables",
                &query.generator,
                &query.text_length,
                &query.min,
                &query.max)
            );
            HttpResponse::Ok().body(gen.pseudotext(query.min, query.max, query.bias, query.text_length)) 
        } 
    }
}

