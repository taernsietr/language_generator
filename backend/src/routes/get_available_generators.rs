use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::sync::Arc;
use crate::{log, types::AppState};

pub async fn get_available_generators(
    request: HttpRequest,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    log(&request, "Requested available generator names".to_string());

        let response = serde_json::to_string(
            &state.generators
                .lock()
                .unwrap()
                .keys()
                .cloned()
                .collect::<Vec<String>>()
        ).expect("Unable to parse default generator names");

    HttpResponse::Ok().body(response)
}

