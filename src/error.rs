#[derive(Debug)]
pub enum Error<'a> {
    InvalidTonic(&'a str),
    InvalidIntervals(&'a str),
}

pub mod messages {
    pub const INVALID_TONIC: &str ="Invalid tonic. Use letter A-G for maj or a-g for min, optionally followed # or b for accidental.";
    pub const INVALID_INTERVAL: &str ="Invalid interval input string. Use M for whole step, m for half step, or A for augmented second.";
}

