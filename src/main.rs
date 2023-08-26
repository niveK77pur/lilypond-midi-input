use std::{
    collections::{HashMap, VecDeque},
    sync::{mpsc, Arc, Mutex},
};

use lilypond_midi_input::{lily, midi};

const BUFFER_SIZE: usize = 1024;

fn main() {
    let (tx, rx) = mpsc::channel();
    let channel_message_buffer: Arc<Mutex<VecDeque<String>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    let message_buffer = Arc::clone(&channel_message_buffer);
    let lilypond_midi_input_handler = std::thread::spawn(move || {
        // initialize the PortMidi context.
        let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");
        let name = "out"; // let name = "USB-MIDI MIDI 1";

        midi::list_devices(&context);

        let port = midi::MidiInputPort::new(name, &context, BUFFER_SIZE)
            .expect("Port name matches an existing port");

        port.clear();

        let mut alterations = HashMap::new();
        alterations.insert(0, "hello");
        alterations.insert(10, "world");

        let mut global_alterations = HashMap::new();
        global_alterations.insert(48, "HELLO");
        global_alterations.insert(50, "BYE");

        let mut parameters = lily::LilyParameters::new(
            lily::LilyKeySignature::GMajor,
            lily::LilyAccidental::Flats,
            alterations,
            global_alterations,
        );

        port.listen_mut(|event| {
            if rx.try_recv().is_ok() {
                while let Some(message) = message_buffer
                    .lock()
                    .expect("Received the mutex lock")
                    .pop_front()
                {
                    match message.as_str().try_into() {
                        Ok(key) => {
                            parameters.set_key(key);
                            println!("PARAMETER SET: {:?}", parameters);
                        }
                        Err(e) => println!("ERROR! {:?}", e),
                    }
                }
            }
            if let midi::MidiMessageType::NoteOn { note, .. } = midi::MidiMessageType::from(event) {
                println!("{:?} {:?}", lily::LilyNote::new(note, &parameters), event)
            }
        })
        .expect("Polling for new messages works.");
    });

    let message_buffer = Arc::clone(&channel_message_buffer);
    let _user_input_handler = std::thread::spawn(move || {
        for line in std::io::stdin()
            .lines()
            .map(|l| l.expect("Managed to read stdin line"))
        {
            message_buffer
                .lock()
                .expect("Received the mutex lock")
                .push_back(line);
            tx.send(()).expect("Receiver is alive");
        }
    });

    match lilypond_midi_input_handler.join() {
        Ok(_) => println!("Lilypond MIDI input handling thread finished."),
        Err(e) => panic!("Lilypond MIDI input handling panicked: {:#?}", e),
    };
}
