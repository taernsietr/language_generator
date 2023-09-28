use std::fs::read_dir;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::Local;
use actix_web::HttpRequest;
 
use angelspeech::generator::text_generator::TextGenerator;

pub const DATE_FORMAT: &str = "%H:%M:%S";

pub fn log(req: &HttpRequest, text: String) {
    println!("[{}] [SERVER: {:?}]: {}",
        Local::now().format(DATE_FORMAT),
        req.peer_addr().unwrap(),
        text
    );
}

pub fn load_generators(settings: PathBuf) -> HashMap<String, TextGenerator> {
    let mut generators = HashMap::new();

    let setting_files = read_dir(settings.as_os_str()).unwrap();

    for file in setting_files {
        let file_name = file
            .as_ref()
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let generator = TextGenerator::load_local(
            file.unwrap().path()
        );

        generators.insert(file_name, generator);
    };

    generators
}


