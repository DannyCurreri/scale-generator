use scale_generator::Scale;
use std::env;

fn main() {
    match Scale::from_args(env::args()) {
        Ok(s) => {
            let notes: String = s.enumerate().iter().map(|note| format!("{} ", note)).collect();
            println!("{} scale: {}", s.name(), notes);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
