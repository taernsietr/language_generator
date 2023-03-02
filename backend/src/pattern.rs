use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum PatternPosition {
    Any,
    Initial,
    Medial,
    Final,
    NonInitial,
    NonMedial,
    NonFinal,
}

impl FromStr for PatternPosition {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternPosition, Self::Err> {

        match input {
            "Any" => Ok(PatternPosition::Any),
            "Initial" => Ok(PatternPosition::Initial),
            "Medial" => Ok(PatternPosition::Medial),
            "Final" => Ok(PatternPosition::Final),
            "NonInitial" => Ok(PatternPosition::NonInitial),
            "NonMedial" => Ok(PatternPosition::NonMedial),
            "NonFinal" => Ok(PatternPosition::NonFinal),
            _ => Ok(PatternPosition::Any),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum PatternWeight {
    Default,
    Light,
    Heavy,
}

impl FromStr for PatternWeight {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternWeight, Self::Err> {

        match input {
            "Default" => Ok(PatternWeight::Default),
            "Light" => Ok(PatternWeight::Light),
            "Heavy" => Ok(PatternWeight::Heavy),
            _ => Ok(PatternWeight::Default),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Pattern {
    pattern: String,
    position: PatternPosition,
    weight: PatternWeight,
}

impl Pattern {
    #[allow(dead_code)]
    pub fn new(pattern: String, position: String, weight: String) -> Pattern {
        Pattern {
            pattern,
            position: PatternPosition::from_str(&position).unwrap(),
            weight: PatternWeight::from_str(&weight).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn new_empty() -> Pattern {
        Pattern {
            pattern: " ".to_string(),
            position: PatternPosition::from_str(" ").unwrap(),
            weight: PatternWeight::from_str(" ").unwrap(),
        }
    }

    pub fn pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn position(&self) -> PatternPosition {
        self.position
    }
}
