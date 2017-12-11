extern crate clap;
extern crate glob;
extern crate ansi_term;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use clap::{Arg, App};

use ansi_term::Color::Yellow;

fn get_commandline() -> (bool, String, String) {
    let matches = App::new("minigrep")
        .version("0.0.1")
        .author("Michael Husmann <michaelhusmann@gmail.com>")
        .about(
            "My minigrep tool. This is a bit extended to the version
which is shown in the Rust book.",
        )
        .arg(
            Arg::with_name("search-pattern")
                .help("The search pattern you are looking for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file")
                .help("The file to investigate. Wildcards are allowed!")
                .required(true)
                .index(2),
        )
        .arg(Arg::with_name("i").short("i").long("ignore").help(
            "Search ignore case",
        ))
        .get_matches();
    let ignorecase = !matches.is_present("i");
    let search = matches.value_of("search-pattern").unwrap();
    let filename = matches.value_of("file").unwrap();

    (ignorecase, search.to_string(), filename.to_string())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let (case_sensitive, query, filename) = get_commandline();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<Error>> {
    for path in glob::glob(&config.filename).expect("Failed to read file pattern") {
        match path {
            Ok(path) => {
                println!(
                    "{}",
                    Yellow.paint(PathBuf::from(&path).into_os_string().into_string().unwrap())
                );
                let mut f: File = File::open(&path)?;
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;
                let results = if config.case_sensitive {
                    search(&config.query, &contents)
                } else {
                    search_case_insensitive(&config.query, &contents)
                };
                for line in results {
                    println!("{:?}", line);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        };
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
