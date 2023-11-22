use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // TODO: why unwrap_or_else need a Config for return, 
        // but we can use exit to suppress the error?
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application run error: {}", e);

        process::exit(1);
    }
}