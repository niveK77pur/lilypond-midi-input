use std::collections::HashMap;

use clap::builder::PossibleValue;

type Alteration = HashMap<u8, String>;

#[derive(Debug)]
pub struct LilyParameters {
    key: LilyKeySignature,
    accidentals: LilyAccidental,
    /// custom alterations within an octave (0-11)
    alterations: Alteration,
    /// custom alterations over all notes
    global_alterations: Alteration,
}

impl LilyParameters {
    pub fn new(
        key: LilyKeySignature,
        accidentals: LilyAccidental,
        alterations: Alteration,
        global_alterations: Alteration,
    ) -> Self {
        LilyParameters {
            key,
            accidentals,
            alterations,
            global_alterations,
        }
    }

    pub fn key(&self) -> &LilyKeySignature {
        &self.key
    }
    pub fn set_key(&mut self, key: LilyKeySignature) {
        self.key = key
    }
    pub fn accidentals(&self) -> &LilyAccidental {
        &self.accidentals
    }
    pub fn set_accidentals(&mut self, accidentals: LilyAccidental) {
        self.accidentals = accidentals
    }
    pub fn alterations(&self) -> &Alteration {
        &self.alterations
    }
    pub fn set_alterations(&mut self, alterations: Alteration) {
        self.alterations = alterations
    }
    pub fn add_alteration(&mut self, note: u8, value: String) {
        self.alterations.insert(note, value);
    }
    pub fn clear_alterations(&mut self) {
        self.set_alterations(HashMap::new());
    }
    pub fn global_alterations(&self) -> &Alteration {
        &self.global_alterations
    }
    pub fn set_global_alterations(&mut self, global_alterations: Alteration) {
        self.global_alterations = global_alterations
    }
    pub fn add_global_alteration(&mut self, note: u8, value: String) {
        self.global_alterations.insert(note, value);
    }
    pub fn clear_global_alterations(&mut self) {
        self.set_global_alterations(HashMap::new());
    }
}

/// Create mappings for enum variants and corresponding string representations
macro_rules! make_lily_str_map {
    ($(#[$outer:meta])* $name:ident;
     $err:ident::$err_variant:ident;
     $($key:ident, $main:literal $(, $string:literal)*);*;
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name {
            $($key),*
        }

        impl std::str::FromStr for $name {
            type Err = $err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($main | stringify!($key) $(|$string)* => Ok($name::$key),)*
                    _ => Err($err::$err_variant),
                }
            }
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = $err;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                <Self as std::str::FromStr>::from_str(value)
            }
        }

        impl TryFrom<$name> for &str {
            type Error = String;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                match value {
                    $($name::$key => Ok($main)),*
                }
            }
        }

        impl clap::ValueEnum for $name {
            fn value_variants<'a>() -> &'a [Self] {
                &[$($name::$key),*]
            }

            fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
                Some(match self {
                    $($name::$key => clap::builder::PossibleValue::new($main).help(stringify!($key))),*
                })
            }
        }
    };
}

make_lily_str_map!(
    /// List of possible musical key signatures
    LilyKeySignature;
    LilypondNoteError::InvalidKeyString;
    CFlatMajor,  "cesM" ; // 7 flats
    GFlatMajor,  "gesM" ; // 6 flats
    DFlatMajor,  "desM" ; // 5 flats
    AFlatMajor,  "aesM" ; // 4 flats
    EFlatMajor,  "eesM" ; // 3 flats
    BFlatMajor,  "besM" ; // 2 flats
    FMajor,      "fM"   ; // 1 flat
    CMajor,      "cM"   ; // 0 flats/sharps
    GMajor,      "gM"   ; // 1 sharp
    DMajor,      "dM"   ; // 2 sharps
    AMajor,      "aM"   ; // 3 sharps
    EMajor,      "eM"   ; // 4 sharps
    BMajor,      "bM"   ; // 5 sharps
    FSharpMajor, "fisM" ; // 6 sharps
    CSharpMajor, "cisM" ; // 7 sharps
    AFlatMinor,  "dm"   ; // 7 flats
    EFlatMinor,  "gm"   ; // 6 flats
    BFlatMinor,  "cm"   ; // 5 flats
    FMinor,      "fm"   ; // 4 flats
    CMinor,      "besm" ; // 3 flats
    GMinor,      "eesm" ; // 2 flats
    DMinor,      "aesm" ; // 1 flat
    AMinor,      "am"   ; // 0 flats/sharps
    EMinor,      "em"   ; // 1 sharp
    BMinor,      "bm"   ; // 2 sharps
    FSharpMinor, "fism" ; // 3 sharps
    CSharpMinor, "cism" ; // 4 sharps
    GSharpMinor, "gism" ; // 5 sharps
    DSharpMinor, "dism" ; // 6 sharps
    ASharpMinor, "aism" ; // 7 sharps
);

make_lily_str_map!(
    /// The accidentals to use for out of key notes.
    LilyAccidental;
    LilypondAccidentalError::InvalidAccidentalString;
    Sharps, "sharps", "s";
    Flats, "flats", "f";
);

#[derive(Debug)]
pub struct LilyNote<'a> {
    /// the LilyPond note string
    letter: &'a str,
    /// octave indication is very small, we do not need a large integer
    octave: i8,
}

impl<'a> LilyNote<'a> {
    pub fn new(value: u8, parameters: &'a LilyParameters) -> Self {
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

    fn note_name(note: u8, parameters: &LilyParameters) -> Result<&'static str, LilypondNoteError> {
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
            _ => Err(LilypondNoteError::OutsideOctave),
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

#[derive(Debug)]
pub enum LilypondNoteError {
    /// Some functions require the note to be within an octave (integer between 0 to 11)
    OutsideOctave,
    /// The string was not recognized for key signatures
    InvalidKeyString,
}

#[derive(Debug)]
pub enum LilypondAccidentalError {
    /// The string was not recognized for accidentals
    InvalidAccidentalString,
}
