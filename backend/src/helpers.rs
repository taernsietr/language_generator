use std::fs::read_dir;
use std::collections::HashMap;
use regex::Regex;
use chrono::Local;
use actix_web::HttpRequest;

use crate::text_generator::TextGenerator;

pub const DF: &str = "%H:%M:%S";

pub fn log(req: HttpRequest, text: String) {
    println!("[{}] [SERVER: {:?}]: {}",
        Local::now().format(DF),
        req.peer_addr().unwrap(),
        text
    );
}

fn extract_file_name(path: String) -> String {
    let re = Regex::new(r"([\w-]+)\.json").unwrap();
    // This is panicking if there is anything other than a json in the folder?
    let result = re.captures(&path).unwrap();
    result.get(1).unwrap().as_str().to_string()
}

pub fn load_generators() -> HashMap<String, TextGenerator> {
    let mut generators = HashMap::new();

    for file in read_dir(format!("{}/settings/", dotenv::var("SETTINGS").unwrap())).unwrap() {
        generators.insert(
            extract_file_name(file.as_ref().unwrap().path().into_os_string().into_string().unwrap()),
            TextGenerator::load_pathbuf(file.unwrap().path())
        );
    };

    generators
}

