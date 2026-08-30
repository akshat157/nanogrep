use std::{env, error::Error, fs, process};

use nanogrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem occured while parsing arguments: {err}");
        process::exit(1);
    });

    // Don't need unwrap_or_else() here since we're only concerned
    // with one of the cases of Result<T, E>, i.e. the Error case here.
    if let Err(e) = run(config) {
        println!("App error: {e}");
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &file_text);

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Temporary workaround for avoiding the borrow checker.
        // TODO: Optimize later.
        if args.len() < 3 {
            return Err("Not enough arguments provided!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
