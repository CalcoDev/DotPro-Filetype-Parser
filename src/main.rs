use std::process;

use pro_parser::{run, parse_arguments};

fn main() {
    let path = parse_arguments(std::env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(&path) {
        println!("{}", err);
        process::exit(1);
    };
}