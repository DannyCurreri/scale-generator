#[derive(PartialEq, Debug)]
struct Note {
    letter: Letter,
    accidental_count: i32,
}

impl Note {
    fn next(&self, step: i32) -> Self {
        let letter = self.letter.next();
        let accidental_count = step - letter.incremental_value();

        Note {
            letter,
            accidental_count,
        }
    }
    
    fn as_string(&self) -> String {
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

#[derive(PartialEq, Debug)]
enum Letter {
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
        let mut note = Letter::A;
        assert_eq!(Letter::A.incremental_value(), 2);
        assert_eq!(Letter::C.incremental_value(), 1);
    }

    #[test]
    fn test_str() {
        let mut note = Letter::A;
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
    fn test_Bsharp_whole_step() {
        let note = Note { 
            letter: Letter::B, 
            accidental_count: 1, 
        };
        assert_eq!(note.next(2).as_string(), "C#");
    }

    #[test]
    fn test_Bsharp_half_step() {
        let note = Note { 
            letter: Letter::B, 
            accidental_count: 1, 
        };
        assert_eq!(note.next(1).as_string(), "C");
    }

    #[test]
    fn test_E_half_step() {
        let note = Note { 
            letter: Letter::E, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(1).as_string(), "F");
    }

    #[test]
    fn test_E_whole_step() {
        let note = Note { 
            letter: Letter::E, 
            accidental_count: 0, 
        };
        assert_eq!(note.next(2).as_string(), "F#");
    }

}

