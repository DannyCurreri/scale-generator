use scale_generator::Scale;
use std::env;

fn main() {
    match Scale::from_args(env::args()) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("Error: {}", e),
    }
}
