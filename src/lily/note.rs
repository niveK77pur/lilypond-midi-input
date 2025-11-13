use std::fmt::Display;

use regex::Regex;

use crate::MidiNote;

use super::language::Note;
use super::{LilyAccidental, LilyKeySignature, LilyParameters, LilypondNoteError};

#[derive(Debug)]
pub struct LilyNote<'a> {
    /// the LilyPond note string
    letter: &'a str,
    /// octave indication is very small, we do not need a large integer
    octave: i8,
    /// original midi value
    note: MidiNote,
    /// absolute octave to include for octave check
    octave_check: Option<i8>,
}

#[derive(Debug)]
pub struct LilyNoteRendered {
    /// The string representation of the note (without octave)
    note_name: &'static str,
    /// The midi note value with the accidental removed (needed for calculating relative octave entry)
    note_no_accidental: MidiNote,
}

impl<'a> LilyNote<'a> {
    pub fn new(value: MidiNote, parameters: &'a LilyParameters) -> Self {
        let LilyParameters {
            alterations,
            global_alterations,
            octave_entry,
            previous_absolute_note_reference,
            octave_check_on_next_note,
            octave_check_notes,
            ..
        } = parameters;
        let note_rendered = Self::render(value, parameters);
        let absolute_octave = (value as i16 / 12) as i8 - 4;
        let mut octave = match octave_entry {
            super::OctaveEntry::Absolute => absolute_octave,
            super::OctaveEntry::Relative => match previous_absolute_note_reference {
                Some(panr) => {
                    let panr_rendered = Self::render(*panr, parameters);
                    let next_octave_distance = if (
                        // The previous note is a B
                        panr_rendered.note_no_accidental % 12 == 11
                    ) && (
                        // The current note is an F
                        note_rendered.note_no_accidental % 12 == 5
                    ) && (
                        // Only consider B to F, not F to B
                        panr_rendered.note_no_accidental < note_rendered.note_no_accidental
                    ) {
                        // Handle special tritone case from B to F
                        5
                    } else {
                        // absolute relative distance until an octave mark is needed
                        6
                    };
                    let interval: i16 = (note_rendered.note_no_accidental as i16)
                        - (panr_rendered.note_no_accidental as i16);
                    (if interval > next_octave_distance {
                        // ceil division
                        (interval - next_octave_distance - 1) / 12 + 1
                    } else if interval < -next_octave_distance {
                        // floor division
                        (interval + next_octave_distance) / 12 - 1
                    } else {
                        // we are within the a fifth
                        0
                    }) as i8
                }
                None => 0, // We cannot determine relative octave, we rely on octave check
            },
        };
        let mut octave_check = match *octave_check_on_next_note || *octave_check_notes {
            true => Some(absolute_octave),
            false => match octave_entry {
                super::OctaveEntry::Absolute => None,
                super::OctaveEntry::Relative => match previous_absolute_note_reference {
                    Some(_) => None,
                    None => Some(absolute_octave),
                },
            },
        };
        LilyNote {
            letter: match global_alterations.get(&value) {
                Some(text) => {
                    octave = 0; // we do not want octaves for global custom alterations
                    if let super::OctaveEntry::Relative = octave_entry {
                        // we cannot determine relative position here, add octave check
                        octave_check = Some(absolute_octave);
                    }
                    text
                }
                None => match alterations.get(&(value % 12)) {
                    Some(text) => match octave_entry {
                        super::OctaveEntry::Absolute => Self::adjust_ottavation(text, &mut octave),
                        super::OctaveEntry::Relative => {
                            // we cannot easily determine relative position here, add octave check
                            octave_check = Some(absolute_octave);
                            Self::adjust_ottavation(
                                text,
                                octave_check
                                    .as_mut()
                                    .expect("Octave check has just been set"),
                            )
                        }
                    },
                    None => note_rendered.note_name,
                },
            },
            octave,
            note: value,
            octave_check,
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

    /// Function to render the given midi note
    ///
    /// # Panics
    ///
    /// The given midi note value is put through a modulo 12 to map each note in the scale to a
    /// specific rendered note. The panic occurs if the value of the module operation falls outside
    /// of 0..11 (inclusive), which will never occur.
    fn render(note: MidiNote, parameters: &LilyParameters) -> LilyNoteRendered {
        let LilyParameters {
            key,
            accidentals,
            language,
            ..
        } = parameters;
        use LilyKeySignature::*;
        match note % 12 {
            0 => match key {
                CSharpMajor | ASharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::BSharp),
                    note_no_accidental: note - 1,
                },
                CSharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::BSharp),
                    note_no_accidental: note - 1,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::C),
                    note_no_accidental: note,
                },
            },
            1 => match key {
                AFlatMajor | FMinor | DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor
                | CFlatMajor | AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::DFlat),
                    note_no_accidental: note + 1,
                },
                DMajor | BMinor | AMajor | FSharpMinor | EMajor | CSharpMinor | BMajor
                | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => {
                    LilyNoteRendered {
                        note_name: language.note_to_str(&Note::CSharp),
                        note_no_accidental: note - 1,
                    }
                }
                DMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::CSharp),
                    note_no_accidental: note - 1,
                },
                _ => match accidentals {
                    LilyAccidental::Sharps => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::CSharp),
                        note_no_accidental: note - 1,
                    },
                    LilyAccidental::Flats => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::DFlat),
                        note_no_accidental: note + 1,
                    },
                },
            },
            2 => match key {
                EFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::D),
                    note_no_accidental: note,
                },
                DSharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::CSharpSharp),
                    note_no_accidental: note - 2,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::D),
                    note_no_accidental: note,
                },
            },
            3 => match key {
                BFlatMajor | GMinor | EFlatMajor | CMinor | AFlatMajor | FMinor | DFlatMajor
                | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => {
                    LilyNoteRendered {
                        note_name: language.note_to_str(&Note::EFlat),
                        note_no_accidental: note + 1,
                    }
                }
                EMajor | CSharpMinor | BMajor | GSharpMinor | FSharpMajor | DSharpMinor
                | CSharpMajor | ASharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::DSharp),
                    note_no_accidental: note - 1,
                },
                EMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::DSharp),
                    note_no_accidental: note - 1,
                },
                _ => match accidentals {
                    LilyAccidental::Sharps => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::DSharp),
                        note_no_accidental: note - 1,
                    },
                    LilyAccidental::Flats => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::EFlat),
                        note_no_accidental: note + 1,
                    },
                },
            },
            4 => match key {
                CFlatMajor | AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::FFlat),
                    note_no_accidental: note + 1,
                },
                FMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::E),
                    note_no_accidental: note,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::E),
                    note_no_accidental: note,
                },
            },
            5 => match key {
                FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::ESharp),
                    note_no_accidental: note - 1,
                },
                FSharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::ESharp),
                    note_no_accidental: note - 1,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::F),
                    note_no_accidental: note,
                },
            },
            6 => match key {
                DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => {
                    LilyNoteRendered {
                        note_name: language.note_to_str(&Note::GFlat),
                        note_no_accidental: note + 1,
                    }
                }
                GMajor | EMinor | DMajor | BMinor | AMajor | FSharpMinor | EMajor | CSharpMinor
                | BMajor | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => {
                    LilyNoteRendered {
                        note_name: language.note_to_str(&Note::FSharp),
                        note_no_accidental: note - 1,
                    }
                }
                GMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::FSharp),
                    note_no_accidental: note - 1,
                },
                _ => match accidentals {
                    LilyAccidental::Sharps => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::FSharp),
                        note_no_accidental: note - 1,
                    },
                    LilyAccidental::Flats => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::GFlat),
                        note_no_accidental: note + 1,
                    },
                },
            },
            7 => match key {
                AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::G),
                    note_no_accidental: note,
                },
                GSharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::FSharpSharp),
                    note_no_accidental: note - 2,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::G),
                    note_no_accidental: note,
                },
            },
            8 => match key {
                EFlatMajor | CMinor | AFlatMajor | FMinor | DFlatMajor | BFlatMinor
                | GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::AFlat),
                    note_no_accidental: note + 1,
                },
                AMajor | FSharpMinor | EMajor | CSharpMinor | BMajor | GSharpMinor
                | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::GSharp),
                    note_no_accidental: note - 1,
                },
                AMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::GSharp),
                    note_no_accidental: note - 1,
                },
                _ => match accidentals {
                    LilyAccidental::Sharps => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::GSharp),
                        note_no_accidental: note - 1,
                    },
                    LilyAccidental::Flats => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::AFlat),
                        note_no_accidental: note + 1,
                    },
                },
            },
            9 => match key {
                BFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::A),
                    note_no_accidental: note,
                },
                ASharpMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::GSharpSharp),
                    note_no_accidental: note - 2,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::A),
                    note_no_accidental: note,
                },
            },
            10 => match key {
                FMajor | DMinor | BFlatMajor | GMinor | EFlatMajor | CMinor | AFlatMajor
                | FMinor | DFlatMajor | BFlatMinor | GFlatMajor | EFlatMinor | CFlatMajor
                | AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::BFlat),
                    note_no_accidental: note + 1,
                },
                BMajor | GSharpMinor | FSharpMajor | DSharpMinor | CSharpMajor | ASharpMinor => {
                    LilyNoteRendered {
                        note_name: language.note_to_str(&Note::ASharp),
                        note_no_accidental: note - 1,
                    }
                }
                BMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::ASharp),
                    note_no_accidental: note - 1,
                },
                _ => match accidentals {
                    LilyAccidental::Sharps => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::ASharp),
                        note_no_accidental: note - 1,
                    },
                    LilyAccidental::Flats => LilyNoteRendered {
                        note_name: language.note_to_str(&Note::BFlat),
                        note_no_accidental: note + 1,
                    },
                },
            },
            11 => match key {
                GFlatMajor | EFlatMinor | CFlatMajor | AFlatMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::CFlat),
                    note_no_accidental: note + 1,
                },
                CMinor => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::B),
                    note_no_accidental: note,
                },
                _ => LilyNoteRendered {
                    note_name: language.note_to_str(&Note::B),
                    note_no_accidental: note,
                },
            },
            _ => panic!("Note within octave"),
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
                    octave_check: None,
                })
            }
            None => Err(LilypondNoteError::InvalidNoteString(s.into())),
        }
    }
}

impl<'a> From<&LilyNote<'a>> for String {
    fn from(value: &LilyNote) -> Self {
        let LilyNote {
            letter,
            octave,
            octave_check,
            ..
        } = value;
        let octave = match octave.cmp(&0) {
            std::cmp::Ordering::Less => ",".repeat(octave.unsigned_abs() as usize),
            std::cmp::Ordering::Equal => "".into(),
            std::cmp::Ordering::Greater => "'".repeat(*octave as usize),
        };
        match octave_check {
            Some(check) => {
                let octave_check = match check.cmp(&0) {
                    std::cmp::Ordering::Less => ",".repeat(check.unsigned_abs() as usize),
                    std::cmp::Ordering::Equal => "".into(),
                    std::cmp::Ordering::Greater => "'".repeat(*check as usize),
                };
                format!("{}{}={}", letter, octave, octave_check)
            }
            None => format!("{}{}", letter, octave),
        }
    }
}

impl<'a> Display for LilyNote<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
