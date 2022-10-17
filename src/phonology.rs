use serde::Deserialize;

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

/*
impl Word {
    fn parse(&self) {
        let mut output = "".to_string();

        for index in 0..self.phonemes.len() {
            for realization in self.phonemes[index] {
                
            }
        }
    }
}
*/
