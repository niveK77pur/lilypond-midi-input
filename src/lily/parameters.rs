use std::collections::{BTreeSet, HashMap};

use getset::{Getters, Setters};

use crate::{InputMode, MidiNote};

use super::{Language, LilyAccidental, LilyKeySignature, LilyNote, LilypondNoteError, OctaveEntry};

type Alteration = HashMap<MidiNote, String>;

#[derive(Debug, Getters, Setters)]
pub struct LilyParameters {
    #[getset(get = "pub", set = "pub")]
    pub(super) key: LilyKeySignature,
    #[getset(get = "pub", set = "pub")]
    pub(super) accidentals: LilyAccidental,
    #[getset(get = "pub", set = "pub")]
    pub(super) mode: InputMode,
    #[getset(get = "pub", set = "pub")]
    pub(super) language: Language,
    #[getset(get = "pub", set = "pub")]
    pub(super) octave_entry: OctaveEntry,
    /// control adding of octave check on the next generated note
    #[getset(get = "pub", set = "pub")]
    pub(super) octave_check_on_next_note: bool,
    /// control adding of octave checks on all generated notes
    #[getset(get = "pub", set = "pub")]
    pub(super) octave_check_notes: bool,
    /// custom alterations within an octave (0-11)
    #[getset(get = "pub")]
    pub(super) alterations: Alteration,
    /// custom alterations over all notes
    #[getset(get = "pub", set = "pub")]
    pub(super) global_alterations: Alteration,
    /// manually set the previous chord for generating a 'q' shorthand
    #[getset(set = "pub")]
    pub(super) previous_chord: Option<BTreeSet<MidiNote>>,
    /// the previous note in absolute pitch used to calculate the next note in relative pitch
    #[getset(set = "pub")]
    pub(super) previous_absolute_note_reference: Option<MidiNote>,
}

impl LilyParameters {
    pub fn new(
        key: LilyKeySignature,
        accidentals: LilyAccidental,
        mode: InputMode,
        language: Language,
        octave_entry: OctaveEntry,
        octave_check_on_next_note: bool,
        octave_check_notes: bool,
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
            language,
            octave_entry,
            octave_check_on_next_note,
            octave_check_notes,
            alterations,
            global_alterations,
            previous_chord: None,
            previous_absolute_note_reference: None,
        })
    }

    pub fn set_alterations(
        &mut self,
        alterations: Alteration,
    ) -> Result<&mut Self, LilypondNoteError> {
        for alt in &alterations {
            Self::verify_alteration(alt.0)?;
        }
        self.alterations = alterations;
        Ok(self)
    }
    pub fn add_alteration(
        &mut self,
        note: MidiNote,
        value: String,
    ) -> Result<&mut Self, LilypondNoteError> {
        Self::verify_alteration(&note)?;
        self.alterations.insert(note, value);
        Ok(self)
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
    pub fn set_previous_chord_lilypond_str(
        &mut self,
        previous_chord: Vec<String>,
    ) -> Result<&mut Self, LilypondNoteError> {
        let mut chord = BTreeSet::new();
        for note in previous_chord.into_iter() {
            chord.insert(*LilyNote::from_lilypond_str(note.as_str())?.note());
        }
        self.previous_chord = Some(chord);
        Ok(self)
    }
    pub fn previous_absolute_note_reference(&mut self) -> Option<&MidiNote> {
        self.previous_absolute_note_reference.as_ref()
    }
    pub fn set_previous_absolute_note_reference_lilypond_str(
        &mut self,
        previous_absolute_note_reference: String,
    ) -> Result<&mut Self, LilypondNoteError> {
        self.previous_absolute_note_reference =
            Some(*LilyNote::from_lilypond_str(&previous_absolute_note_reference)?.note());
        Ok(self)
    }
}

#[derive(Debug)]
pub enum LilyParametersError {
    NoteError(LilypondNoteError),
}
