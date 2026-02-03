use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::sync::Arc;
use crate::{log, types::{WordParams, AppState}};

pub async fn random_words(
    request: HttpRequest,
    query: web::Query<WordParams>,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    match state.generators.lock().unwrap().get(&query.generator) {
        None => { 
            log(&request, format!("Words requested for [{}], which wasn't found.", &query.generator));
            HttpResponse::NotFound().body(format!("Generator [{}] not found.", &query.generator))
        },
        Some(gen) => { 
            log(&request, format!("Generating words with [{}], length {}, with words of {} to {} syllables",
                &query.generator,
                &query.text_length,
                &query.min,
                &query.max)
            );
            let text = {
                let mut text = gen.random_text(query.min, query.max, query.bias, query.text_length);
                for rule in gen.ruleset.iter() {
                    text = rule.apply(&text);
                };
                text
            };
            HttpResponse::Ok().body(text) 
        } 
    }
}

