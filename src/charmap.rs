use std::collections::HashMap;

fn find_key_or_value(map: &HashMap<&str, &str>, input: &str, get_key: bool) -> String {
    let result = map.iter().find_map(|(&key, &val)| {
        if get_key && val == input {
            return Some(key.to_string());
        } else if !get_key && key == input {
            return Some(val.to_string());
        } else {
            return None;
        }
    });

    result.unwrap_or("".to_string())
}

pub fn get_swara(character: &str, get_latin: bool) -> String {
    let char_list = HashMap::from([
        ("a", "ᮃ"),
        ("i", "ᮄ"),
        ("u", "ᮅ"),
        ("e", "ᮈ"),
        ("é", "ᮆ"),
        ("eu", "ᮉ"),
        ("o", "ᮇ"),
    ]);

    find_key_or_value(&char_list, character, get_latin)
}

pub fn get_ngalagena(character: &str, get_latin: bool) -> String {
    let char_list = HashMap::from([
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

    find_key_or_value(&char_list, character, get_latin)
}

pub fn get_rarangken(character: &str, get_latin: bool) -> String {
    let char_list = HashMap::from([
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

    find_key_or_value(&char_list, character, get_latin)
}

pub fn get_rarangken_sonorant(character: &str, get_latin: bool) -> String {
    let char_list = HashMap::from([("l", "ᮣ"), ("r", "ᮢ"), ("y", "ᮡ")]);

    find_key_or_value(&char_list, character, get_latin)
}

pub fn get_angka(character: &str, get_latin: bool) -> String {
    let char_list = HashMap::from([
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

    find_key_or_value(&char_list, character, get_latin)
}
