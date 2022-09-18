use std::env;
use std::fmt;
use std::str::FromStr;

pub mod notes;
use notes::Note;

#[derive(Copy, Clone)]
pub enum Mode {Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian, Locrian}

impl Mode {
    fn intervals(&self) -> [i32; 7] {
        match *self {
            Mode::Ionian     => [2, 2, 1, 2, 2, 2, 1],
            Mode::Dorian     => [2, 1, 2, 2, 2, 1, 2],
            Mode::Phrygian   => [1, 2, 2, 2, 1, 2, 2],
            Mode::Lydian     => [2, 2, 2, 1, 2, 2, 1],
            Mode::Mixolydian => [2, 2, 1, 2, 2, 1, 2],
            Mode::Aeolian    => [2, 1, 2, 2, 1, 2, 2],
            Mode::Locrian    => [1, 2, 2, 1, 2, 2, 2],
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Mode::Ionian     => "Major",
            Mode::Dorian     => "Dorian",
            Mode::Phrygian   => "Phrygian",
            Mode::Lydian     => "Lydian",
            Mode::Mixolydian => "Mixolydian",
            Mode::Aeolian    => "Minor",
            Mode::Locrian    => "Locrian",
        };
        write!(f, "{}", name)
    }
}

pub struct Scale {
    tonic: Note,
    mode: Mode,
}

impl Scale {
    pub fn new(tonic: Note, mode: Mode) -> Self {
        Scale { tonic, mode }
    }

    pub fn from_args(args: env::Args) -> Result<Self, String> {
        let mut args = args;
        args.next();

        let tonic_str = match args.next() {
            Some(s) => s,
            None => return Err("No tonic note provided.".to_string()),
        };

        let tonic = match Note::from_str(&tonic_str) {
            Ok(note) => note,
            Err(e) => return Err(e.to_string()),
        };

        let mode = match args.next() {
            None => {
                if tonic_str.chars().next().unwrap().is_uppercase() {
                    Mode::Ionian
                } else {
                    Mode::Aeolian
                }
            }
            Some(arg) => match arg.to_lowercase().as_str() {
                "maj"        => Mode::Ionian,
                "major"      => Mode::Ionian,
                "ionian"     => Mode::Ionian,
                "dorian"     => Mode::Dorian,
                "phrygian"   => Mode::Phrygian,
                "lydian"     => Mode::Lydian,
                "mixolydian" => Mode::Mixolydian,
                "min"        => Mode::Aeolian,
                "minor"      => Mode::Aeolian,
                "aeolian"    => Mode::Aeolian,
                "locrian"    => Mode::Locrian,
                other => return Err(format!("Invalid argument: {}", other)),
            },
        };
        Ok(Scale::new(tonic, mode))
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = vec![];
        let mut current_note = self.tonic;

        for step in self.mode.intervals() {
            notes.push(format!("{}", current_note));
            current_note = current_note.next(step);
        }
        notes
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notes: String = self
            .enumerate()
            .iter()
            .map(|note| format!("{} ", note))
            .collect();
        write!(f, "{} {} scale: {}", self.tonic, self.mode, notes)
    }
}
