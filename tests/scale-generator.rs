use scale_generator::notes::{Letter, Note};
use scale_generator::*;

/// Process a single test case for the property `interval`
///
/// All cases for the `interval` property are implemented
/// in terms of this function.
fn process_interval_case(tonic: Note, mode: Mode, expected: &[&str]) {
    let s = Scale::new(tonic, mode);
    assert_eq!(s.enumerate(), expected);
}

#[test]
/// Simple major scale
fn test_simple_major_scale() {
    process_interval_case(
        Note::new(Letter::C, 0),
        Mode::Ionian,
        &["C", "D", "E", "F", "G", "A", "B"],
    );
}

#[test]
/// Major scale with sharps
fn test_major_scale_with_sharps() {
    process_interval_case(
        Note::new(Letter::G, 0),
        Mode::Ionian,
        &["G", "A", "B", "C", "D", "E", "F#"],
    );
}

#[test]
/// Major scale with flats
fn test_major_scale_with_flats() {
    process_interval_case(
        Note::new(Letter::F, 0),
        Mode::Ionian,
        &["F", "G", "A", "Bb", "C", "D", "E"],
    );
}

#[test]
/// Minor scale with sharps
fn test_minor_scale_with_sharps() {
    process_interval_case(
        Note::new(Letter::F, 1),
        Mode::Aeolian,
        &["F#", "G#", "A", "B", "C#", "D", "E"],
    );
}

#[test]
/// Minor scale with flats
fn test_minor_scale_with_flats() {
    process_interval_case(
        Note::new(Letter::B, -1),
        Mode::Aeolian,
        &["Bb", "C", "Db", "Eb", "F", "Gb", "Ab"],
    );
}

#[test]
/// Dorian mode
fn test_dorian_mode() {
    process_interval_case(
        Note::new(Letter::D, 0),
        Mode::Dorian,
        &["D", "E", "F", "G", "A", "B", "C"],
    );
}

#[test]
/// Mixolydian mode
fn test_mixolydian_mode() {
    process_interval_case(
        Note::new(Letter::E, -1),
        Mode::Mixolydian,
        &["Eb", "F", "G", "Ab", "Bb", "C", "Db"],
    );
}

#[test]
/// Lydian mode
fn test_lydian_mode() {
    process_interval_case(
        Note::new(Letter::A, 0),
        Mode::Lydian,
        &["A", "B", "C#", "D#", "E", "F#", "G#"],
    );
}

#[test]
/// Phrygian mode
fn test_phrygian_mode() {
    process_interval_case(
        Note::new(Letter::E, 0),
        Mode::Phrygian,
        &["E", "F", "G", "A", "B", "C", "D"],
    );
}

#[test]
/// Locrian mode
fn test_locrian_mode() {
    process_interval_case(
        Note::new(Letter::G, 0),
        Mode::Locrian,
        &["G", "Ab", "Bb", "C", "Db", "Eb", "F"],
    );
}
