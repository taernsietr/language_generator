use chrono::Local;
use actix_web::HttpRequest;

pub const DATE_FORMAT: &str = "%H:%M:%S";

pub fn log(req: &HttpRequest, text: String) {
    println!(
        "[{}] [SERVER: {:?}]: {}",
        Local::now().format(DATE_FORMAT),
        req.peer_addr().unwrap(),
        text
    );
}

pub fn server_start_msg<T: Into<String>>(server_address: T) {
    println!(
        "[{}] [SERVER]: Server up! Open your preferred browser and access 「http://{}」!",
        Local::now().format(DATE_FORMAT),
        &server_address.into()
    );
}

