use clap::{Arg, Command};
use rayon::prelude::*;
use std::env;
use std::time::Instant;

mod capturer;
mod charmap;
mod transliterator;

fn main() {
    // Benchmark start
    let now = Instant::now();

    let arguments = Command::new("sunda")
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

    let is_into_sunda = arguments.is_present("sunda");
    let input = match arguments.values_of("input") {
        Some(input) => input.collect::<Vec<&str>>().join(" ").to_lowercase(),
        None => String::new(),
    };

    let (groups, matches) = match is_into_sunda {
        true => capturer::capture_latin(&input),
        false => capturer::capture_sunda(&input),
    };

    let output: String = match is_into_sunda {
        true => matches
            .par_iter()
            .map(|capture| transliterator::to_sundanese(&groups, capture))
            .collect::<Vec<String>>()
            .join(""),
        false => matches
            .par_iter()
            //TODO: change this
            .map(|capture| transliterator::to_sundanese(&groups, capture))
            .collect::<Vec<String>>()
            .join(""),
    };

    println!("{}", output);

    // Benchmark end
    println!("Elapsed: {:.2?}", now.elapsed());
}
