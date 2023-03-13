use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand::prelude::{IteratorRandom, SliceRandom};
use crate::pattern::{Pattern, PatternPosition};

const CONSONANTS: [&str; 63] = ["p", "b", "t", "d", "t`", "d`", "c", "J\\", "k", "g", "q", "G\\", "?", "m", "F", "n", "n`", "J", "N", "N\\", "B\\", "r", "R\\", "4", "r`", "p\\", "B", "f", "v", "T", "D", "s", "z", "S", "Z", "s`", "z`", "C", "j\\", "x", "G", "X", "R", "X\\", "?\\", "h\\", "K", "K\\", "v\\", "r\\", "r\\`", "j", "M\\", "l", "l`", "L", "L\\", "W", "w", "H", "s\\", "z\\", "x\\"];
const VOWELS: [&str; 34] = ["i", "y", "1", "}", "M", "u", "I", "Y", "I\\", "U\\", "U", "e", "2", "@\\", "8", "7", "o", "e_o", "2_o", "@", "o_o", "E", "9", "3", "3\\", "V", "O", "{", "6", "a", "&", "a_", "A", "Q"];

#[derive(Deserialize, Serialize)]
pub struct TextGenerator {
    name: String,
    // #[serde(deserialize_with = "Self::deserialize")]
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

    // Optionally take parameters such as consonant/vowel ratio, inventory size, etc
    // This has to be rewritten more adequately
    pub fn new_fully_random() -> TextGenerator {
        let mut rng = rand::thread_rng();

        let ratio: f32 = rng.gen_range(0.2..=1.0);
        let size: u8 = rng.gen_range(5..=30);

        let consonants = CONSONANTS
            .choose_multiple(&mut rng, ((ratio * size as f32)/(ratio + 1.0)) as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();
        let vowels = VOWELS
            .choose_multiple(&mut rng, (size as f32 / (ratio + 1.0)) as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        dbg!(consonants.len(), vowels.len(), ratio, size);

        let no_consonant_cats = rng.gen_range(1..=3);
        let no_vowel_cats = rng.gen_range(1..=3);
        let no_pats = rng.gen_range(1..5);
        let mut categories = HashMap::<String, Vec<String>>::new(); 
        let mut patterns = Vec::<Pattern>::new();

        for i in 0..=no_consonant_cats {
            let category_size = rng.gen_range(1..=consonants.len());
            categories.insert(
                i.to_string(),
                consonants
                    .choose_multiple(&mut rng, category_size)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            );
        }
        
        for j in no_consonant_cats..=no_vowel_cats+no_consonant_cats {
            let category_size = rng.gen_range(1..=vowels.len());
            categories.insert(
                j.to_string(),
                vowels
                    .choose_multiple(&mut rng, category_size)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            );
        }
        
        for _ in 0..=no_pats {
            let pattern_size = rng.gen_range(1..=4);
            patterns.push(
                Pattern::new(
                    categories
                        .clone()
                        .into_keys()
                        .choose_multiple(&mut rng, pattern_size)
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<String>(),
                    "any".to_string(),
                    "default".to_string()
                )
            );
        }
        TextGenerator {
            name: "alpha".to_string(),
            categories,
            patterns,
        }
        // generate random categories
        // generate patterns
        // generate names
    }

    // TODO: Refactor this entire function, this is somewhat disgusting
    pub fn load_pathbuf(file: PathBuf) -> TextGenerator {
        let data = std::fs::read_to_string(file).expect("Failed to load generator settings file");
        let generator: TextGenerator = serde_json::from_str::<TextGenerator>(&data).expect("Failed to read JSON data");

        /* Error checking:
         * patterns must not have any symbol that isn't assigned to a category;
         * pattern symbols must be capital letters or numbers;
         * pattern symbols must be unique within a generator - using a symbol more than once will
         * overwrite the previous categories upon loading the generator;
         * ideally, the program should not panic when encountering these errors, but an elegant
         * solution has to be found first.
         */
        let result = {
            // TODO: make this check more than just the length
            let defined_symbols = generator.categories.keys();
            let mut used_symbols = Vec::<String>::new();

            for i in &generator.patterns {
                for j in i.pattern().chars() {
                    if j.is_uppercase() || j.is_numeric() {
                        used_symbols.push(j.to_string());
                    }
                }
            }
            used_symbols.sort();
            used_symbols.dedup();
            // dbg!(&defined_symbols, &used_symbols);
            used_symbols.len() <= defined_symbols.len()
        }; 
        if result { generator }
        else { panic!("Mismatch between number of defined and used pattern symbols"); }
    }
   
    // TODO: Either use a database or smarter file addresses; possibly both for development
    pub fn save(&self) {
        std::fs::write(
            // format!("{}/src/settings/{}.json", env::current_dir().unwrap().display(), &self.name),
            format!("{}/settings/{}.json", dotenv::var("SETTINGS").unwrap(), &self.name),
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
        let mut word = Vec::<String>::new();

        let word_length: u8 = match min_syllables.cmp(&max_syllables) {
            Ordering::Less => { 
                rng.gen_range({if min_syllables > 0 { min_syllables } else { 1 }}..=max_syllables)
            },
            Ordering::Equal => { if min_syllables > 0 { min_syllables } else { 1 } },
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

            for element in syllable_pattern.pattern().chars() {
                if element.is_uppercase() || element.is_numeric() { 
                    word.push(self.categories.get(&element.to_string()).unwrap().choose(&mut rng).unwrap().clone());
                }
                else if element.is_lowercase() { 
                    word.push(element.to_string());
                }
                else {
                    panic!("Invalid character in syllable pattern: {}", element);
                }
            }
        }
        word.concat()
    }

    pub fn random_text(&self, min_syllables: u8, max_syllables: u8, text_size: u8) -> String {
        let mut text = Vec::<String>::new();

        for _ in 1..=text_size {
            text.push(self.random_word(min_syllables, max_syllables));
        }
        text.join(" ")
    }
}