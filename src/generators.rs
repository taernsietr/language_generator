use rand::{Rng, prelude::SliceRandom};

pub struct SimpleGenerator {
    categories: Vec<Vec<String>>, 
    symbols: Vec<String>,
    patterns: Vec<String>,
}

impl SimpleGenerator {
    pub fn new(categories: Vec<Vec<String>>, symbols: Vec<String>, patterns: Vec<String>) -> SimpleGenerator {
        SimpleGenerator {categories, symbols, patterns}
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
}
