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
    Catalan, "catalan", "català";
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
            Language::Catalan => match note {
                Note::C => "do",
                Note::CFlat => "dob",
                Note::CSharp => "dod",
                Note::CSharpSharp => "dodd",
                Note::D => "re",
                Note::DFlat => "reb",
                Note::DSharp => "red",
                Note::E => "mi",
                Note::EFlat => "mib",
                Note::ESharp => "mid",
                Note::F => "fa",
                Note::FFlat => "fab",
                Note::FSharp => "fad",
                Note::FSharpSharp => "fadd",
                Note::G => "sol",
                Note::GFlat => "solb",
                Note::GSharp => "sold",
                Note::GSharpSharp => "soldd",
                Note::A => "la",
                Note::AFlat => "lab",
                Note::ASharp => "lad",
                Note::B => "si",
                Note::BFlat => "sib",
                Note::BSharp => "sid",
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
