use std::collections::{BTreeSet, HashMap};

use crate::{InputMode, MidiNote};

use super::{LilyAccidental, LilyKeySignature, LilyNote, LilypondNoteError};

type Alteration = HashMap<MidiNote, String>;

#[derive(Debug)]
pub struct LilyParameters {
    pub(super) key: LilyKeySignature,
    pub(super) accidentals: LilyAccidental,
    pub(super) mode: InputMode,
    /// custom alterations within an octave (0-11)
    pub(super) alterations: Alteration,
    /// custom alterations over all notes
    pub(super) global_alterations: Alteration,
    /// manually set the previous chord for generating a 'q' shorthand
    pub(super) previous_chord: Option<BTreeSet<MidiNote>>,
}

impl LilyParameters {
    pub fn new(
        key: LilyKeySignature,
        accidentals: LilyAccidental,
        mode: InputMode,
        alterations: Alteration,
        global_alterations: Alteration,
    ) -> Result<Self, LilyParametersError> {
        for alt in &alterations {
            if let Err(error) = Self::verify_alteration(alt.0) {
                return Err(LilyParametersError::NoteError(error));
            }
        }
        Ok(LilyParameters {
            key,
            accidentals,
            mode,
            alterations,
            global_alterations,
            previous_chord: None,
        })
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
    pub fn mode(&self) -> &InputMode {
        &self.mode
    }
    pub fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode
    }
    pub fn alterations(&self) -> &Alteration {
        &self.alterations
    }
    pub fn set_alterations(&mut self, alterations: Alteration) -> Result<(), LilypondNoteError> {
        for alt in &alterations {
            Self::verify_alteration(alt.0)?;
        }
        self.alterations = alterations;
        Ok(())
    }
    pub fn add_alteration(
        &mut self,
        note: MidiNote,
        value: String,
    ) -> Result<(), LilypondNoteError> {
        Self::verify_alteration(&note)?;
        self.alterations.insert(note, value);
        Ok(())
    }
    pub fn clear_alterations(&mut self) {
        self.set_alterations(HashMap::new())
            .expect("Empty map has no invalid alterations");
    }
    /// Verify if the alteration is within an octave
    ///
    /// # Errors
    ///
    /// This function will return an error if the note is a value outside of an
    /// octave (i.e. outside of the range 0 to 11 inclusive)
    pub fn verify_alteration(note: &MidiNote) -> Result<(), LilypondNoteError> {
        if (&0..=&11).contains(&note) {
            Ok(())
        } else {
            Err(LilypondNoteError::OutsideOctave(*note))
        }
    }
    pub fn global_alterations(&self) -> &Alteration {
        &self.global_alterations
    }
    pub fn set_global_alterations(&mut self, global_alterations: Alteration) {
        self.global_alterations = global_alterations
    }
    pub fn add_global_alteration(&mut self, note: MidiNote, value: String) {
        self.global_alterations.insert(note, value);
    }
    pub fn clear_global_alterations(&mut self) {
        self.set_global_alterations(HashMap::new());
    }
    pub fn take_previous_chord(&mut self) -> Option<BTreeSet<MidiNote>> {
        self.previous_chord.take()
    }
    pub fn previous_chord(&mut self) -> Option<&BTreeSet<MidiNote>> {
        self.previous_chord.as_ref()
    }
    pub fn set_previous_chord(
        &mut self,
        previous_chord: Vec<String>,
    ) -> Result<(), LilypondNoteError> {
        let mut chord = BTreeSet::new();
        for note in previous_chord.into_iter() {
            chord.insert(*LilyNote::from_lilypond_str(note.as_str())?.note());
        }
        self.previous_chord = Some(chord);
        Ok(())
    }
}

#[derive(Debug)]
pub enum LilyParametersError {
    NoteError(LilypondNoteError),
}
