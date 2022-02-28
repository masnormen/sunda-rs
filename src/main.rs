mod capturer;
mod charmap;
mod transliterator;

use clap::{Arg, Command};
use rayon::prelude::*;
use std::env;
use std::time::Instant;

use crate::{
    capturer::{capture_latin, capture_sunda},
    transliterator::to_sundanese,
};

fn main() {
    // Benchmark start
    let now = Instant::now();

    let args = Command::new("sunda")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nourman Hajar <nourmanhajar@gmail.com>")
        .about(
            "Convert/transliterate Latin into Sundanese script (Aksara Sunda Baku), or vice versa",
        )
        .arg(
            Arg::new("sunda")
                .short('s')
                .long("sunda")
                .required_unless_present("latin")
                .overrides_with("latin")
                .help("Turns on transliteration into Sundanese script"),
        )
        .arg(
            Arg::new("latin")
                .short('l')
                .long("latin")
                .required_unless_present("sunda")
                .overrides_with("sunda")
                .help("Turns on transliteration into Latin script"),
        )
        .arg(
            Arg::new("input")
                .raw(true)
                .help("Input string to be transliterated"),
        )
        .get_matches();

    let into_sunda = args.is_present("sunda");
    let input = match args.values_of("input") {
        Some(input) => input.collect::<Vec<&str>>().join(" ").to_lowercase(),
        None => String::new(),
    };

    let output = if into_sunda {
        get_sundanese(&input)
    } else {
        get_latin(&input)
    };

    println!("{}", output);

    // Benchmark end
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn get_sundanese(input: &str) -> String {
    let lowercased = input.to_lowercase();
    let (groups, matches) = capture_latin(&lowercased);
    matches
        .par_iter()
        .map(|capture| to_sundanese(&groups, capture))
        .collect::<Vec<String>>()
        .join("")
}

fn get_latin(input: &str) -> String {
    let (groups, matches) = capture_latin(input);
    matches
        .par_iter()
        .map(|capture| to_sundanese(&groups, capture))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod latin_to_sunda_tests {
    use super::*;

    #[test]
    fn convert_basic_letters() {
        let result = get_sundanese(
            "ka ga nga ca ja nya ta da na pa ba ma ya ra la wa sa ha fa qa va xa za kha",
        );
        assert_eq!(result, "ᮊ ᮌ ᮍ ᮎ ᮏ ᮑ ᮒ ᮓ ᮔ ᮕ ᮘ ᮙ ᮚ ᮛ ᮜ ᮝ ᮞ ᮠ ᮖ ᮋ ᮗ ᮟ ᮐ ᮮ");
    }

    #[test]
    fn convert_vowel_rarangken() {
        let result = get_sundanese("pa pi pu pe pé peu po p");
        assert_eq!(result, "ᮕ ᮕᮤ ᮕᮥ ᮕᮨ ᮕᮦ ᮕᮩ ᮕᮧ ᮕ᮪");
    }

    #[test]
    fn convert_consonant_rarangken() {
        let result = get_sundanese("di klatén, ada santri kyai tebang pohon buah pir");
        assert_eq!(result, "ᮓᮤ ᮊᮣᮒᮦᮔ᮪, ᮃᮓ ᮞᮔ᮪ᮒᮢᮤ ᮊᮡᮄ ᮒᮨᮘᮀ ᮕᮧᮠᮧᮔ᮪ ᮘᮥᮃᮂ ᮕᮤᮁ");
    }

    #[test]
    fn convert_numbers_and_add_pipe() {
        let result = get_sundanese("tanggal 17 bulan 8 taun 1945");
        assert_eq!(result, "ᮒᮀᮌᮜ᮪ |᮱᮷| ᮘᮥᮜᮔ᮪ |᮸| ᮒᮅᮔ᮪ |᮱᮹᮴᮵|");
    }
}

#[cfg(test)]
mod sunda_to_latin_tests {
    use super::*;

    #[test]
    fn convert_basic_letters() {
        let result = get_latin("ᮊ ᮌ ᮍ ᮎ ᮏ ᮑ ᮒ ᮓ ᮔ ᮕ ᮘ ᮙ ᮚ ᮛ ᮜ ᮝ ᮞ ᮠ ᮖ ᮋ ᮗ ᮟ ᮐ ᮮ");
        assert_eq!(
            result,
            "ka ga nga ca ja nya ta da na pa ba ma ya ra la wa sa ha fa qa va xa za kha"
        );
    }

    #[test]
    fn convert_vowel_rarangken() {
        let result = get_latin("ᮕ ᮕᮤ ᮕᮥ ᮕᮨ ᮕᮦ ᮕᮩ ᮕᮧ ᮕ᮪");
        assert_eq!(result, "pa pi pu pe pé peu po p");
    }

    #[test]
    fn convert_consonant_rarangken() {
        let result = get_latin("ᮓᮤ ᮊᮣᮒᮦᮔ᮪, ᮃᮓ ᮞᮔ᮪ᮒᮢᮤ ᮊᮡᮄ ᮒᮨᮘᮀ ᮕᮧᮠᮧᮔ᮪ ᮘᮥᮃᮂ ᮕᮤᮁ");
        assert_eq!(result, "di klatén, ada santri kyai tebang pohon buah pir");
    }

    #[test]
    fn convert_numbers_and_remove_pipe() {
        let result = get_latin("ᮒᮀᮌᮜ᮪ |᮱᮷| ᮘᮥᮜᮔ᮪ |᮸| ᮒᮅᮔ᮪ |᮱᮹᮴᮵|");
        assert_eq!(result, "tanggal 17 bulan 8 taun 1945");
    }
}
