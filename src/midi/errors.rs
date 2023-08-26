use portmidi::MidiMessage;

/// Errors related to MIDI devices
#[derive(Debug)]
pub enum LilypondMidiDeviceError {
    NamedDeviceNotFound(String),
}

/// Errors related to MIDI messages
#[derive(Debug)]
pub enum LilypondMidiMessageError {
    UnknownMidiMessageType(MidiMessage),
}
