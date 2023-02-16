use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
enum PhonologicalTraits {
    Plosive,
    Fricative,
    Affricate,
    Approximant,
    Lateral,
    LateralFricative,
    Nasal,
    Tap,
    Flap,
    Trill,
    Implosive,
    Pulmonic,
    Ejective,
    Nasalized,
    Labialized,
    Unvoiced,
    Voiced,
    CreakyVoiced,
    BreathyVoiced,
    Bilabial,
    Labiodental,
    Linguodental,
    Dental,
    Alveolar,
    Coronal,
    Apical,
    Retroflex,
    Palatal,
    PalatoAlveolar,
    AlveoloPalatal,
    Velar,
    Uvular,
    Pharyngeal,
    Epiglottal,
    Glottal,
    Click,
    Aspirated,
    Palatalized,
    Velarized,
    Pharyngealized,
    Rhotic,
    Nasalized,
    Closed,
    MediumClosed,
    Medium,
    MediumOpen,
    Open,
    Front,
    MidFront,
    Mid,
    MidBack,
    Back,
    Rounded,
    Unrounded,
    Short,
    Long,
    HalfLong,
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
    allophonic_realizations: Vec<(Phone, String)>,
}

#[derive(Deserialize)]
struct SyllabicPattern {
    id: String,
    form: Vec<Phoneme>,
    environment: Vec<SyllablePosition>,
    weight: u8,
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

#[derive(Deserialize)]
pub struct Phonology {
    name: String,
    phonemes: Vec<Phoneme>,
    syllabic_patterns: Vec<SyllabicPattern>,
}

