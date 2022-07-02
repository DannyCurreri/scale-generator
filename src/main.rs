use scale_generator::Scale;
use std::env;

fn main() {
    match Scale::from_args(env::args()) {
        Ok(s) => println!("{} scale: {:?}", s.name(), s.enumerate()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
