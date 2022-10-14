use std::fs;
use serde::Deserialize;

use crate::phonology::Phonology;
use crate::grammar::Grammar;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Language {
    phonology: Phonology,
    grammar: Grammar,
}

impl Language {
    pub fn load(file: &str) -> Language {
        let data = fs::read_to_string(file).expect("Failed to load language file");
        let language: Language = serde_yaml::from_str(&data).expect("Failed to read YAML data");
        language
    }
}
