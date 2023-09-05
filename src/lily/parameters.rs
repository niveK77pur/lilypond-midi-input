use std::collections::HashMap;

use super::{LilyAccidental, LilyKeySignature};

type Alteration = HashMap<u8, String>;

#[derive(Debug)]
pub struct LilyParameters {
    pub(super) key: LilyKeySignature,
    pub(super) accidentals: LilyAccidental,
    /// custom alterations within an octave (0-11)
    pub(super) alterations: Alteration,
    /// custom alterations over all notes
    pub(super) global_alterations: Alteration,
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
