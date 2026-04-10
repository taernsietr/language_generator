pub(crate) mod helpers;
pub(crate) mod routes;
pub(crate) mod scopes;
pub(crate) mod types;
pub(crate) mod log;

use actix_web::{App, HttpServer};
use actix_cors::Cors;

use crate::helpers::*;
use crate::log::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server_address = dotenvy::var("SERVER_ADDRESS")
        .expect("Couldn't find the environment variable for the server address.");
    let data = initialize_shared_data().await;
    
    server_start_msg(&server_address);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(data.clone())
            .service(scopes::generators())
    })
    .bind(server_address)?
    .run()
    .await
}

