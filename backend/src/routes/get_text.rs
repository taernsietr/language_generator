use std::sync::Mutex;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::*;
use crate::{log, types::WordParams};

pub async fn get_text(
    request: HttpRequest,
    query: web::Query<WordParams>,
    generators: web::Data<Mutex<Vec<TextGenerator>>>
) -> impl Responder {
    match &generators.lock() {
        Ok(guard) => {
            if let Some(result) = guard.iter()
                .find(|generator| generator.get_name() == query.generator) {
                    let query = query.into_inner();
                    let params: TextParams = query.clone().into();
                    let text = match params.text_type {
                        TextType::GenericWord => result.text(&params),
                        TextType::GenericPseudotext => result.pseudotext(&params)
                    };
                    log(&request, format!("Generating text with [{}]: length {}, words between {} to {} syllables",
                        &query.generator,
                        &params.text_size,
                        &params.min_syllables,
                        &params.max_syllables)
                    );
                    HttpResponse::Ok().body(text) 
                }
            else {
                log(&request, format!("Pseudotext requested for [{}], which wasn't found.", &query.generator));
                HttpResponse::NotFound().finish()
            }
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().finish()
        }
    }
}

