use regex::RegexSet;

pub fn xsampa_to_ipa(input: String) -> String {
    let xsampa = ["p"];
    let ipa = ["p"];

    let regex_set = RegexSet::new(&xsampa).unwrap();
}
