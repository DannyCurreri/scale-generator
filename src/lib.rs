use std::env;
pub mod error;
mod notes;

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
                    "maj" => Mode::Ionian,
                    "major" => Mode::Ionian,
                    "ionian" => Mode::Ionian,
                    "dorian" => Mode::Dorian,
                    "phrygian" => Mode::Phrygian,
                    "lydian" => Mode::Lydian,
                    "mixolydian" => Mode::Mixolydian,
                    "min" => Mode::Aeolian,
                    "minor" => Mode::Aeolian,
                    "aeolian" => Mode::Aeolian,
                    "locrian" => Mode::Locrian,
                    "chromatic" => Mode::Chromatic,
                    other => return Err(format!("Invalid argument: {}", other)),
                }
            },
        };
        Ok(Config{ tonic, mode })
    }
}

#[derive(PartialEq, Debug)]
enum Signature {
    Sharp,
    Flat, 
    AllSharp,
    AllFlat,
}

impl Signature {
    fn chromatic_note_range(&self) -> [&str; 12] {
        match *self {
            Signature::Sharp => 
                ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"],
            Signature::AllSharp => // For key of C# and associated modes
                ["A", "A#", "B", "B#", "C#", "D", "D#", "E", "E#", "F#", "G", "G#"],
            Signature::Flat => 
                ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"],
            Signature::AllFlat => // For key of Cb and associated modes
                ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Chromatic,
}

enum Family {
    Major,
    Minor,
    Chromatic,
}

impl Mode {
    fn intervals(&self) -> Vec<usize> {
        match *self {
            Mode::Ionian => vec![2,2,1,2,2,2,1],
            Mode::Dorian => vec![2,1,2,2,2,1,2],
            Mode::Phrygian => vec![1,2,2,2,1,2,2],
            Mode::Lydian => vec![2,2,2,1,2,2,1],
            Mode::Mixolydian => vec![2,2,1,2,2,1,2],
            Mode::Aeolian => vec![2,1,2,2,1,2,2],
            Mode::Locrian => vec![1,2,2,1,2,2,2],
            Mode::Chromatic => vec![1,1,1,1,1,1,1,1,1,1,1,1],
        }
    }
    fn family(&self) -> Family {
        match *self {
            Mode::Ionian => Family::Major,
            Mode::Dorian => Family::Minor,
            Mode::Phrygian => Family::Minor,
            Mode::Lydian => Family::Major,
            Mode::Mixolydian => Family::Major,
            Mode::Aeolian => Family::Minor,
            Mode::Locrian => Family::Minor,
            Mode::Chromatic => Family::Chromatic,
        }
    }
    fn name(&self) -> &'static str {
        match *self {
            Mode::Ionian => "major",
            Mode::Dorian => "dorian",
            Mode::Phrygian => "phrygian",
            Mode::Lydian => "lydian",
            Mode::Mixolydian => "mixolydian",
            Mode::Aeolian => "minor",
            Mode::Locrian => "locrian",
            Mode::Chromatic => "chromatic",
        }
    }
}

pub struct Scale {
    tonic: String,
    mode: Mode,
    signature: Signature,
}

impl Scale {
    pub fn new(tonic: &str, mode: Mode) -> Result<Scale, &'static str> {
        // convert tonic to a Vec to capitalize first letter, 
        // then convert back to str
        let mut v: Vec<char> = tonic.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s: String = v.into_iter().collect();
        let tonic = s.as_str();

        let signature = match Scale::signature(tonic, mode) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(Scale { 
            tonic: tonic.to_string(),
            mode: mode,
            signature: signature,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = vec![];
        let note_bank = self.signature.chromatic_note_range();

        let mut position = note_bank
            .iter()
            .position(|&x| x == self.tonic)
            .unwrap();

        notes.push(note_bank[position].to_string());

        for step in self.mode.intervals() {
            position += step;
            notes.push(note_bank[position % 12].to_string());
        }
        notes
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        name.push_str(&self.tonic);
        name.push_str(" ");
        name.push_str(self.mode.name());
        name
    }

    fn signature(tonic: &str, mode: Mode) -> Result<Signature, &'static str> {
        match mode.family() {
            Family::Major => {
                if ["C", "G", "D", "A", "E", "B", "F#"].contains(&tonic) {
                    return Ok(Signature::Sharp);
                } else if ["F", "Bb", "Eb", "Ab", "Db", "Gb"].contains(&tonic) {
                    return Ok(Signature::Flat);
                } else {
                    return Err("Invalid Tonic");
                }
            }
            Family::Minor => {
                if ["A", "E", "B", "F#", "C#", "G#", "D#"].contains(&tonic) {
                    return Ok(Signature::Sharp);
                } else if ["D", "G", "C", "F", "Bb", "Eb"].contains(&tonic) {
                    return Ok(Signature::Flat);
                } else if "A#" == tonic {
                    return Ok(Signature::AllSharp);
                } else {
                    return Err("Invalid Tonic");
                }
            }
            Family::Chromatic => {
                if ["C", "G", "D", "A", "E", "B", "F#"].contains(&tonic) {
                    return Ok(Signature::Sharp);
                } else if ["F", "Bb", "Eb", "Ab", "Db", "Gb"].contains(&tonic) {
                    return Ok(Signature::Flat);
                } else {
                    return Err("Invalid Tonic");
                }
            }
        }
    }
}
