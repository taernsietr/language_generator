use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
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
            "Any" | "any" => Ok(PatternPosition::Any),
            "Initial" | "initial" => Ok(PatternPosition::Initial),
            "Medial" | "medial" => Ok(PatternPosition::Medial),
            "Final" | "final" => Ok(PatternPosition::Final),
            "NonInitial" | "noninitial" => Ok(PatternPosition::NonInitial),
            "NonMedial" | "nonmedial" => Ok(PatternPosition::NonMedial),
            "NonFinal" | "nonfinal" => Ok(PatternPosition::NonFinal),
            _ => Ok(PatternPosition::Any),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum PatternWeight {
    Default,
    Light,
    Heavy,
}

impl FromStr for PatternWeight {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternWeight, Self::Err> {

        match input {
            "Default" | "default" => Ok(PatternWeight::Default),
            "Light" | "light" => Ok(PatternWeight::Light),
            "Heavy" | "heavy" => Ok(PatternWeight::Heavy),
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
    pub fn new(pattern: String, position: String, weight: String) -> Pattern {
        Pattern { 
            pattern, 
            position: PatternPosition::from_str(&position).unwrap(), 
            weight: PatternWeight::from_str(&weight).unwrap(), 
        }
    }

    pub fn pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn position(&self) -> PatternPosition {
        self.position
    }

    pub fn weight(&self) -> PatternWeight {
        self.weight
    }
}
