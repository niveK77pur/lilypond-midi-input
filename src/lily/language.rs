use super::LilypondLanguageError;

make_lily_str_map!(
    /// List of supported languages for note names
    ///
    /// See: <https://lilypond.org/doc/v2.25/Documentation/notation/writing-pitches#note-names-in-other-languages>
    #[derive(Default)]
    Language;
    LilypondLanguageError::InvalidLanguageString;
    #[default]
    Nederlands, "nederlands";
);

impl Language {
    pub fn note_to_str(&self, note: &Note) -> &'static str {
        match self {
            Language::Nederlands => match note {
                Note::C => "c",
                Note::CFlat => "ces",
                Note::CSharp => "cis",
                Note::CSharpSharp => "cisis",
                Note::D => "d",
                Note::DFlat => "des",
                Note::DSharp => "dis",
                Note::E => "e",
                Note::EFlat => "ees",
                Note::ESharp => "eis",
                Note::F => "f",
                Note::FFlat => "fes",
                Note::FSharp => "fis",
                Note::FSharpSharp => "fisis",
                Note::G => "g",
                Note::GFlat => "ges",
                Note::GSharp => "gis",
                Note::GSharpSharp => "gisis",
                Note::A => "a",
                Note::AFlat => "aes",
                Note::ASharp => "ais",
                Note::B => "b",
                Note::BFlat => "bes",
                Note::BSharp => "bis",
            },
        }
    }
}

/// This enum is used to specify notes in the source code, but have them be
/// converted to a language dependent string.
///
/// See: <https://lilypond.org/doc/v2.25/Documentation/notation/writing-pitches#note-names-in-other-languages>
#[derive(Debug)]
pub enum Note {
    C,
    CFlat,
    CSharp,
    CSharpSharp,
    D,
    DFlat,
    DSharp,
    E,
    EFlat,
    ESharp,
    F,
    FFlat,
    FSharp,
    FSharpSharp,
    G,
    GFlat,
    GSharp,
    GSharpSharp,
    A,
    AFlat,
    ASharp,
    B,
    BFlat,
    BSharp,
}
