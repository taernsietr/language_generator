use bimap::BiMap;
// use regex::{Regex, RegexSet, escape};

/*
fn escape_regex(regex: String) {
    let re = Regex::new(r"[\/{}[]().*<>]").unwrap();
    re.replace_all(regex
}
*/

// check current symbol with the following symbols, continue if the added symbols are
// valid xsampa. If a symbol that cannot be parsed as part of a xsampa representation or
// a suprasegmental, convert the sequence to IPA
pub fn xsampa_to_ipa(table: &BiMap<String, String>) -> String {
    table.get_by_left("S").unwrap().to_string()
}

pub fn ipa_to_xsampa(table: &BiMap<String, String>) -> String {
    table.get_by_right("ʃ").unwrap().to_string()
}
