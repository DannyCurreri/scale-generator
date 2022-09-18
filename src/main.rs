use scale_generator::Scale;
use std::env;

fn main() {
    match Scale::from_args(env::args()) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("\n{}", USAGE);
        }
    }
}

const USAGE: &str = "usage: scg tonic_note [mode]

tonic_note: <note letter>['#' | 'b']

mode args:
    major/maj,
    minor/min,
    ionian,
    dorian,
    phrygian,
    lydian,
    mixolydian,
    aeolian,
    locrian

If mode is not provided, an upper case tonic note
defaults to major and lower case defaults to minor.";
