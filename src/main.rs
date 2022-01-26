mod charmap;
mod capturer;
mod transliterator;

use core::env;

fn main() {
    let mut get_latin: bool = false;
    let mut input = String::new();

    let arguments: Vec<String> = env::args().collect();

    // Check length of arguments
    if arguments.len() <= 1 {
        print_help(1);
    }

    // Set transliteration option
    match arguments[1].as_str() {
        "--latin" | "-l" => {
            get_latin = true;
        }
        "--sunda" | "-s" => {
            get_latin = false;
        }
        "--help" | "-h" => {
            print_help(0);
        }
        _ => {
            println!("Unrecognized command. Please refer to usage below.");
            print_help(1);
        }
    }

    // Process the rest of arguments as one input string
    input = arguments[2..].join(" ");
    input = input.trim().to_lowercase();

    let matches = capturer::capture_latin(&input);

    let mut output = String::new();

    for syllable in matches {
        let out = transliterator::to_sundanese(&syllable);
        output.push_str(out.as_str());
    }

    println!("{}", output);

    // if to_latin {
    //     println!("HEHE");
    // } else {

    // }

    // match to_latin {
    //     true => {
    //         let latin_output = capturer::capture_sunda(&input);
    //         println!("{:?}", latin_output);
    //     }
    //     false => {
    //         let sunda_output = capturer::capture_latin(&input);
    //         println!("{:?}", sunda_output);
    //     }
    // }
}

fn print_help(exit_code: i32) {
    println!("Usage: trans [options] [string]");
    println!("");
    println!("Options:");
    println!("  --help, -h        Print this help message");
    println!("  --sunda, -s       Transliterate into Sundanese script");
    println!("  --latin, -l       Transliterate into Latin script");

    std::process::exit(exit_code);
}
