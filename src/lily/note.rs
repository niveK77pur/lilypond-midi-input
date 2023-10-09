use std::fmt::Display;

use regex::Regex;

use crate::MidiNote;

use super::{LilyAccidental, LilyKeySignature, LilyParameters, LilypondNoteError};

#[derive(Debug)]
pub struct LilyNote<'a> {
    /// the LilyPond note string
    letter: &'a str,
    /// octave indication is very small, we do not need a large integer
    octave: i8,
    /// original midi value
    note: MidiNote,
}

impl<'a> LilyNote<'a> {
    pub fn new(value: MidiNote, parameters: &'a LilyParameters) -> Self {
        let LilyParameters {
            alterations,
            global_alterations,
            ..
        } = parameters;
        let mut octave = (value as i16 / 12) as i8 - 4;
        LilyNote {
            letter: match global_alterations.get(&value) {
                Some(text) => {
                    octave = 0; // we do not want octaves for global custom alterations
                    text
                }
                None => match alterations.get(&(value % 12)) {
                    Some(text) => Self::adjust_ottavation(text, &mut octave),
                    None => Self::note_name(value % 12, parameters).expect("Note within octave"),
                },
            },
            octave,
            note: value,
        }
    }

    /// Function to adjust the `octave` if there are trailing `+` or `-`
    ///
    /// # Panics
    ///
    /// Panics if a character other than `+` or `-` was matched by the regex to check for
    /// ottavation adjustments. This panic is not expected to occur.
    fn adjust_ottavation(note: &'a str, octave: &mut i8) -> &'a str {
        let re_note_octave =
            Regex::new(r"(?<note>.*?)(?<ottavation>\++|-+)$").expect("Regex is valid");
        match re_note_octave.captures(note) {
            Some(caps) => {
                *octave += match &caps["ottavation"].chars().next().unwrap() {
                    '+' => caps["ottavation"].len() as i8,
                    '-' => -(caps["ottavation"].len() as i8),
                    _ => panic!("Nothing else should have been matched"),
                };
                // get substring for first capture group
                caps.extract::<2>().1[0]
            }
            None => note,
        }
    }

    fn note_name(
        note: MidiNote,
        parameters: &LilyParameters,
    ) -> Result<&'static str, LilypondNoteError> {
        let LilyParameters {
            key, accidentals, ..
        } = parameters;
        use LilyKeySignature::*;
        match note {
            0 => match key {
                CSharpMajor | ASharpMinor => Ok("bis"),
                CSharpMinor => Ok("bis"),
                _ => Ok("c"),
            },
            1 => match key {
                AFlatMajor | FMinor | DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor
                | CFlatMajor | AFlatMinor => Ok("des"),
                DMajor | BMinor | AMajor | FSharpMinor | EMajor | CSharpMinor | BMajor
                | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => Ok("cis"),
                DMinor => Ok("cis"),
                _ => match accidentals {
                    LilyAccidental::Sharps => Ok("cis"),
                    LilyAccidental::Flats => Ok("des"),
                },
            },
            2 => match key {
                EFlatMinor => Ok("d"),
                DSharpMinor => Ok("cisis"),
                _ => Ok("d"),
            },
            3 => match key {
                BFlatMajor | GMinor | EFlatMajor | CMinor | AFlatMajor | FMinor | DFlatMajor
                | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => Ok("ees"),
                EMajor | CSharpMinor | BMajor | GSharpMinor | FSharpMajor | DSharpMinor
                | CSharpMajor | ASharpMinor => Ok("dis"),
                EMinor => Ok("dis"),
                _ => match accidentals {
                    LilyAccidental::Sharps => Ok("dis"),
                    LilyAccidental::Flats => Ok("ees"),
                },
            },
            4 => match key {
                CFlatMajor | AFlatMinor => Ok("fes"),
                FMinor => Ok("e"),
                _ => Ok("e"),
            },
            5 => match key {
                FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => Ok("eis"),
                FSharpMinor => Ok("eis"),
                _ => Ok("f"),
            },
            6 => match key {
                DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => {
                    Ok("ges")
                }
                GMajor | EMinor | DMajor | BMinor | AMajor | FSharpMinor | EMajor | CSharpMinor
                | BMajor | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => {
                    Ok("fis")
                }
                GMinor => Ok("fis"),
                _ => match accidentals {
                    LilyAccidental::Sharps => Ok("fis"),
                    LilyAccidental::Flats => Ok("ges"),
                },
            },
            7 => match key {
                AFlatMinor => Ok("g"),
                GSharpMinor => Ok("fisis"),
                _ => Ok("g"),
            },
            8 => match key {
                EFlatMajor | CMinor | AFlatMajor | FMinor | DFlatMajor | BFlatMinor
                | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => Ok("aes"),
                AMajor | FSharpMinor | EMajor | CSharpMinor | BMajor | GSharpMinor
                | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => Ok("gis"),
                AMinor => Ok("gis"),
                _ => match accidentals {
                    LilyAccidental::Sharps => Ok("gis"),
                    LilyAccidental::Flats => Ok("aes"),
                },
            },
            9 => match key {
                BFlatMinor => Ok("a"),
                ASharpMinor => Ok("gisis"),
                _ => Ok("a"),
            },
            10 => match key {
                FMajor | DMinor | BFlatMajor | GMinor | EFlatMajor | CMinor | AFlatMajor
                | FMinor | DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor
                | AFlatMinor => Ok("bes"),
                BMajor | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => {
                    Ok("ais")
                }
                BMinor => Ok("ais"),
                _ => match accidentals {
                    LilyAccidental::Sharps => Ok("ais"),
                    LilyAccidental::Flats => Ok("bes"),
                },
            },
            11 => match key {
                GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => Ok("ces"),
                CMinor => Ok("b"),
                _ => Ok("b"),
            },
            _ => Err(LilypondNoteError::OutsideOctave(note)),
        }
    }

