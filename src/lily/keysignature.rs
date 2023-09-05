use super::LilypondNoteError;

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
