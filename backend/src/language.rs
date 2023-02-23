use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
enum PhonologicalTraits {
    Affricate,
    Alveolar,
    AlveoloPalatal,
    Apical,
    Aspirated,
    Approximant,
    Back,
    Bilabial,
    BreathyVoiced,
    Click,
    Closed,
    Coronal,
    CreakyVoiced,
    Dental,
    Ejective,
    Epiglottal,
    Flap,
    Fricative,
    Front,
    Glottal,
    HalfLong,
    Implosive,
    Labialized,
    Labiodental,
    Lateral,
    LateralFricative,
    Linguodental,
    Long,
    Medium,
    MediumClosed,
    MediumOpen,
    Mid,
    MidBack,
    MidFront,
    Nasal,
    Nasalized,
    Open,
    Palatal,
    Palatalized,
    Pharyngeal,
    Pharyngealized,
    Plosive,
    Pulmonic,
    Retroflex,
    Rhotic,
    Rounded,
    Short,
    Tap,
    Trill,
    Unrounded,
    Unvoiced,
    Uvular,
    Velar,
    Velarized,
    Voiced,
}

#[derive(Deserialize)]
enum SyllablePosition {
    WordInitial,
    WordFinal,
    WordMedial,
}

#[derive(Deserialize)]
struct Phone {
    id: String,
    ipa: String,
    xsampa: String,
    traits: Vec<PhonologicalTraits>, 
}

#[derive(Deserialize)]
struct Phoneme {
    id: String,
    canonical_realization: Vec<Phone>,
    allophonic_realizations: Option(Vec<(Phone, String)>, None),
}

#[derive(Deserialize)]
struct SyllabicPattern {
    id: String,
    form: Vec<Phoneme>,
    environment: Vec<SyllablePosition>,
    weight: u8,
}

#[derive(Deserialize)]
pub struct Phonology {
    name: String,
    phonemes: Vec<Phoneme>,
    syllabic_patterns: Vec<SyllabicPattern>,
}

#[derive(Deserialize)]
pub struct Language {
    phonology: Phonology,
    grammar: Grammar,
}

impl Language {
    pub fn load(file: &str) -> Language {
        let data = fs::read_to_string(file).expect("Failed to load language file");
        let language: Language = serde_json::from_str(&data).expect("Failed to read JSON data");
        language
    }
}

