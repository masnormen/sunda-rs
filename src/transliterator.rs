use crate::charmap::{
    get_angka, get_ngalagena, get_rarangken, get_rarangken_sonorant, get_swara, CHAR_RARANGKEN,
};
use fancy_regex::Captures;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{collections::HashMap, ops::Not};

/// Takes `Captures` from Latin and returns the transliterated Sundanese `String`.
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
    if matches["digits"].is_empty().not() {
        let numbers = matches["digits"]
            .chars()
            .map(|digit| get_angka(digit.encode_utf8(&mut [0; 4]), get_latin))
            .collect::<Vec<String>>()
            .join("");
        return format!("|{}|", numbers);
    }

    // Transliterate punctuations
    if matches["punctuations"].is_empty().not() {
        return matches["punctuations"].to_owned();
    }

    // Transliterate standalone consonant
    if matches["consonant_standalone"].is_empty().not() {
        let consonant = get_ngalagena(matches["consonant_standalone"].as_str(), get_latin);
        let pamaeh = get_rarangken("pamaeh", get_latin);
        return format!("{consonant}{pamaeh}");
    }

    // Transliterate anything else
    let mut output = String::new();

    if matches["consonant_main"].is_empty().not() {
        output.push_str(&get_ngalagena(&matches["consonant_main"], get_latin));
        if matches["consonant_sonorant"].is_empty().not() {
            output.push_str(&get_rarangken_sonorant(
                &matches["consonant_sonorant"],
                get_latin,
            ));
        }
        output.push_str(&get_rarangken(&matches["vowel"], get_latin));
    } else {
        output.push_str(&get_swara(&matches["vowel"], get_latin));
    }

    if matches["consonant_rarangken"].is_empty().not() {
        output.push_str(&get_rarangken(&matches["consonant_rarangken"], get_latin));
    }

    if matches["consonant_final"].is_empty().not() {
        output.push_str(&get_ngalagena(&matches["consonant_final"], get_latin));
        output.push_str(&get_rarangken("pamaeh", get_latin));
    }

    output
}

pub fn to_latin(groups: &[String], text: &Captures) -> String {
    let get_latin = true;

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
    if matches["angka"].is_empty().not() {
        return get_angka(&matches["angka"], get_latin);
    }

    // Transliterate punctuations
    if matches["not_sunda"].is_empty().not() {
        return matches["not_sunda"].to_owned();
    }

    // Transliterate anything else
    let mut output = String::new();

    if matches["ngalagena"].is_empty().not() {
        output.push_str(&get_ngalagena(&matches["ngalagena"], get_latin));
        if matches["rarangken_vowel"] == CHAR_RARANGKEN["pamaeh"] {
            return output;
        }
        if matches["rarangken_sonorant"].is_empty().not() {
            output.push_str(&get_rarangken_sonorant(
                &matches["rarangken_sonorant"],
                get_latin,
            ));
        }
        if matches["rarangken_vowel"].is_empty().not() {
            output.push_str(&get_rarangken(&matches["rarangken_vowel"], get_latin));
        } else {
            output.push('a');
        }
    } else {
        output.push_str(&get_swara(&matches["swara"], get_latin));
    }

    if matches["rarangken_final"].is_empty().not() {
        output.push_str(&get_rarangken(&matches["rarangken_final"], get_latin));
    }

    output
}
