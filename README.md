# Scale Generator

Generate diatonic scales from the command line.

## Instructions

Requires Rust and Cargo installed on machine.

Clone repository and from project folder run with cargo run.

The program accepts two arguments. The first is mandatory and is the tonic note of the scale to be generated, represented by a letter A-G optionally followed by '#' or 'b' for sharp or flat. The second argument is optional and specifies the type of scale (mode) to be generated. If a second argument is not provided, an upper case tonic will default to a major scale and a lower case tonic to a minor scale. Current accepted scale types are any of the seven diatonic modes.

## Examples

    $ cargo run C
    > C Major scale: C D E F G A B

    $ cargo run a
    > A Minor scale: A B C D E F G

    $ cargo run C# Dorian
    > C# Dorian scale: C# D# E F# G# A# B

    $ cargo run Ab Mixolydian
    > Ab Mixolydian scale: Ab Bb C Db Eb F Gb
