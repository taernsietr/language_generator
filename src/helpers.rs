use std::path::PathBuf;

use crate::simple_generator::SimpleGenerator;

pub fn load_generators(files: Vec<PathBuf>) -> Vec<SimpleGenerator> {
    let mut generators = Vec::new();

    for file in files {
        generators.push(SimpleGenerator::load_pathbuf(file));
    };
    generators
}
