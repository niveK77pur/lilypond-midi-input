use std::{
    collections::{HashMap, VecDeque},
    sync::{mpsc, Arc, Mutex},
};

use clap::Parser;
use lilypond_midi_input::{
    lily::{self, LilyAccidental},
    midi,
};
use regex::Regex;

const BUFFER_SIZE: usize = 1024;

#[derive(Parser, Debug)]
struct InputArgs {
    // #[arg(required(true))]
    device: String,
    #[arg(short, long)]
    list_devices: bool,
    #[arg(short, long, default_value_t = String::from("cM"))]
    key: String,
    #[arg(short, long, default_value_t = String::from("sharps"))]
    accidentals: String,
    #[arg(long, visible_alias("alt"), default_value_t = String::new())]
    alterations: String,
    #[arg(long, visible_alias("galt"), default_value_t = String::new())]
    global_alterations: String,
}

fn main() {
    let input_args = InputArgs::parse();

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
        global_alterations.insert(60, "HELLO");
        global_alterations.insert(62, "BYE");

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
                let lilynote = lily::LilyNote::new(note, &parameters);
                println!("{} {:?} {:?}", String::from(&lilynote), lilynote, event)
            }
        })
        .expect("Polling for new messages works.");
    });

    let message_buffer = Arc::clone(&channel_message_buffer);
    let _user_input_handler = std::thread::spawn(move || {
        let re_keyval = Regex::new(r"(?<key>\w+)=(?<value>[^[:space:]]+)").expect("Regex is valid");
        let re_subkeyval = Regex::new(r"(?<key>\w+):(?<value>[^,]+)").expect("Regex is valid");
        for line in std::io::stdin()
            .lines()
            .map(|l| l.expect("Managed to read stdin line"))
        {
            let mut commands: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
            for cap in re_keyval.captures_iter(line.as_str()) {
                let key = cap.name("key").expect("Valid named group").as_str();
                let value = cap.name("value").expect("Valid named group").as_str();
                let mut subvalues = HashMap::new();
                for subcap in re_subkeyval.captures_iter(value) {
                    subvalues.insert(
                        subcap.name("key").expect("Valid named group").as_str(),
                        subcap.name("value").expect("Valid named group").as_str(),
                    );
                }
                commands.insert(key, subvalues);
            }

            println!("{:?}", commands);
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
