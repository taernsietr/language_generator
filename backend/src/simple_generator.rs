use serde::{Deserialize, Serialize};
use rand::{Rng, prelude::SliceRandom};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct SimpleGenerator {
    name: String,
    categories: Vec<Vec<String>>, 
    symbols: Vec<String>,
    patterns: Vec<String>,
}

impl SimpleGenerator {
    #[allow(dead_code)]
    pub fn new(name: String, categories: Vec<Vec<String>>, symbols: Vec<String>, patterns: Vec<String>) -> SimpleGenerator {
        SimpleGenerator {name, categories, symbols, patterns}
    }

    #[allow(dead_code)]
    pub fn load_str(file: &str) -> SimpleGenerator {
        let data = std::fs::read_to_string(format!("{}/settings/{}", env::current_dir().unwrap().display(), file)).expect("Failed to load generator settings file");
        serde_json::from_str::<SimpleGenerator>(&data).expect("Failed to read JSON data")
    }
   
    pub fn load_pathbuf(file: PathBuf) -> SimpleGenerator {
        let data = std::fs::read_to_string(file).expect("Failed to load generator settings file");
        serde_json::from_str::<SimpleGenerator>(&data).expect("Failed to read JSON data")
    }
   
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn random_word(&self, min_syllables: u8, max_syllables: u8) -> String {
        let mut rng = rand::thread_rng();
        let mut word = "".to_string();

        let word_length: u8 = match min_syllables.cmp(&max_syllables) {
            Ordering::Less => { rng.gen_range(min_syllables..=max_syllables) },
            Ordering::Equal => { min_syllables },
            Ordering::Greater => { println!("[SimpleGenerator] Error: Minimum syllables has to be equal to or less than maximum syllables"); min_syllables },
        };
        
        for _ in 1..=word_length {
            let current = self.patterns.choose(&mut rng);
            for letter in current.unwrap().chars() {
                let index = self.symbols.iter().cloned().position(|x| x == letter.to_string()).unwrap();
                let chosen = self.categories[index].choose(&mut rng).unwrap();
                word.push_str(chosen);
            }
        }
        word
    }

    pub fn random_text(&self, min_syllables: u8, max_syllables: u8, text_size: u8) -> String {
        let mut text = String::new();

        for _ in 1..=text_size {
            text.push_str(&self.random_word(min_syllables, max_syllables));
            text.push_str(" ");
        }
        text
    }
}
