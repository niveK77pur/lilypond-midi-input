use portmidi::{MidiEvent, MidiMessage};

use crate::MidiNote;

/// Explicity see the type of MIDI message
#[derive(Debug)]
pub enum MidiMessageType {
    /// A note has been pressed
    NoteOn { note: MidiNote, velocity: u8 },
    /// A note has been released
    NoteOff { note: MidiNote, velocity: u8 },
    /// A piano pedal has been pressed
    PedalOn { pedal: MidiNote, value: u8 },
    /// A piano pedal has been released
    ///
    /// The `value` is omitted here, because it is back to 0 when the pedal was
    /// released.
    PedalOff { pedal: MidiNote },
    /// A midi message which has not been handled
    Unknown,
}

impl From<MidiMessage> for MidiMessageType {
    fn from(value: MidiMessage) -> Self {
        match value.status {
            144 => MidiMessageType::NoteOn {
                note: value.data1,
                velocity: value.data2,
            },
            128 => MidiMessageType::NoteOff {
                note: value.data1,
                velocity: value.data2,
            },
            176 => match value.data2.cmp(&0) {
                std::cmp::Ordering::Less => MidiMessageType::Unknown,
                std::cmp::Ordering::Equal => MidiMessageType::PedalOff { pedal: value.data1 },
                std::cmp::Ordering::Greater => MidiMessageType::PedalOn {
                    pedal: value.data1,
                    value: value.data2,
                },
            },
            _ => MidiMessageType::Unknown,
        }
    }
}

impl From<MidiEvent> for MidiMessageType {
    fn from(value: MidiEvent) -> Self {
        value.message.into()
    }
}
