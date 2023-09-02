use regex::Regex;

const CONVERSION_TABLE: [(&str, &str); 103] = [("h\\", "ɦ"), ("j\\", "ʝ"), ("l\\", "ɺ"), ("p\\", "ɸ"), ("r\\`", "ɻ"), ("r\\", "ɹ"), ("s\\", "ɕ"), ("x\\", "ɧ"), ("z\\", "ʑ"), ("B\\", "ʙ"), ("G\\_<", "ʛ"), ("G\\", "ɢ"), ("H\\", "ʜ"), ("J\\_<", "ʄ"), ("J\\", "ɟ"), ("K\\", "ɮ"), ("L\\", "ʟ"), ("N\\", "ɴ"), ("M\\", "ɰ"), ("R\\", "ʀ"), ("X\\", "ħ"), ("@\\", "ɚ"), ("3\\", "ɞ"), ("?\\", "ʕ"), ("<\\", "ʢ"), (">\\", "ʡ"), ("b_<", "ɓ"), ("g_<", "ɠ"), ("d_<", "ɗ"), ("d`", "ɖ"), ("l`", "ɭ"), ("n`", "ɳ"), ("r`", "ɽ"), ("s`", "ʂ"), ("t`", "ʈ"), ("z`", "ʐ"), ("@`", "ə"), ("a", "a"), ("b", "b"), ("c", "c"), ("d", "d"), ("e", "e"), ("f", "f"), ("g", "g"), ("h", "h"), ("i", "i"), ("j", "j"), ("k", "k"), ("l", "l"), ("m", "m"), ("n", "n"), ("o", "o"), ("p", "p"), ("q", "q"), ("r", "r"), ("s", "s"), ("t", "t"), ("u", "u"), ("v", "v"), ("x", "x"), ("w", "w"), ("y", "y"), ("z", "z"), ("A", "ɑ"), ("B", "β"), ("C", "ç"), ("D", "ð"), ("E", "ɛ"), ("F", "ɱ"), ("G", "ɣ"), ("H", "ɥ"), ("I", "ɪ"), ("J", "ɲ"), ("K", "ɬ"), ("L", "ʎ"), ("M", "ɯ"), ("N", "ŋ"), ("O", "ɔ"), ("P", "ʋ"), ("Q", "ɒ"), ("R", "ʁ"), ("S", "ʃ"), ("T", "θ"), ("U", "ʊ"), ("V", "ʌ"), ("X", "χ"), ("W", "ʍ"), ("Y", "ʏ"), ("Z", "ʒ"), ("@", "ə"), ("{", "ɐ"), ("}", "ʉ"), ("1", "ɨ"), ("2", "ø"), ("3", "ɜ"), ("4", "ɾ"), ("5", "ɫ"), ("6", "ɶ"), ("7", "ɵ"), ("8", "ɵ"), ("9", "œ̞"), ("&", "ɶ̝"), ("?", "ʔ")];

pub fn escape_regex(regex: &str) -> String {
    let re = Regex::new(r"([\\/\{\}\[\]\(\)\.\*\?])").unwrap();
    re.replace_all(regex, r"\$1").to_string()
}

// Still compiles regex on every call
// .iter() returns elements in arbitrary order
pub fn xsampa_to_ipa(input: String) -> String {
    let mut result = input.clone();
    let mut regexes = Vec::<(Regex, String)>::new();
    
    for (left, right) in CONVERSION_TABLE.iter() {
        regexes.push((Regex::new(escape_regex(left).as_str()).unwrap(), right.to_string()));
    }

    for each in regexes {
        result = each.0.replace_all(&result, each.1).to_string();
    }

    result
}

pub fn ipa_to_xsampa(input: String) -> String {
    let mut result = String::new();

    for ipa in input.chars() {
        for (left, right) in CONVERSION_TABLE.iter() {
            if &ipa.to_string() == right { result.push_str(left) }
            else if ipa == ' ' { result.push(' ') }
        }
    }

    result
}
