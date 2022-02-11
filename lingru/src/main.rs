use std:: {
    io,
    fs,
    path
};
use rand::Rng;
use serde::Deserialize;

// Represents a group of sounds (represented as graphemes), syllabic patterns and phonological rules
struct Language < 'a > {
    id: usize,
    name: String,
    categories: Vec < Category < 'a>>,
    patterns: Vec < Pattern < 'a>>
}

// Represents a group of graphemes
struct Category < 'a > {
    id: usize,
    index: char,
    members: Vec<&'a str >,
    weights: Vec < u8 >
}

// Represents a syllabic pattern
struct Pattern < 'a > {
    id: usize,
    format: &'a str,
    weight: u8
}

fn load_languages(Path::) {

    let

}

fn main() {

    let json_path = path::Path::new("../../languages.json");
    let file = fs::File::open(json_path);

}