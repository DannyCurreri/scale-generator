use std::env;
use std::process;
use scale_generator::*;
use error::Error;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    match Scale::new(&config.tonic, config.mode) {
        Ok(s) => println!("{} scale: {:?}", s.name(), s.enumerate()),
        Err(Error::InvalidTonic(e)) => eprintln!("Error: {}", e),
        Err(Error::InvalidIntervals(e)) => eprintln!("Error: {}", e),
    }
}

