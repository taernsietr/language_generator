use serde::{Deserialize, Serialize};

enum PhonologicalTraits {
    Plosive,
    Unvoiced,
    Bilabial,
}

struct Phone {
    id: String,
    ipa: String,
    xsampa: String,
    traits: Vec<PhonologicalTraits>, 
}

