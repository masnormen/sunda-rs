use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_SWARA: HashMap<&'static str, &'static str> = HashMap::from([
        ("a", "ᮃ"),
        ("i", "ᮄ"),
        ("u", "ᮅ"),
        ("e", "ᮈ"),
        ("é", "ᮆ"),
        ("eu", "ᮉ"),
        ("o", "ᮇ"),
    ]);
    static ref CHAR_NGALAGENA: HashMap<&'static str, &'static str> = HashMap::from([
        ("k", "ᮊ"),
        ("g", "ᮌ"),
        ("ng", "ᮍ"),
        ("c", "ᮎ"),
        ("j", "ᮏ"),
        ("ny", "ᮑ"),
        ("t", "ᮒ"),
        ("d", "ᮓ"),
        ("n", "ᮔ"),
        ("p", "ᮕ"),
        ("b", "ᮘ"),
        ("m", "ᮙ"),
        ("y", "ᮚ"),
        ("r", "ᮛ"),
        ("l", "ᮜ"),
        ("w", "ᮝ"),
        ("s", "ᮞ"),
        ("h", "ᮠ"),
        ("f", "ᮖ"),
        ("q", "ᮋ"),
        ("v", "ᮗ"),
        ("x", "ᮟ"),
        ("z", "ᮐ"),
        ("kh", "ᮮ"),
        ("sy", "ᮯ"),
    ]);
    static ref CHAR_RARANGKEN: HashMap<&'static str, &'static str> = HashMap::from([
        ("a", ""),
        ("i", "ᮤ"),
        ("u", "ᮥ"),
        ("e", "ᮨ"),
        ("é", "ᮦ"),
        ("eu", "ᮩ"),
        ("o", "ᮧ"),
        ("r", "ᮁ"),
        ("ng", "ᮀ"),
        ("h", "ᮂ"),
        ("pamaeh", "᮪"),
    ]);
    static ref CHAR_RARANGKEN_SONORANT: HashMap<&'static str, &'static str> =
        HashMap::from([("l", "ᮣ"), ("r", "ᮢ"), ("y", "ᮡ"),]);
    static ref CHAR_ANGKA: HashMap<&'static str, &'static str> = HashMap::from([
        ("0", "᮰"),
        ("1", "᮱"),
        ("2", "᮲"),
        ("3", "᮳"),
        ("4", "᮴"),
        ("5", "᮵"),
        ("6", "᮶"),
        ("7", "᮷"),
        ("8", "᮸"),
        ("9", "᮹"),
    ]);
}

pub fn get_swara(character: &str, get_latin: bool) -> String {
    let result = CHAR_SWARA.par_iter().find_map_first(|(&key, &val)| {
        if get_latin && val == character {
            Some(key.to_string())
        } else if !get_latin && key == character {
            Some(val.to_string())
        } else {
            None
        }
    });

    result.unwrap_or_else(|| "".to_string())
}

pub fn get_ngalagena(character: &str, get_latin: bool) -> String {
    let result = CHAR_NGALAGENA.par_iter().find_map_first(|(&key, &val)| {
        if get_latin && val == character {
            Some(key.to_string())
        } else if !get_latin && key == character {
            Some(val.to_string())
        } else {
            None
        }
    });

    result.unwrap_or_else(|| "".to_string())
}

pub fn get_rarangken(character: &str, get_latin: bool) -> String {
    let result = CHAR_RARANGKEN.par_iter().find_map_first(|(&key, &val)| {
        if get_latin && val == character {
            Some(key.to_string())
        } else if !get_latin && key == character {
            Some(val.to_string())
        } else {
            None
        }
    });

    result.unwrap_or_else(|| "".to_string())
}

pub fn get_rarangken_sonorant(character: &str, get_latin: bool) -> String {
    let result = CHAR_RARANGKEN_SONORANT
        .par_iter()
        .find_map_first(|(&key, &val)| {
            if get_latin && val == character {
                Some(key.to_string())
            } else if !get_latin && key == character {
                Some(val.to_string())
            } else {
                None
            }
        });

    result.unwrap_or_else(|| "".to_string())
}

pub fn get_angka(character: &str, get_latin: bool) -> String {
    let result = CHAR_ANGKA.par_iter().find_map_first(|(&key, &val)| {
        if get_latin && val == character {
            Some(key.to_string())
        } else if !get_latin && key == character {
            Some(val.to_string())
        } else {
            None
        }
    });

    result.unwrap_or_else(|| "".to_string())
}
