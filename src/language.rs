use std::fs;
use serde::Deserialize;

use crate::phonology::Phonology;
use crate::grammar::Grammar;

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
/*
    pub fn random_word(&self, length: u8) -> String {
        let mut word: String = String::new();

        for _ in 0..length {
            for phoneme in &self.phonology.syllabic_pattern.unwrap().chars() {
                
            }
        }
    }
*/
}
