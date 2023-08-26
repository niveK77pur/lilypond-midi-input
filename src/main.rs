use lilypond_midi_input as lmi;

const BUFFER_SIZE: usize = 1024;

fn main() {
    // initialize the PortMidi context.
    let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");
    let name = "USB-MIDI MIDI 1";

    lmi::list_devices(&context);

    let port = lmi::MidiInputPort::new(name, &context, BUFFER_SIZE)
        .expect("Port name matches an existing port");

    port.clear();

    port.listen(|event| println!("{:?}", lmi::MidiMessageType::from(event)))
        .expect("Polling for new messages works.");
}
