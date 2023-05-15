use bimap::BiMap;
use regex::RegexSet;

fn escape_regex(regex: String) {
    /[-\/\\^$*+?.()|[\]{}]/g, '\\$&'
}

// check current symbol with the following symbols, continue if the added symbols are
// valid xsampa. If a symbol that cannot be parsed as part of a xsampa representation or
// a suprasegmental, convert the sequence to IPA
pub fn xsampa_to_ipa(input: String) -> String {
    let conversion_table: BiMap<String, String> = serde_json::from_str(&std::fs::read_to_string(format!("{}/resources/conversion_table.json", dotenv::var("SETTINGS").unwrap())).unwrap()).unwrap();
    let mut result: String = "".to_string();
    let mut regexes = RegexSet::new(conversion_table.left_values());
    regexes.unwrap().patterns().join(" ")
}

pub fn ipa_to_xsampa(input: String) -> String {
    let conversion_table: BiMap<String, String> = serde_json::from_str(&std::fs::read_to_string(format!("{}/resources/conversion_table.json", dotenv::var("SETTINGS").unwrap())).unwrap()).unwrap();
    let mut result: String = "".to_string();
    for char in input.chars() {
         result.push_str(&conversion_table.get_by_right(&char.to_string()).expect("failure looking up input"));
    }
    result
}
