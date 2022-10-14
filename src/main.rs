mod language;
mod phonology;
mod grammar;
mod generators;

use crate::language::Language;
use crate::generators::SimpleGenerator;

fn main() {
    let language = Language::load("./example.yaml");  

    let cats: Vec<Vec<String>> = vec!(["p", "t", "k", "s", "m", "n", "h", "w"].map(String::from).to_vec(), ["i", "a", "u"].map(String::from).to_vec());
    let syms: Vec<String> = ["C", "V"].map(String::from).to_vec();
    let pats: Vec<String> = ["CV"].map(String::from).to_vec();

    /*
    let cats = vec!(vec!("p", "t", "k", "s", "m", "n", "h", "w"), vec!("i", "a", "u"));
    let syms = vec!("C", "V");
    let pats = vec!("CV");
    */

    let generator = SimpleGenerator::new(cats, syms, pats);
    println!("{}", generator.random_word(3, false));
}
