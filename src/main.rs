mod capturer;
mod charmap;
mod transliterator;

use clap::{Arg, Command};
use rayon::prelude::*;
use std::env;
use std::time::Instant;

use crate::{
    capturer::{capture_latin, capture_sunda},
    charmap::get_angka,
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
