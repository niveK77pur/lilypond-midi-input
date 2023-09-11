use std::fmt::Display;

use crate::MidiNote;

use super::{LilyAccidental, LilyKeySignature, LilyParameters, LilypondNoteError};

#[derive(Debug)]
pub struct LilyNote<'a> {
    /// the LilyPond note string
    letter: &'a str,
    /// octave indication is very small, we do not need a large integer
    octave: i8,
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
                    Some(text) => text,
                    None => Self::note_name(value % 12, parameters).expect("Note within octave"),
                },
            },
            octave,
        }
    }

    fn note_name(note: MidiNote, parameters: &LilyParameters) -> Result<&'static str, LilypondNoteError> {
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
}

impl<'a> From<&LilyNote<'a>> for String {
    fn from(value: &LilyNote) -> Self {
        let LilyNote { letter, octave } = value;
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
