pub mod lily;
pub mod midi;

pub type MidiNote = u8;

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
    PedalChord, "pedal-chord", "pc";
    /// Behave like [Mode::Single] when the pedal is pressed, otherwise behave like [Mode::Chord]
    PedalSingle, "pedal-single", "ps";
);

pub enum InputModeError {
    InvalidModeString(String),
}

/// List all available options to stdout
pub trait ListOptions {
    fn list_options();
}
