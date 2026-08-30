use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for `{}`", config.query);
    println!("In the file `{}`", config.file_path);

    let file_text =
        fs::read_to_string(config.file_path).expect("Should've been able to read the file.");

    println!("Text read from file:\n\n{file_text}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // Temporary workaround for avoiding the borrow checker.
        // TODO: Optimize later.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
