use super::LilypondAccidentalError;

make_lily_str_map!(
    /// The accidentals to use for out of key notes.
    LilyAccidental;
    LilypondAccidentalError::InvalidAccidentalString;
    Sharps, "sharps", "s";
    Flats, "flats", "f";
);
