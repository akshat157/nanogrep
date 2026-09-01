use std::{env, error::Error, fs, process};

use nanogrep::{search, search_case_insensitive};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem occured while parsing arguments: {err}");
        process::exit(1);
    });

    // Don't need unwrap_or_else() here since we're only concerned
    // with one of the cases of Result<T, E>, i.e. the Error case here.
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &file_text) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &file_text) {
            println!("{line}");
        }
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Ignore the first value
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query string not provided!"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("file path not provided!"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
