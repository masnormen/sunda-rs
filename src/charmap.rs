use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

type CharMap = HashMap<&'static str, &'static str>;

trait FindMatch {
    fn find_match(&self, character: &str, get_key: bool) -> Option<String>;
}

impl FindMatch for CharMap {
    fn find_match(&self, character: &str, get_key: bool) -> Option<String> {
        self.par_iter().find_map_first(|(&key, &val)| {
            if get_key && val == character {
                Some(key.to_string())
            } else if !get_key && key == character {
                Some(val.to_string())
            } else {
                Some("".to_string())
            }
        })
    }
}

lazy_static! {
    static ref CHAR_SWARA: CharMap = HashMap::from([
        ("a", "ᮃ"),
        ("i", "ᮄ"),
        ("u", "ᮅ"),
        ("e", "ᮈ"),
        ("é", "ᮆ"),
        ("eu", "ᮉ"),
        ("o", "ᮇ"),
    ]);
    static ref CHAR_NGALAGENA: CharMap = HashMap::from([
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
    static ref CHAR_RARANGKEN: CharMap = HashMap::from([
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
    static ref CHAR_RARANGKEN_SONORANT: CharMap =
        HashMap::from([("l", "ᮣ"), ("r", "ᮢ"), ("y", "ᮡ"),]);
    static ref CHAR_ANGKA: CharMap = HashMap::from([
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

/// Get the corresponding Swara character from a given Latin/Sundanese input
pub fn get_swara(character: &str, get_latin: bool) -> String {
    CHAR_SWARA.find_match(character, get_latin).unwrap()
}

/// Get the corresponding Ngalagena character from a given Latin/Sundanese input.
pub fn get_ngalagena(character: &str, get_latin: bool) -> String {
    CHAR_NGALAGENA.find_match(character, get_latin).unwrap()
}

/// Get the corresponding Rarangken character from a given Latin/Sundanese input.
pub fn get_rarangken(character: &str, get_latin: bool) -> String {
    CHAR_RARANGKEN.find_match(character, get_latin).unwrap()
}

/// Get the corresponding Rarangken sonorant character from a given Latin/Sundanese input.
pub fn get_rarangken_sonorant(character: &str, get_latin: bool) -> String {
    CHAR_RARANGKEN_SONORANT
        .find_match(character, get_latin)
        .unwrap()
}

/// Get the corresponding Angka character from a given Latin/Sundanese input.
pub fn get_angka(character: &str, get_latin: bool) -> String {
    CHAR_ANGKA.find_match(character, get_latin).unwrap()
}
