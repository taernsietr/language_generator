pub(crate) mod types;
pub(crate) mod db;
pub(crate) mod routes;
pub(crate) mod scopes;
pub(crate) mod log;

use actix_web::{
    App,
    HttpServer,
    middleware::{Logger,NormalizePath}
};
use actix_cors::Cors;

use crate::db::*;
use crate::log::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server_address = std::env::var("SERVER_URL")
        .unwrap_or(String::from("0.0.0.0:30011"));

    let pool = initialize_pool().await;
    let generators = initialize_generators(&pool).await;
    
    server_start_msg(&server_address);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(actix_web::middleware::TrailingSlash::Trim))
            .app_data(pool.clone())
            .app_data(generators.clone())
            .service(scopes::generators())
    })
    .keep_alive(actix_web::http::KeepAlive::Timeout(core::time::Duration::new(60,0)))
    .bind(server_address)?
    .run()
    .await
}

