use std::sync::Arc;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use crate::{log, types::AppState};

pub async fn get_generators(
    request: HttpRequest,
    state: web::Data<Arc<AppState>>
) -> impl Responder {
    log(&request, "Requested available generator names".to_string());

    match &state.generators.lock() {
        Ok(guard) => {
            let data = serde_json::to_string(
                &guard.iter()
                    .map(|generator| { generator.get_name() })
                    .collect::<Vec<String>>()
            ).expect("Unable to parse generator names");
            HttpResponse::Ok().body(data)
        },
        Err(_poisoned) => {
            eprintln!("Mutex acquisition failed.");
            HttpResponse::InternalServerError().body("Mutex acquisition failed. Unable to parse generator names.")
        }
    }
}

