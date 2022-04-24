use std::env;
pub mod error;
pub mod notes;
use notes::Note;

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
                    other => return Err(format!("Invalid argument: {}", other)),
                }
            },
        };
        Ok(Config{ tonic, mode })
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
}

impl Mode {
    fn intervals(&self) -> [i32; 7] {
        match *self {
            Mode::Ionian     => [2,2,1,2,2,2,1],
            Mode::Dorian     => [2,1,2,2,2,1,2],
            Mode::Phrygian   => [1,2,2,2,1,2,2],
            Mode::Lydian     => [2,2,2,1,2,2,1],
            Mode::Mixolydian => [2,2,1,2,2,1,2],
            Mode::Aeolian    => [2,1,2,2,1,2,2],
            Mode::Locrian    => [1,2,2,1,2,2,2],
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Mode::Ionian     => "major",
            Mode::Dorian     => "dorian",
            Mode::Phrygian   => "phrygian",
            Mode::Lydian     => "lydian",
            Mode::Mixolydian => "mixolydian",
            Mode::Aeolian    => "minor",
            Mode::Locrian    => "locrian",
        }
    }
}

pub struct Scale {
    tonic: Note,
    mode: Mode,
}

impl Scale {
    pub fn new(tonic: Note, mode: Mode) -> Result<Scale, &'static str> {
        Ok(Scale { 
            tonic,
            mode,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = vec![];
        let mut current_note = self.tonic;

        for step in self.mode.intervals() {
            notes.push(current_note.as_string());
            current_note = current_note.next(step);
        }
        notes
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.tonic.as_string(), self.mode.name())
    }
}

