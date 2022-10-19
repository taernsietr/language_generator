use serde::{Deserialize, Serialize};
use rand::{Rng, prelude::SliceRandom};

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

    pub fn load(file: &str) -> SimpleGenerator {
        let data = std::fs::read_to_string(format!("/home/tsrodr/Run/language_generator/src/settings/{}", file)).expect("Failed to load generator settings file");
        let generator: SimpleGenerator = serde_yaml::from_str(&data).expect("Failed to read YAML data");
        generator
    }
   
    #[allow(dead_code)]
    pub fn save(&self, name: &str) {
        std::fs::write(
            format!("/home/tsrodr/Run/language_generator/src/settings/{}.yaml", name),
            serde_yaml::to_string(&self).unwrap(),
        )
        .unwrap();
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_generator_setup(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn random_word(&self, max_syllables: u8, exactly: bool) -> String {
        let mut rng = rand::thread_rng();
        let mut word = "".to_string();
        
        let word_length = if exactly { max_syllables } else { rng.gen_range(1..=max_syllables) };

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

    pub fn random_text(&self, text_size: u8) -> String {
        let mut text = "".to_string();

        for _ in 1..=text_size {
            text.push_str(&self.random_word(5, false));
            text.push_str(" ");
        }
        text
    }
}
