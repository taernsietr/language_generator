use serde::{Deserialize, Serialize};
use rand::{Rng, prelude::SliceRandom};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;

use crate::pattern::{Pattern, PatternPosition};

#[derive(Deserialize, Serialize)]
pub struct TextGenerator {
    name: String,
    categories: HashMap<String,Vec<String>>, 
    patterns: Vec<Pattern>,
}

impl TextGenerator {
    #[allow(dead_code)]
    pub fn new(name: String, categories: HashMap<String, Vec<String>>, patterns: Vec<Pattern>) -> TextGenerator {
        TextGenerator { name, categories, patterns }
    }

    pub fn new_empty(name: String) -> TextGenerator {
        TextGenerator {
            name, 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
        }
    }

    pub fn load_pathbuf(file: PathBuf) -> TextGenerator {
        let data = std::fs::read_to_string(file).expect("Failed to load generator settings file");
        serde_json::from_str::<TextGenerator>(&data).expect("Failed to read JSON data")
    }
   
    pub fn save(&self) {
        std::fs::write(
            format!("{}/settings/{}.json", env::current_dir().unwrap().display(), &self.name),
            serde_json::to_string(&self).unwrap(),
        )
        .unwrap();
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn random_word(&self, min_syllables: u8, max_syllables: u8) -> String {
        let mut rng = rand::thread_rng();
        let mut word = "".to_string();

        let word_length: u8 = match min_syllables.cmp(&max_syllables) {
            Ordering::Less => { rng.gen_range(min_syllables..=max_syllables) },
            Ordering::Equal => { min_syllables },
            Ordering::Greater => { println!("[TextGenerator] Warning: Minimum syllables has to be equal to or less than maximum syllables"); min_syllables },
        };
        
        for index in 1..=word_length {
            // let current = self.patterns.choose(&mut rng);
            /* 
            initial
            non-final
            medial
            non-initial
            final
            
            index  = 1 && len  = 1 -> any, 
            index  = 1 && len >= 2 -> any, initial, non-final
            index  = 2 && len  = 2 -> any, non-final, medial, non-
            index  =  
            
            */ 
            
            let syllable_pattern = self.patterns
                .iter()
                .filter(|x| x.position() != PatternPosition::Medial)
                .cloned()
                .collect::<Vec<Pattern>>()
                .choose(&mut rng);

            for letter in current.unwrap().pattern().chars() {
                let chosen = self.categories.get(&letter.to_string()).unwrap().choose(&mut rng).unwrap();
                word.push_str(chosen);
            }
        }
        word
    }

    pub fn random_text(&self, min_syllables: u8, max_syllables: u8, text_size: u8) -> String {
        let mut text = String::new();

        for _ in 1..=text_size {
            text.push_str(&self.random_word(min_syllables, max_syllables));
            text.push(' ');
        }
        text
    }
}
