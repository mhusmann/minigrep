extern crate minigrep;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    // println!("Searching for {}", config.query);
    // println!("In file: {}\n", config.filename);
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
