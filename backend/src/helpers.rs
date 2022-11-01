use core::iter::zip;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::simple_generator::SimpleGenerator;

pub fn load_generators(names: Vec<String>, files: Vec<PathBuf>) -> HashMap<String, SimpleGenerator> {
    // let mut generators = Vec::new();
    let mut generators = HashMap::new();

    for (name, file) in zip(names, files) {
        generators.insert(name, SimpleGenerator::load_pathbuf(file));
    };
    generators
}
