use actix_web::{Scope, web};

use crate::routes::{
    get_generator::get_generator,
    get_generators::get_generators,
    post_new_generator::post_new_generator,
    get_text::get_text,
    get_random_generator::get_random_generator
};

pub fn generators() -> Scope {
    web::scope("/generators")
        .route("", web::get().to(get_generators))
        .route("/settings", web::get().to(get_generator))
        .route("/text", web::get().to(get_text))
        .route("/random", web::get().to(get_random_generator))
        .route("/save", web::post().to(post_new_generator))
}