    pub fn note(&self) -> &MidiNote {
        &self.note
    }

    pub fn from_lilypond_str(s: &'a str) -> Result<Self, LilypondNoteError> {
        let re_lilypond_note =
            Regex::new(r"(?<note>[abcdefg](?:[ie]?s)*)(?<octave>[',]+)?").expect("Regex is valid");
        match re_lilypond_note.captures(s) {
            Some(caps) => {
                let letter = caps.name("note").unwrap().as_str();
                let octave = match caps.name("octave") {
                    Some(o) => match o.as_str().chars().next().expect("Octave is not empty") {
                        ',' => -(o.len() as i8),
                        '\'' => o.len() as i8,
                        _ => panic!("This case should not happen. Octave should be `'` or `,`"),
                    },
                    None => 0,
                };
                let note: MidiNote = (octave + 4) as u8 * 12
                    + match letter {
                        "c" | "bis" | "deses" => 0,
                        "cis" | "bisis" | "des" => 1,
                        "d" | "cisis" | "eeses" => 2,
                        "dis" | "ees" | "feses" => 3,
                        "e" | "disis" | "fes" => 4,
                        "f" | "eis" | "geses" => 5,
                        "fis" | "eisis" | "ges" => 6,
                        "g" | "fisis" | "aeses" => 7,
                        "gis" | "aes" => 8,
                        "a" | "gisis" | "beses" => 9,
                        "ais" | "bes" | "ceses" => 10,
                        "b" | "aisis" | "ces" => 11,
                        _ => panic!("Unrecognized note letter: {letter}"),
                    };
                Ok(LilyNote {
                    letter,
                    octave,
                    note,
                })
            }
            None => Err(LilypondNoteError::InvalidNoteString(s.into())),
        }
    }
}

impl<'a> From<&LilyNote<'a>> for String {
    fn from(value: &LilyNote) -> Self {
        let LilyNote { letter, octave, .. } = value;
        let octave = match octave.cmp(&0) {
            std::cmp::Ordering::Less => ",".repeat(octave.unsigned_abs() as usize),
            std::cmp::Ordering::Equal => "".into(),
            std::cmp::Ordering::Greater => "'".repeat(*octave as usize),
        };
        format!("{}{}", letter, octave)
    }
}

impl<'a> Display for LilyNote<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
