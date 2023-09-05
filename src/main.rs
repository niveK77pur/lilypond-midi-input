use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use clap::{arg, command, value_parser, ArgAction};
use lilypond_midi_input::{
    lily::{self, LilyAccidental, LilyKeySignature},
    midi::{self, list_input_devices},
};
use regex::Regex;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .next_line_help(false)
        .args([
            arg!(<DEVICE> "MIDI Input Device"),
            arg!(-k --key "Specify musical key")
                .action(ArgAction::Set)
                .value_parser(value_parser!(LilyKeySignature))
                .default_value("cM"),
            arg!(-a --accidentals "Accidental style to use for out-of-key notes")
                .action(ArgAction::Set)
                .value_parser(value_parser!(LilyAccidental))
                .default_value("sharps"),
            arg!(--alterations "Custom alterations within an octave").action(ArgAction::Set),
            arg!(--"global-alterations" <alterations> "Global alterations over all notes")
                .action(ArgAction::Set),
        ])
        .args([
            arg!(-l --"list-devices" "List available MIDI input devices").exclusive(true),
            // arg!(--"list-keys" "List available musical keys").exclusive(true),
        ])
        .get_matches();

    // initialize the PortMidi context.
    let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");

    if *matches.get_one::<bool>("list-devices").unwrap_or(&false) {
        list_input_devices(&context);
        return;
    }

    let lily_parameters = Arc::new(Mutex::new(lily::LilyParameters::new(
        matches
            .get_one::<LilyKeySignature>("key")
            .expect("key is given and valid")
            .clone(),
        matches
            .get_one::<LilyAccidental>("accidentals")
            .expect("accidental style is given and valid")
            .clone(),
        HashMap::new(),
        HashMap::new(),
    )));

    let parameters = Arc::clone(&lily_parameters);
    let lilypond_midi_input_handler = std::thread::spawn(move || {
        let name = matches
            .get_one::<String>("DEVICE")
            .expect("Device was given");

        midi::list_input_devices(&context);

        let port = midi::MidiInputPort::new(name, &context, BUFFER_SIZE)
            .expect("Port name matches an existing port");

        port.clear();

        port.listen_mut(|event| {
            if let midi::MidiMessageType::NoteOn { note, .. } = midi::MidiMessageType::from(event) {
                let params = parameters.lock().expect("Received the mutex lock");
                let lilynote = lily::LilyNote::new(note, &params);
                println!("{} {:?} {:?}", String::from(&lilynote), lilynote, event)
            }
        })
        .expect("Polling for new messages works.");
    });

    let parameters = Arc::clone(&lily_parameters);
    let _user_input_handler = std::thread::spawn(move || {
        let re_keyval = Regex::new(r"(?<key>\w+)=(?<value>[^[:space:]]+)").expect("Regex is valid");
        let re_subkeyval = Regex::new(r"(?<key>\w+):(?<value>[^,]+)").expect("Regex is valid");
        for line in std::io::stdin()
            .lines()
            .map(|l| l.expect("Managed to read stdin line"))
        {
            let mut params = parameters.lock().expect("Received the mutex lock");
            for cap in re_keyval.captures_iter(line.as_str()) {
                let key = cap.name("key").expect("Valid named group").as_str();
                let value = cap.name("value").expect("Valid named group").as_str();
                match key {
                    "key" | "k" => params.set_key(match value.try_into() {
                        Ok(v) => v,
                        Err(e) => match e {
                            lily::LilypondNoteError::OutsideOctave => {
                                panic!("This error will not occur here.")
                            }
                            lily::LilypondNoteError::InvalidKeyString => {
                                eprintln!("Invalid key provided.");
                                continue;
                            }
                        },
                    }),
                    "accidentals" | "a" => params.set_accidentals(match value.try_into() {
                        Ok(v) => v,
                        Err(e) => match e {
                            lily::LilypondAccidentalError::InvalidAccidentalString => {
                                eprintln!("Invalid accidental provided");
                                continue;
                            }
                        },
                    }),
                    "alterations" | "alt" => match value {
                        "clear" => params.clear_alterations(),
                        _ => {
                            for subcap in re_subkeyval.captures_iter(value) {
                                let subkey: u8 = match subcap
                                    .name("key")
                                    .expect("Valid named group")
                                    .as_str()
                                    .parse()
                                {
                                    Ok(n) => n,
                                    Err(e) => {
                                        eprintln!("Key is not a number: {e}");
                                        continue;
                                    }
                                };
                                let subvalue = subcap
                                    .name("value")
                                    .expect("Valid named group")
                                    .as_str()
                                    .into();
                                eprintln!(">> subkey={:?} subvalue={:?}", subkey, subvalue);
                                params.add_alteration(subkey, subvalue);
                            }
                        }
                    },
                    "global-alterations" | "galt" => match value {
                        "clear" => params.clear_global_alterations(),
                        _ => {
                            for subcap in re_subkeyval.captures_iter(value) {
                                let subkey: u8 = match subcap
                                    .name("key")
                                    .expect("Valid named group")
                                    .as_str()
                                    .parse()
                                {
                                    Ok(n) => n,
                                    Err(e) => {
                                        eprintln!("Key is not a number: {e}");
                                        continue;
                                    }
                                };
                                let subvalue = subcap
                                    .name("value")
                                    .expect("Valid named group")
                                    .as_str()
                                    .into();
                                eprintln!(">> subkey={:?} subvalue={:?}", subkey, subvalue);
                                params.add_global_alteration(subkey, subvalue);
                            }
                        }
                    },
                    _ => todo!("match keys using args keys"),
                }
            }
        }
    });

    match lilypond_midi_input_handler.join() {
        Ok(_) => eprintln!("Lilypond MIDI input handling thread finished."),
        Err(e) => panic!("Lilypond MIDI input handling panicked: {:#?}", e),
    };
}
