pub mod lily;
pub mod midi;

make_lily_str_map!(
    /// How should note inputs behave
    InputMode;
    InputModeError::InvalidModeString;
    /// Enter one note at a time
    Single, "single", "s";
    /// Enter notes as chords
    ///
    /// Holding down multiple notes will aggregate them into a chord. Once everything was released,
    /// a chord with the given notes is created.
    Chord, "chord", "c";
    /// Behave like [Mode::Chord] when the pedal is pressed, otherwise behave like [Mode::Single]
    Pedal, "pedal", "p";
);

pub enum InputModeError {
    InvalidModeString(String),
}
