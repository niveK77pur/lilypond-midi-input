use lilypond_midi_input as lmi;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let lilypond_midi_input_handler = std::thread::spawn(move || {
        // initialize the PortMidi context.
        let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");
        let name = "USB-MIDI MIDI 1";

        lmi::list_devices(&context);

        let port = lmi::MidiInputPort::new(name, &context, BUFFER_SIZE)
            .expect("Port name matches an existing port");

        port.clear();

        port.listen(|event| {
            // TODO: buffer unreceived lines
            let message = rx.try_recv();
            println!("{:?} ({:?})", lmi::MidiMessageType::from(event), message)
        }).expect("Polling for new messages works.");
    });

    let user_input_handler = std::thread::spawn(move || {
        for line in std::io::stdin().lines() {
            println!(">> GOT LINE: {:?}", line);
            tx.send(line).expect("Receiver is alive");
        }
    });

    match lilypond_midi_input_handler.join() {
        Ok(_) => println!("Lilypond MIDI input handling thread finished."),
        Err(e) => panic!("Lilypond MIDI input handling panicked: {:#?}", e),
    };
}
