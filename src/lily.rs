use std::collections::HashMap;

type Alteration<'a> = HashMap<u8, &'a str>;

#[derive(Debug)]
pub struct LilyParameters<'a> {
    key: LilyKeySignature,
    accidentals: LilyAccidental,
    /// custom alterations within an octave (0-11)
    alterations: Alteration<'a>,
    /// custom alterations over all notes
    global_alterations: Alteration<'a>,
}

impl<'a> LilyParameters<'a> {
    pub fn new(
        key: LilyKeySignature,
        accidentals: LilyAccidental,
        alterations: Alteration<'a>,
        global_alterations: Alteration<'a>,
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

    pub fn alterations(&self) -> &Alteration<'a> {
        &self.alterations
    }
    pub fn set_alterations(&mut self, alterations: Alteration<'a>) {
        self.alterations = alterations
    }
    pub fn global_alterations(&self) -> &Alteration<'a> {
        &self.global_alterations
    }
    pub fn set_global_alterations(&mut self, global_alterations: Alteration<'a>) {
        self.global_alterations = global_alterations
    }
}

#[derive(Debug)]
/// List of possible musical key signatures
pub enum LilyKeySignature {
    CFlatMajor,  // 7 flats
    GFlatMajor,  // 6 flats
    DFlatMajor,  // 5 flats
    AFlatMajor,  // 4 flats
    EFlatMajor,  // 3 flats
    BFlatMajor,  // 2 flats
    FMajor,      // 1 flat
    CMajor,      // 0 flats/sharps
    GMajor,      // 1 sharp
    DMajor,      // 2 sharps
    AMajor,      // 3 sharps
    EMajor,      // 4 sharps
    BMajor,      // 5 sharps
    FSharpMajor, // 6 sharps
    CSharpMajor, // 7 sharps
    AFlatMinor,  // 7 flats
    EFlatMinor,  // 6 flats
    BFlatMinor,  // 5 flats
    FMinor,      // 4 flats
    CMinor,      // 3 flats
    GMinor,      // 2 flats
    DMinor,      // 1 flat
    AMinor,      // 0 flats/sharps
    EMinor,      // 1 sharp
    BMinor,      // 2 sharps
    FSharpMinor, // 3 sharps
    CSharpMinor, // 4 sharps
    GSharpMinor, // 5 sharps
    DSharpMinor, // 6 sharps
    ASharpMinor, // 7 sharps
}

impl TryFrom<&str> for LilyKeySignature {
    type Error = LilypondNoteError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cesM" => Ok(LilyKeySignature::CFlatMajor),
            "gesM" => Ok(LilyKeySignature::GFlatMajor),
            "desM" => Ok(LilyKeySignature::DFlatMajor),
            "aesM" => Ok(LilyKeySignature::AFlatMajor),
            "eesM" => Ok(LilyKeySignature::EFlatMajor),
            "besM" => Ok(LilyKeySignature::BFlatMajor),
            "fM" => Ok(LilyKeySignature::FMajor),
            "cM" => Ok(LilyKeySignature::CMajor),
            "gM" => Ok(LilyKeySignature::GMajor),
            "dM" => Ok(LilyKeySignature::DMajor),
            "aM" => Ok(LilyKeySignature::AMajor),
            "eM" => Ok(LilyKeySignature::EMajor),
            "bM" => Ok(LilyKeySignature::BMajor),
            "fisM" => Ok(LilyKeySignature::FSharpMajor),
            "cisM" => Ok(LilyKeySignature::CSharpMajor),
            "dm" => Ok(LilyKeySignature::AFlatMinor),
            "gm" => Ok(LilyKeySignature::EFlatMinor),
            "cm" => Ok(LilyKeySignature::BFlatMinor),
            "fm" => Ok(LilyKeySignature::FMinor),
            "besm" => Ok(LilyKeySignature::CMinor),
            "eesm" => Ok(LilyKeySignature::GMinor),
            "aesm" => Ok(LilyKeySignature::DMinor),
            "am" => Ok(LilyKeySignature::AMinor),
            "em" => Ok(LilyKeySignature::EMinor),
            "bm" => Ok(LilyKeySignature::BMinor),
            "fism" => Ok(LilyKeySignature::FSharpMinor),
            "cism" => Ok(LilyKeySignature::CSharpMinor),
            "gism" => Ok(LilyKeySignature::GSharpMinor),
            "dism" => Ok(LilyKeySignature::DSharpMinor),
            "aism" => Ok(LilyKeySignature::ASharpMinor),
            _ => Err(LilypondNoteError::InvalidKeyString),
        }
    }
}

/// The accidentals to use for out of key notes.
#[derive(Debug)]
pub enum LilyAccidental {
    Sharps,
    Flats,
}

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
    ///
    InvalidKeyString,
}
