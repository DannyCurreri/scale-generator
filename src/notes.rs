#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Note {
    letter: Letter,
    accidental_count: i32,
}

impl Note {
    pub fn new(letter: Letter, accidental_count: i32) -> Self {
        Note {
            letter,
            accidental_count,
        }
    }
    
    pub fn from_string(input: &str) -> Result<Self, &'static str> {
        let mut input = input.chars();
        let letter = match input.next() {
            Some(c) => match c.to_ascii_uppercase() {
                'A' => Letter::A,
                'B' => Letter::B,
                'C' => Letter::C,
                'D' => Letter::D,
                'E' => Letter::E,
                'F' => Letter::F,
                'G' => Letter::G,
                _ => return Err("Invalid tonic note"),
            },
            None => return Err("No tonic note provided."),
        };
        let accidental_count = match input.next() {
            None => 0,
            Some('#') => 1,
            Some('b') => -1,
            _ => return Err("Invalid tonic note"),
        };
        if let Some(_) = input.next() {
            return Err("Invalid tonic note");
        }
        Ok(Note::new(letter, accidental_count))
    }

    pub fn next(&self, step: i32) -> Self {
        let letter = self.letter.next();
        let accidental_count = 
            self.accidental_count + step - letter.incremental_value();

        Note {
            letter,
            accidental_count,
        }
    }
    
    pub fn as_string(&self) -> String {
        let mut accidental = String::new();
        if self.accidental_count > 0 {
            for _ in 0..self.accidental_count {
                accidental.push_str("#");
            }
        } else if self.accidental_count < 0 {
            for _ in 0..-self.accidental_count {
                accidental.push_str("b");
            }
        }
        format!("{}{}", self.letter.as_str(), accidental)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Letter {
    fn next(&self) -> Self {
        match self {
            Letter::A => Letter::B,
            Letter::B => Letter::C,
            Letter::C => Letter::D,
            Letter::D => Letter::E,
            Letter::E => Letter::F,
            Letter::F => Letter::G,
            Letter::G => Letter::A,
        }
    }

    fn incremental_value(&self) -> i32 {
        match self {
            Letter::A => 2,
            Letter::B => 2,
            Letter::C => 1,
            Letter::D => 2,
            Letter::E => 2,
            Letter::F => 1,
            Letter::G => 2,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Letter::A => "A",
            Letter::B => "B",
            Letter::C => "C",
            Letter::D => "D",
            Letter::E => "E",
            Letter::F => "F",
            Letter::G => "G",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_note() {
        let note = Letter::E;
        assert_eq!(note.next(), Letter::F);
    }

    #[test]
    fn test_incr_value() {
        assert_eq!(Letter::A.incremental_value(), 2);
        assert_eq!(Letter::C.incremental_value(), 1);
    }

    #[test]
    fn test_str() {
        assert_eq!(Letter::A.as_str(), "A");
    }

    #[test]
    fn test_sharp_note() {
        let note = Note { 
            letter: Letter::A, 
            accidental_count: 1,
        };
        assert_eq!(note.as_string(), "A#");
    }

    #[test]
    fn test_flat_note() {
        let note = Note { 
            letter: Letter::A, 
            accidental_count: -1, 
        };
        assert_eq!(note.as_string(), "Ab");
    }

    #[test]
    fn test_natural_note() {
        let note = Note { 
            letter: Letter::A, 
            accidental_count: 0, 
        };
        assert_eq!(note.as_string(), "A");
    }

    #[test]
    fn test_simple_whole_step() {
        let note = Note { 
            letter: Letter::A, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(2).as_string(), "B");
    }

    #[test]
    fn test_half_step() {
        let note = Note { 
            letter: Letter::A, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(1).as_string(), "Bb");
    }

    #[test]
    fn test_bsharp_whole_step() {
        let note = Note { 
            letter: Letter::B, 
            accidental_count: 1, 
        };
        assert_eq!(note.next(2).as_string(), "C##");
    }

    #[test]
    fn test_bsharp_half_step() {
        let note = Note { 
            letter: Letter::B, 
            accidental_count: 1, 
        };
        assert_eq!(note.next(1).as_string(), "C#");
    }

    #[test]
    fn test_e_half_step() {
        let note = Note { 
            letter: Letter::E, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(1).as_string(), "F");
    }

    #[test]
    fn test_e_whole_step() {
        let note = Note { 
            letter: Letter::E, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(2).as_string(), "F#");
    }

    #[test]
    fn test_from_empty_string() {
        if let Err(e) = Note::from_string("") {
            assert_eq!(e, "No tonic note provided.");
        } else { 
            panic!();
        }
    }

    #[test]
    fn test_from_invalid_string() {
        if let Err(e) = Note::from_string("X") {
            assert_eq!(e, "Invalid tonic note");
        } else {
            panic!();
        }
    }

    #[test]
    fn test_from_invalid_accidental() {
        if let Err(e) = Note::from_string("A@") {
            assert_eq!(e, "Invalid tonic note");
        } else {
            panic!();
        }
    }

    #[test]
    fn test_natural_from_string() {
        let mut note = Note::from_string("A");
        assert_eq!(note, 
                   Ok(Note {
                       letter: Letter::A,
                       accidental_count: 0
                   }));
        note = Note::from_string("F");
        assert_eq!(note, 
                   Ok(Note {
                       letter: Letter::F,
                       accidental_count: 0
                   }));
    }

    #[test]
    fn test_accidental_from_string() {
        let mut note = Note::from_string("A#");
        assert_eq!(note, 
                   Ok(Note {
                       letter: Letter::A,
                       accidental_count: 1
                   }));
        note = Note::from_string("Gb");
        assert_eq!(note, 
                   Ok(Note {
                       letter: Letter::G,
                       accidental_count: -1 
                   }));
    }

}

