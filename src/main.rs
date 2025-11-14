use std::error::Error;
use std::fs;
use std::{env, process};

use minigrep::Config;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let config = Config::new(&arg).unwrap_or_else(|err| {
        eprintln!("Error parsing the arguments {}", err);
        process::exit(1);
    });

    //we do not use unwrap because we do not require the inner value
    if let Err(e) = run(&config) {
        eprintln!("Error running the main logic {}", e);
        process::exit(1);
    };
}
fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.get_filepath())?;

    let arr = if *config.get_option() {
        minigrep::search(&content, config.get_string())
    } else {
            minigrep::search_case_insensitive(&content, config.get_string())
    };

    for line in arr {
        println!("{} ", line);
    }
    Ok(())
}
