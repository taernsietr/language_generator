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
    pub fn new_empty(name: String) -> TextGenerator {
        TextGenerator {
            name, 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
        }
    }

    pub fn load_pathbuf(file: PathBuf) -> TextGenerator {
        let data = std::fs::read_to_string(file).expect("Failed to load generator settings file");
        let generator: TextGenerator = serde_json::from_str::<TextGenerator>(&data).expect("Failed to read JSON data");
        let result = {
            // TODO: make this check more than just the length
            let defined_symbols = generator.categories.keys();
            let mut used_symbols = Vec::<String>::new();
            
            for i in &generator.patterns {
                for j in i.pattern().chars() {
                    used_symbols.push(j.to_string());
                }
            }
            used_symbols.sort();
            used_symbols.dedup();
            used_symbols.len() == defined_symbols.len()
        }; if result { generator }
        else { 
            panic!("[Error] Mismatch between number of defined and used pattern symbols");  
        }
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

    fn random_word(&self, min_syllables: u8, max_syllables: u8) -> String {
        let mut rng = rand::thread_rng();
        let mut word = "".to_string();

        let word_length: u8 = match min_syllables.cmp(&max_syllables) {
            Ordering::Less => { rng.gen_range(min_syllables..=max_syllables) },
            Ordering::Equal => { min_syllables },
            Ordering::Greater => { println!("[TextGenerator] Warning: Minimum syllables has to be equal to or less than maximum syllables"); min_syllables },
        };
        
        // Each syllable
        for index in 1..=word_length {
        /*  index  = 1     && len  =  1 -> any, non-medial
            index  = 1     && len >=  2 -> any, initial, non-final
            index  = len   && len  >  1 -> any, non-initial, final
        1 < index <= len-1 && len  >  2 -> any, non-final, medial, non-initial */
            let position_filters: Vec<PatternPosition> = match (index, word_length) {
                (1, 1) => {vec!(PatternPosition::Any, PatternPosition::NonMedial)},
                (1, 2..) => {vec!(PatternPosition::Any, PatternPosition::Initial, PatternPosition::NonMedial, PatternPosition::NonFinal)},
                (index, 2..) if index == word_length => {vec!(PatternPosition::Any, PatternPosition::NonInitial, PatternPosition::NonMedial, PatternPosition::Final)},
                (index, 3..) if 1 < index && index < word_length => {vec!(PatternPosition::Any, PatternPosition::NonFinal, PatternPosition::Medial, PatternPosition::NonInitial)},
                _ => unreachable!(),
            };
            /*
            // TODO: move this to a more adequate file
            pub enum WordMoraPattern {
                Any,
                Alternating,
                AlternatingDoubleHeavy,
                SingleHeavy,
                DoubleHeavy,
            }
            */

            /* 
            // TODO: Decide if implementing this is worth it
            let weight_filters: Vec<PatternWeight> = match (current_weight, word_mora_pattern) {
                ("L" || "H", WordMoraPattern::Any) => {},
                ("L", WordMoraPattern::Alternating) => {},
                ("H", WordMoraPattern::Alternating) => {},

            }
            */
            
            let syllable_pattern = self.patterns
                .iter()
                .filter(|x| position_filters.contains(&x.position()))
                .cloned()
                .collect::<Vec<Pattern>>()
                .choose(&mut rng)
                .unwrap()
                .to_owned();

            for letter in syllable_pattern.pattern().chars() {
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
