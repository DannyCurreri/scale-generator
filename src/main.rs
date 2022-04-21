use std::env;
use std::process;
use scale_generator::*;
use error::Error;

fn main() {
    let mut args = env::args();
    args.next();

    let tonic = args.next().unwrap_or_else(|| {
        eprintln!("No tonic note provided");
        process::exit(1);
    });

    let scale = if tonic.chars().next().unwrap().is_uppercase() {
        Scale::new(&tonic, Mode::Ionian)
    } else {
        Scale::new(&tonic, Mode::Aeolian)
    };

    match scale {
        Ok(s) => println!("{} scale: {:?}", s.name(), s.enumerate()),
        Err(Error::InvalidTonic(e)) => eprintln!("Error: {}", e),
        Err(Error::InvalidIntervals(e)) => eprintln!("Error: {}", e),
    }
}

