use std::env;
use scale_generator::Scale;

fn main() {
    match Scale::from_args(env::args()) {
        Ok(s) => println!("{} scale: {:?}", s.name(), s.enumerate()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

