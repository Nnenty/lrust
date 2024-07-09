use std::env;
use std::process;

use config::{self, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}\nTo use program enter:");
        eprintln!("cargo run 'word to find in file' 'file'");

        process::exit(1);
    });

    if !config.ignore_case {
        println!(">>To search word with case insensitive use 'IGNORE_CASE=1'<<\n");
    }

    println!("Searching {} in {}..", config.query, config.file_path);

    if let Err(e) = config::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}
