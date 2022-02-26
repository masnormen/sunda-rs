use crate::charmap;
use std::collections::HashMap;
use fancy_regex::Captures;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn to_sundanese(groups: &[String], text: &Captures) -> String {
    let get_latin = false;

    let matches: HashMap<String, String> = text
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(index, match_)| {
            (
                groups[index].to_string(),
                match match_ {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                },
            )
        })
        .collect();

    // Transliterate digits
    if !matches["digits"].is_empty() {
        println!("{:?}", matches["digits"]);
        let numbers = charmap::get_angka(matches["digits"].as_str(), get_latin);
        return format!("|{}|", numbers);
    }

    // Transliterate punctuations
    if !matches["punctuations"].is_empty() {
        return matches["punctuations"].to_owned();
    }

    // Transliterate standalone consonant
    if !matches["consonant_standalone"].is_empty() {
        let consonant = charmap::get_ngalagena(matches["consonant_standalone"].as_str(), get_latin);
        let pamaeh = charmap::get_rarangken("pamaeh", get_latin);
        return format!("{consonant}{pamaeh}");
    }

    let mut output = String::new();

    if !matches["consonant_main"].is_empty() {
        output.push_str(&charmap::get_ngalagena(
            &matches["consonant_main"],
            get_latin,
        ));

        if !matches["consonant_sonorant"].is_empty() {
            output.push_str(&charmap::get_rarangken_sonorant(
                &matches["consonant_sonorant"],
                get_latin,
            ));
        }

        output.push_str(&charmap::get_rarangken(&matches["vowel"], get_latin));
    } else {
        output.push_str(&charmap::get_swara(&matches["vowel"], get_latin));
    }

    if !matches["consonant_rarangken"].is_empty() {
        output.push_str(&charmap::get_rarangken(
            &matches["consonant_rarangken"],
            get_latin,
        ));
    }

    if !matches["consonant_final"].is_empty() {
        let consonant = charmap::get_angka(&matches["consonant_final"], get_latin);
        let pamaeh = charmap::get_rarangken("pamaeh", get_latin);
        output.push_str(format!("{consonant}{pamaeh}").as_str());
    }

    output.to_string()
}
