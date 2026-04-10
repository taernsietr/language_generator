use std::sync::Arc;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::*;
use crate::{log, types::{WordParams, AppState}};

pub async fn get_text(
    request: HttpRequest,
    query: web::Query<WordParams>,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    match &state.generators.lock() {
        Ok(guard) => {
            if let Some(result) = guard.iter()
                .find(|generator| generator.get_name() == query.generator) {
                    let query = query.into_inner();
                    let params: TextParams = query.clone().into();
                    let text = match query.text_type {
                        TextType::GenericWord => result.text(&params),
                        TextType::GenericPseudotext => result.pseudotext(&params)
                    };
                    log(&request, format!("Generating text with [{}]: length {}, words between {} to {} syllables",
                        &query.generator,
                        &query.text_length,
                        &query.min,
                        &query.max)
                    );
                    HttpResponse::Ok().body(text) 
                }
            else {
                log(&request, format!("Pseudotext requested for [{}], which wasn't found.", &query.generator));
                HttpResponse::NotFound().body("Generator not found.")
            }
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().body("Unable to return generator settings.")
        }
    }
}

