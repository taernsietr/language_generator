// use core::iter::zip;
use std::fs::read_dir;
// use std::path::PathBuf;
use std::collections::HashMap;
use std::env;
use regex::Regex;

use crate::simple_generator::SimpleGenerator;

fn extract_file_name(path: String) -> String {
    let re = Regex::new(r"([\w-]+)\.json").unwrap();
    let result = re.captures(&path).unwrap();
    result.get(1).unwrap().as_str().to_string()
}

pub fn load_generators() -> HashMap<String, SimpleGenerator> {

    let mut generators = HashMap::new();

    for file in read_dir(format!("{}/settings/", env::current_dir().unwrap().display())).unwrap() {
        generators.insert(
            extract_file_name(file.as_ref().unwrap().path().into_os_string().into_string().unwrap()),
            SimpleGenerator::load_pathbuf(file.unwrap().path())
        );
    };

    generators
}
