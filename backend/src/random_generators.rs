use std::collections::HashMap;
use rand::Rng;
use rand::prelude::{IteratorRandom, SliceRandom};

use crate::pattern::Pattern;
use crate::text_generator::TextGenerator;
use crate::convert::escape_regex;

const CONSONANTS: [&str; 63] = ["p", "b", "t", "d", "t`", "d`", "c", r"J\\", "k", "g", "q", r"G\\", "?", "m", "F", "n", "n`", "J", "N", r"N\\", r"B\\", "r", r"R\\", "4", "r`", r"p\\", "B", "f", "v", "T", "D", "s", "z", "S", "Z", "s`", "z`", "C", r"j\\", "x", "G", "X", "R", r"X\\", r"?\\", r"h\\", "K", r"K\\", r"v\\", r"r\\", "r\\`", "j", r"M\\", "l", "l`", "L", r"L\\", "W", "w", "H", r"s\\", r"z\\", r"x\\"];
const VOWELS: [&str; 34] = ["i", "y", "1", "}", "M", "u", "I", "Y", r"I\\", r"U\\", "U", "e", "2", r"@\\", "8", "7", "o", "e_o", "2_o", "@", "o_o", "E", "9", "3", r"3\\", "V", "O", "{", "6", "a", "&", "a_", "A", "Q"];

impl TextGenerator {

    // Optionally take parameters such as consonant/vowel ratio, inventory size, etc
    // This has to be rewritten more adequately
    pub fn new_random_generator() -> TextGenerator {
        let mut rng = rand::thread_rng();

        let ratio: f32 = rng.gen_range(0.2..=1.0);
        let size: u8 = rng.gen_range(5..=30);

        let consonants = CONSONANTS
            .choose_multiple(&mut rng, ((ratio * size as f32) / (ratio + 1.0)).round() as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let vowels = VOWELS
            .choose_multiple(&mut rng, (size as f32 / (ratio + 1.0)).round() as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let no_consonant_cats = rng.gen_range(1..=(consonants.len() as f32 / 2.0).round() as usize);
        let no_vowel_cats = rng.gen_range(1..=(vowels.len() as f32 / 2.0).round() as usize);
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

        let temp = TextGenerator::new("temp".to_string(), categories.clone(), patterns.clone());
        TextGenerator::new(temp.random_word(1, 4), categories, patterns)
    }
}
