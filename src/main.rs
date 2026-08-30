use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem occured while parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for `{}`", config.query);
    println!("In the file `{}`", config.file_path);

    run(config);
}

fn run(config: Config) {
    let file_text =
        fs::read_to_string(config.file_path).expect("Should've been able to read the file.");

    println!("Text read from file:\n\n{file_text}");
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
