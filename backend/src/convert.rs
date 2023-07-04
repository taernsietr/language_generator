use bimap::BiMap;
use regex::{RegexSet, escape};

/*
fn escape_regex(regex: String) {
    let re = Regex::new(r"[\/{}[]().*<>]").unwrap();
    re.replace_all(regex
}
*/

// check current symbol with the following symbols, continue if the added symbols are
// valid xsampa. If a symbol that cannot be parsed as part of a xsampa representation or
// a suprasegmental, convert the sequence to IPA
pub fn xsampa_to_ipa(table: BiMap<String, String>) -> Vec<String> {
    let conversion_table: BiMap<String, String> = serde_json::from_str(&std::fs::read_to_string(format!("{}/resources/conversion_table.json", dotenv::var("SETTINGS").unwrap())).unwrap()).unwrap();
    let regexes = RegexSet::new(conversion_table.left_values().map(|x| escape(x)));
    // for (i, p) in regexes.unwrap().patterns().iter().enumerate() {
    //     println!("{}: {}", i, p);
    // }
    let result: Vec<String> = regexes.unwrap().matches("J\\_<abaT").into_iter().map(|x| x.to_string()).collect();
    result
}

/*
pub fn ipa_to_xsampa(input: String) -> String {
    let conversion_table: BiMap<String, String> = serde_json::from_str(&std::fs::read_to_string(format!("{}/resources/conversion_table.json", dotenv::var("SETTINGS").unwrap())).unwrap()).unwrap();
    let mut result: String = "".to_string();
    for char in input.chars() {
         result.push_str(&conversion_table.get_by_right(&char.to_string()).expect("failure looking up input"));
    }
    result
}
*/
