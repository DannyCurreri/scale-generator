use std::env;
pub mod error;
use error::Error;

pub struct Config {
    pub tonic: String,
    pub mode: Mode,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, String> {
        let mut args = args;
        args.next();

        let tonic = match args.next() {
            Some(s) => s,
            None => return Err("No tonic note provided".to_string()),
        };

        let mode = match args.next() {
            None => {
                if tonic.chars().next().unwrap().is_uppercase() {
                    Mode::Ionian
                } else {
                    Mode::Aeolian
                }
            },
            Some(arg) => {
                match arg.to_lowercase().as_str() {
                    "major" => Mode::Ionian,
                    "maj" => Mode::Ionian,
                    "ionian" => Mode::Ionian,
                    "minor" => Mode::Aeolian,
                    "min" => Mode::Aeolian,
                    "aeolian" => Mode::Aeolian,
                    "chromatic" => Mode::Chromatic,
                    other => return Err(format!("Invalid argument: {}", other)),
                }
            },
        };
        Ok(Config{ tonic, mode })
    }
}

pub struct Scale<'a> {
    tonic: &'a str,
    mode: Mode,
    signature: Signature,
}

#[derive(PartialEq, Debug)]
enum Signature {
    Natural,
    Sharp,
    Flat, 
}

impl Signature {
    fn chromatic_note_range(&self) -> [&str; 12] {
        match *self {
            Signature::Natural => 
                ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"],
            Signature::Sharp => 
                ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"],
            Signature::Flat =>
                ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    Ionian,
    Aeolian,
    Chromatic,
}

impl Mode {
    fn intervals(&self) -> Vec<usize> {
        match *self {
            Mode::Ionian => vec![2,2,1,2,2,2,1],
            Mode::Aeolian => vec![2,1,2,2,1,2,2],
            Mode::Chromatic => vec![1,1,1,1,1,1,1,1,1,1,1,1],
        }
    }
    fn name(&self) -> &'static str {
        match *self {
            Mode::Ionian => "major",
            Mode::Aeolian => "minor",
            Mode::Chromatic => "chromatic",
        }
    }
}

impl<'a> Scale<'a> {
    pub fn new(tonic: &'a str, mode: Mode) -> Result<Scale<'a>, Error<'a>> {
        let signature = match Scale::signature(tonic) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(Scale { 
            tonic: tonic,
            mode: mode,
            signature: signature,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = vec![];
        let note_bank = self.signature.chromatic_note_range();
        // convert tonic to a Vec to capitalize first letter, 
        // then convert back to str
        let mut v: Vec<char> = self.tonic.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s: String = v.into_iter().collect();
        let tonic = &s;

        let mut position = note_bank
            .iter()
            .position(|&x| x == tonic)
            .unwrap();

        notes.push(note_bank[position].to_string());

        //let intervals = self.mode.intervals();
        for step in self.mode.intervals() {
            position += step;
            notes.push(note_bank[position % 12].to_string());
        }
        notes
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        name.push_str(self.tonic);
        name.push_str(" ");
        name.push_str(self.mode.name());
        name
    }

    fn signature(tonic: &'a str) -> Result<Signature, Error> {
        if ["C", "a"].contains(&tonic) {
            return Ok(Signature::Natural);
        }
        if ["G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#"]
            .contains(&tonic) {
                return Ok(Signature::Sharp);
            }
        if ["F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb"]
            .contains(&tonic) {
                return Ok(Signature::Flat);
            }
        else {
            return Err(Error::InvalidTonic(error::messages::INVALID_TONIC));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural() {
        assert_eq!(Scale::signature("C").unwrap(), Signature::Natural);
    }

    #[test]
    fn test_sharp() {
        assert_eq!(Scale::signature("b").unwrap(), Signature::Sharp);
        assert_eq!(Scale::signature("F#").unwrap(), Signature::Sharp);
    }

    #[test]
    fn test_flat() {
        assert_eq!(Scale::signature("F").unwrap(), Signature::Flat);
        assert_eq!(Scale::signature("eb").unwrap(), Signature::Flat);
    }

    #[test]
    #[should_panic]
    fn test_err() {
        Scale::signature("x").unwrap();
    }
}
