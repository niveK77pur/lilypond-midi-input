type Alteration<'a> = (u8, &'a str);

#[derive(Debug)]
pub struct LilyParameters<'a> {
    key: LilyKeySignature,
    /// custom alterations within an octave (0-11)
    alterations: Vec<Alteration<'a>>,
    /// custom alterations over all notes
    global_alterations: Vec<Alteration<'a>>,
}

impl<'a> LilyParameters<'a> {
    pub fn new(
        key: LilyKeySignature,
        alterations: Vec<Alteration<'a>>,
        global_alterations: Vec<Alteration<'a>>,
    ) -> Self {
        LilyParameters {
            key,
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

    pub fn alterations(&self) -> &Vec<Alteration<'a>> {
        &self.alterations
    }
    pub fn set_alterations(&mut self, alterations: Vec<Alteration<'a>>) {
        self.alterations = alterations
    }
    pub fn global_alterations(&self) -> &Vec<Alteration<'a>> {
        &self.global_alterations
    }
    pub fn set_global_alterations(&mut self, global_alterations: Vec<Alteration<'a>>) {
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

#[derive(Debug)]
pub struct LilyNote {
    /// the LilyPond note string
    letter: &'static str,
    /// octave indication is very small, we do not need a large integer
    octave: i8,
}

impl LilyNote {
    pub fn new(value: u8, parameters: &LilyParameters) -> Self {
        LilyNote {
            letter: Self::note_name(value % 12, &parameters).expect("Note within octave"),
            octave: (value as i16 / 12) as i8 - 4,
        }
    }

    fn note_name(note: u8, parameters: &LilyParameters) -> Result<&'static str, LilypondNoteError> {
        let LilyParameters { key, .. } = parameters;
        use LilyKeySignature::*;
        match note {
            0 => match key {
                CSharpMajor => Ok("bis"),
                CSharpMinor => Ok("bis"),
                _ => Ok("c"),
            },
            1 => match key {
                AFlatMajor | DFlatMajor | GFlatMajor | CFlatMajor => Ok("des"),
                DMajor | AMajor | EMajor | BMajor | FSharpMajor | CSharpMajor => Ok("cis"),
                DMinor => Ok("cis"),
                _ => Ok("cis"),
            },
            2 => match key {
                EFlatMinor => Ok("d"),
                DSharpMinor => Ok("cisis"),
                _ => Ok("d"),
            },
            3 => match key {
                BFlatMajor | EFlatMajor | AFlatMajor | DFlatMajor | GFlatMajor | CFlatMajor => {
                    Ok("ees")
                }
                EMajor | BMajor | FSharpMajor | CSharpMajor => Ok("dis"),
                EMinor => Ok("dis"),
                _ => Ok("dis"),
            },
            4 => match key {
                CFlatMajor => Ok("fes"),
                FMinor => Ok("e"),
                _ => Ok("e"),
            },
            5 => match key {
                FSharpMajor | CSharpMajor => Ok("eis"),
                FSharpMinor => Ok("eis"),
                _ => Ok("f"),
            },
            6 => match key {
                DFlatMajor | GFlatMajor | CFlatMajor => Ok("ges"),
                GMajor | DMajor | AMajor | EMajor | BMajor | FSharpMajor | CSharpMajor => Ok("fis"),
                GMinor => Ok("fis"),
                _ => Ok("fis"),
            },
            7 => match key {
                AFlatMinor => Ok("g"),
                GSharpMinor => Ok("fisis"),
                _ => Ok("g"),
            },
            8 => match key {
                EFlatMajor | AFlatMajor | DFlatMajor | GFlatMajor | CFlatMajor => Ok("aes"),
                AMajor | EMajor | BMajor | FSharpMajor | CSharpMajor => Ok("gis"),
                AMinor => Ok("gis"),
                _ => Ok("gis"),
            },
            9 => match key {
                BFlatMinor => Ok("a"),
                ASharpMinor => Ok("gisis"),
                _ => Ok("a"),
            },
            10 => match key {
                FMajor | BFlatMajor | EFlatMajor | AFlatMajor | DFlatMajor | GFlatMajor
                | CFlatMajor => Ok("bes"),
                BMajor | FSharpMajor | CSharpMajor => Ok("ais"),
                BMinor => Ok("ais"),
                _ => Ok("ais"),
            },
            11 => match key {
                GFlatMajor | CFlatMajor => Ok("ces"),
                CMinor => Ok("b"),
                _ => Ok("b"),
            },
            _ => Err(LilypondNoteError::OutsideOctave),
        }
    }
}

#[derive(Debug)]
pub enum LilypondNoteError {
    /// Some functions require the note to be within an octave (integer between 0 to 11)
    OutsideOctave,
    ///
    InvalidKeyString,
}
