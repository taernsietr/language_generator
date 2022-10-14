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
    realizations: Vec<(String, String, String)>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Phone {
    symbol: String,
    features: Vec<Features>,
}
