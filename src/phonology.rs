use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct Phonology {
    name: String,
    phonemes: Vec<Phoneme>,
    syllabic_pattern: String,
}

#[derive(Deserialize)]
struct Phoneme {
    symbol: String,
    realizations: Vec<(String, String, String)>,
}

#[derive(Deserialize)]
struct Phone {
    symbol: String,
    features: Vec<Features>,
}
