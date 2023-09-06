#[derive(Debug)]
pub enum LilypondNoteError {
    /// Some functions require the note to be within an octave (integer between
    /// 0 to 11)
    OutsideOctave(u8),
    /// The string was not recognized for key signatures
    InvalidKeyString,
}

#[derive(Debug)]
pub enum LilypondAccidentalError {
    /// The string was not recognized for accidentals
    InvalidAccidentalString,
}
