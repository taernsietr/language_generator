use std::fs::read_dir;
use std::collections::HashMap;
// use std::fmt::{Formatter, Result};
// use serde::{Deserializer, self, de::Visitor};
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

// TextGenerator deserialization

/*
struct CategoryVisitor;

impl<'de> Visitor<'de> for CategoryVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("JSON of the general form { categories: [ { symbol: string, elements: [ string ] } ] }") 
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(String::from(value))
    }
}

impl<'de> Deserialize<'de> for String {
    fn deserialize<D>(deserializer: D)
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_string(CategoryVisitor)
        }
}
*/
