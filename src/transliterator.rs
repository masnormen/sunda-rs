use std::collections::HashMap;

use crate::charmap;
use fancy_regex::{Captures, Match};

fn get_string(i: usize, s: Option<Match>) -> (String, String) {
    let groups = vec![
        "",
        "digits",
        "punctuations",
        "consonant_main",
        "consonant_sonorant",
        "vowel",
        "consonant_rarangken",
        "consonant_final",
        "consonant_standalone",
    ];

    let string  = match s {
        Some(m) => m.as_str().to_string(),
        None => String::new(),
    };

    (groups[i].to_string(), string)
}

#[allow(unused)]
pub fn to_sundanese(text: &Captures) -> String {
    let get_latin = false;

    let matches: HashMap<String, String> = text.iter()
        .enumerate()
        .map(|(index, match_)| get_string(index, match_))
        .collect();

    // Transliterate digits
    if !matches["digits"].is_empty() {
        println!("{:?}", matches["digits"]);
        let numbers = charmap::get_angka(&matches["digits"].as_str(), get_latin);
        return format!("|{}|", numbers)
    }

    // Transliterate punctuations
    if !matches["punctuations"].is_empty() {
        return matches["punctuations"].to_owned()
    }

    // Transliterate standalone consonant
    if !matches["consonant_standalone"].is_empty() {
        let consonant = charmap::get_ngalagena(&matches["consonant_standalone"].as_str(), get_latin);
        let pamaeh = charmap::get_rarangken(&"pamaeh", get_latin);
        return format!("{consonant}{pamaeh}")
    }

    let mut output = String::new();

    if !matches["consonant_main"].is_empty() {
        output.push_str(&charmap::get_ngalagena(&matches["consonant_main"], get_latin));

        if !matches["consonant_sonorant"].is_empty() {
            output.push_str(&charmap::get_rarangken_sonorant(&matches["consonant_sonorant"], get_latin));
        }

        output.push_str(&charmap::get_rarangken(&matches["vowel"], get_latin));
    } else {
        output.push_str(&charmap::get_swara(&matches["vowel"], get_latin));
    }

    if !matches["consonant_rarangken"].is_empty() {
        output.push_str(&charmap::get_rarangken(&matches["consonant_rarangken"], get_latin));
    }

    if !matches["consonant_final"].is_empty() {
        let consonant = charmap::get_angka(&matches["consonant_final"], get_latin);
        let pamaeh = charmap::get_rarangken(&"pamaeh", get_latin);
        output.push_str(format!("{consonant}{pamaeh}").as_str());
    }

    output.to_string()
}
