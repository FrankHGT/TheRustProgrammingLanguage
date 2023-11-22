use std::{env, fs, process, error::Error};
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

    if let Err(e) = run(config) {
        println!("Application run error: {}", e);

        process::exit(1);
    }
}

// TODO: meaning of Box<dyn Error>
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}