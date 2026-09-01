use std::{env, process};

use nanogrep::{Config, run};

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
