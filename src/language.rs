use std::fs;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Language {
    phonology: Phonology,
    grammar: Grammar,
}

impl Language {
    #[allow(dead_code)]
    pub fn load(file: &str) -> Language {
        let data = fs::read_to_string(file).expect("Failed to load language file");
        let language: Language = serde_yaml::from_str(&data).expect("Failed to read YAML data");
        language
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Grammar {
    placeholder: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub enum Features {
    Stop,
    Plosive,
    Fricative,
    Approximant,
    Nasal,
    Lateral,
    LateralFricative,
    Implosive,
    Vowel,
    High,
    Center,
    Low,
    Front,
    Mid,
    Back,
    Rounded,
    Unrounded,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Phonology {
    name: String,
    phonemes: Vec<Phoneme>,
    syllabic_pattern: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Phoneme {
    symbol: String,
    realizations: Vec<(Phone, String, String)>, // realization, symbol, environment
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Phone {
    symbol: String,
    features: Vec<Features>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Word {
    phonemes: Vec<Phoneme>,
}

