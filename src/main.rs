use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
};

use lilypond_midi_input::midi;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let (tx, rx) = mpsc::channel();
    let channel_message_buffer = Arc::new(Mutex::new(VecDeque::new()));

    let message_buffer = Arc::clone(&channel_message_buffer);
    let lilypond_midi_input_handler = std::thread::spawn(move || {
        // initialize the PortMidi context.
        let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");
        let name = "out"; // let name = "USB-MIDI MIDI 1";

        midi::list_devices(&context);

        let port = midi::MidiInputPort::new(name, &context, BUFFER_SIZE)
            .expect("Port name matches an existing port");

        port.clear();

        port.listen(|event| {
            if rx.try_recv().is_ok() {
                while let Some(message) = message_buffer
                    .lock()
                    .expect("Received the mutex lock")
                    .pop_front()
                {
                    println!("MESSAGE RECEIVED: {:?}", message);
                }
            }
            println!("{:?}", midi::MidiMessageType::from(event),)
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
