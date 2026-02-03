use actix_web::{web, Responder, HttpRequest, HttpResponse};
use angelspeech::prelude::TextGenerator;
use std::sync::Arc;
use crate::{log, types::AppState};

pub async fn random_generator(
    request: HttpRequest,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    log(&request, "Returning random generator".to_string());

    let random_generator = TextGenerator::random();
    let name = random_generator.get_name();
    state.generators
        .lock()
        .unwrap()
        .insert(random_generator.get_name(), random_generator);
    HttpResponse::Ok().body(serde_json::to_string(&name).expect("Unable to parse generator name."))
}

