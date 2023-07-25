use bimap::BiMap;
use regex::Regex;

fn escape_regex(regex: &mut str) -> String {
    let re = Regex::new(r"([\\/\{\}\[\]\(\)\.\*\?])").unwrap();
    re.replace_all(regex, r"\$1").to_string()
}

pub fn xsampa_to_ipa(input: String, table: &BiMap<String, String>) -> String {
    let mut result = input.clone();
    let mut regexes = Vec::<Regex>::new();
    
    for each in table.left_values() {
        // println!("Escaping regex: {}", &each);
        let mut esc = each.clone();
        esc = escape_regex(esc.as_mut_str());
        // println!("Escaped regex: {}", &esc);
        regexes.push(Regex::new(&esc).unwrap());
    }

    for each in regexes {
        result = each.replace_all(&result, "000").to_string();
    }

    result
}

pub fn ipa_to_xsampa(input: String, table: &BiMap<String, String>) -> String {
    let mut result = String::new();
    let placeholder = "?".to_string();

    for each in input.chars() {
        result.push_str(table.get_by_right(&each.to_string()).unwrap_or_else(|| &placeholder))
    }
    result
}
