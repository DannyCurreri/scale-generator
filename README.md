# Scale Generator

Why deal with browsers, search engines and loading entire web pages just to look up the notes of a musical scale? Do it in a flash right in the terminal. Scale Generator will print any of the seven diatonic modes for any root note with at most one accidental, for 147 possible scales. It will always provide the musically correct phrasing; that is to say, each note letter A-G will appear exactly once with their accidentals reflecting the scale intervals.

## Install

Will build and install a Rust binary called `scg`

Prerequisite: Must have [Rust](https://rustup.rs/) installed on machine.

    git clone https://github.com/DannyCurreri/scale-generator.git
    cargo install --path scale_generator

## Usage

    usage: scg tonic_note [mode]

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

If `mode` is not provided, an upper case tonic note defaults to major and lower case defaults to minor.

## Examples

    $ scg C
    C Major scale: C D E F G A B

    $ scg a
    A Minor scale: A B C D E F G

    $ scg C# dorian
    C# Dorian scale: C# D# E F# G# A# B

    $ scg Ab mixolydian
    Ab Mixolydian scale: Ab Bb C Db Eb F Gb
