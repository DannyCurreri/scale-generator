#[derive(Debug)]
pub enum Error<'a> {
    InvalidTonic(&'a str),
    InvalidIntervals(&'a str),
}

pub struct Scale {
    notes: Vec<String>,
}

#[derive(PartialEq, Debug)]
enum Signature {
    Natural,
    Sharp,
    Flat, 
}

const SHARPS: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const FLATS: [&str; 12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

pub mod error_messages {
    pub const INVALID_TONIC: &str ="Invalid tonic. Use letter A-G for maj or a-g for min, optionally followed # or b for accidental.";
    pub const INVALID_INTERVAL: &str ="Invalid interval input string. Use M for whole step, m for half step, or A for augmented second.";
}

impl Scale {
    pub fn new<'a>(tonic: &'a str, intervals: &'a str) -> Result<Scale, Error<'a>> {
        let mut scale = Scale { notes: vec![] };

        let note_bank = match Scale::signature(tonic) {
            Ok(Signature::Natural) => SHARPS,
            Ok(Signature::Sharp) => SHARPS,
            Ok(Signature::Flat) => FLATS,
            Err(e) => return Err(e),
        };

        // convert tonic to a Vec to capitalize first letter, then convert back to
        // str
        let mut v: Vec<char> = tonic.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s: String = v.into_iter().collect();
        let tonic = &s;

        let mut position = note_bank
            .iter()
            .position(|&x| x == tonic)
            .unwrap();
        println!("start position: {:?}", position);

        scale.notes.push(note_bank[position].to_string());

        for step in intervals.chars() {
            if step == 'M' {
                position += 2;
                scale.notes.push(note_bank[position % 12].to_string());
            }
            else if step == 'm' {
                position += 1;
                scale.notes.push(note_bank[position % 12].to_string());
            }
            else if step == 'A' {
                position += 3;
                scale.notes.push(note_bank[position % 12].to_string());
            }
            else {
                return Err(Error::InvalidIntervals(error_messages::INVALID_INTERVAL));
            }
        }
        Ok(scale)
    }

    pub fn chromatic<'a>(tonic: &'a str) -> Result<Scale, Error<'a>> {
        let mut scale = Scale { notes: vec![] };

        let note_bank = match Scale::signature(tonic) {
            Ok(Signature::Natural) => SHARPS,
            Ok(Signature::Sharp) => SHARPS,
            Ok(Signature::Flat) => FLATS,
            Err(e) => return Err(e),
        };

        let start_position = note_bank.iter().position(|&x| x == tonic).unwrap();
        println!("start position: {:?}", start_position);

        for i in start_position..start_position+13 {
            scale.notes.push(String::from(note_bank[i % 12]))
        }

        Ok(scale)
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }

    fn signature(tonic: &str) -> Result<Signature, Error> {
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
            return Err(Error::InvalidTonic(error_messages::INVALID_INTERVAL));
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
