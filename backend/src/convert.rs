use bimap::BiMap;
use regex::Regex;

fn escape_regex(regex: &mut str) -> String {
    let re = Regex::new(r"([\\/\{\}\[\]\(\)\.\*\?])").unwrap();
    re.replace_all(regex, r"\$1").to_string()
}

// Still compiles regex on every call
// .iter() returns elements in arbitrary order
pub fn xsampa_to_ipa(input: String, table: &BiMap<String, String>) -> String {
    let mut result = input.clone();
    let mut regexes = Vec::<(Regex, String)>::new();
    
    for (left, right) in table.iter() {
        let mut esc = left.clone();
        esc = escape_regex(esc.as_mut_str());
        regexes.push((Regex::new(&esc).unwrap(), right.clone()));
    }

    for each in regexes {
        result = each.0.replace_all(&result, each.1).to_string();
    }

    result
}

pub fn ipa_to_xsampa(input: String, table: &BiMap<String, String>) -> String {
    let mut result = String::new();
    let placeholder = "?".to_string();

    for each in input.chars() {
        result.push_str(table.get_by_right(&each.to_string()).unwrap_or(&placeholder))
    }
    result
}
