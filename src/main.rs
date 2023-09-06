use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex},
};

use clap::{arg, command, value_parser, ArgAction};
use lilypond_midi_input::{
    lily::{self, LilyAccidental, LilyKeySignature},
    midi::{self, list_input_devices},
    InputMode,
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
            arg!(-m --mode "Input mode to use")
                .action(ArgAction::Set)
                .value_parser(value_parser!(InputMode))
                .default_value("single"),
            arg!(--alterations "Custom alterations within an octave").action(ArgAction::Set),
            arg!(--"global-alterations" <alterations> "Global alterations over all notes")
                .action(ArgAction::Set),
        ])
        .args([
            arg!(-l --"list-devices" "List available MIDI input devices").exclusive(true),
            // arg!(--"list-keys" "List available musical keys").exclusive(true),
        ])
        .get_matches();
    let re_keyval = Regex::new(r"(?<key>\w+)=(?<value>[^[:space:]]+)").expect("Regex is valid");
    let re_subkeyval = Regex::new(r"(?<key>\w+):(?<value>[^,]+)").expect("Regex is valid");

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
        matches
            .get_one::<InputMode>("mode")
            .expect("accidental style is given and valid")
            .clone(),
        match matches.get_one::<String>("alterations") {
            Some(alts) => {
                let mut result = HashMap::new();
                for alt in
                    parse_subkeys(&re_subkeyval, alts).expect("All of the subkeys are numbers")
                {
                    let (note, value) = alt;
                    result.insert(note, value);
                }
                result
            }
            None => HashMap::new(),
        },
        match matches.get_one::<String>("global-alterations") {
            Some(alts) => {
                let mut result = HashMap::new();
                for alt in
                    parse_subkeys(&re_subkeyval, alts).expect("All of the subkeys are numbers")
                {
                    let (note, value) = alt;
                    result.insert(note, value);
                }
                result
            }
            None => HashMap::new(),
        },
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

        let mode = InputMode::Pedal;
        // track notes to be put into a chord
        let mut notes: BTreeSet<u8> = BTreeSet::new();
        // track notes being pressed to know when everything was released
        let mut pressed: BTreeSet<u8> = BTreeSet::new();
        // track pedals being pressed to know when everything was released
        let mut pedals: BTreeSet<u8> = BTreeSet::new();
        port.listen_mut(|event| {
            let use_chords: bool = match mode {
                InputMode::Single => false,
                InputMode::Chord => true,
                InputMode::Pedal => !pedals.is_empty(),
            };
            match midi::MidiMessageType::from(event) {
                midi::MidiMessageType::NoteOn { note, .. } => {
                    pressed.insert(note);
                    notes.insert(note);
                }
                midi::MidiMessageType::NoteOff { note, .. } => {
                    pressed.remove(&note);
                }
                midi::MidiMessageType::PedalOn { pedal, .. } => {
                    pedals.insert(pedal);
                    return;
                }
                midi::MidiMessageType::PedalOff { pedal } => {
                    pedals.remove(&pedal);
                    return;
                }
                midi::MidiMessageType::Unknown => todo!(),
            }
            dbg!(&notes, &pressed, &pedals);
            let params = parameters.lock().expect("Received the mutex lock");
            match use_chords {
                true => {
                    if pressed.is_empty() {
                        match notes.len().cmp(&1) {
                            std::cmp::Ordering::Less => (),
                            std::cmp::Ordering::Equal => {
                                let lilynote = lily::LilyNote::new(
                                    notes.pop_first().expect("A note was pressed"),
                                    &params,
                                );
                                println!("{lilynote}")
                            }
                            std::cmp::Ordering::Greater => {
                                let chord: String = notes
                                    .iter()
                                    .map(|note| lily::LilyNote::new(*note, &params).to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ");
                                notes.clear();
                                println!("<{}>", chord);
                            }
                        }
                    }
                }
                false => {
                    if !notes.is_empty() {
                        let lilynote = lily::LilyNote::new(
                            notes.pop_first().expect("A note was pressed"),
                            &params,
                        );
                        println!("{lilynote}")
                    }
                }
            }
        })
        .expect("Polling for new messages works.");
    });

    let parameters = Arc::clone(&lily_parameters);
    let _user_input_handler = std::thread::spawn(move || {
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
                        _ => match parse_subkeys(&re_subkeyval, value) {
                            Some(alts) => {
                                for alt in alts {
                                    let (note, value) = alt;
                                    params.add_alteration(note, value);
                                }
                            }
                            None => eprintln!("One of the keys is not a number"),
                        },
                    },
                    "global-alterations" | "galt" => match value {
                        "clear" => params.clear_global_alterations(),
                        _ => match parse_subkeys(&re_subkeyval, value) {
                            Some(galts) => {
                                for galt in galts {
                                    let (note, value) = galt;
                                    params.add_global_alteration(note, value);
                                }
                            }
                            None => eprintln!("One of the keys is not a number"),
                        },
                    },
                    _ => eprintln!("An invalid/unknown key was specified"),
                }
            }
        }
    });

    match lilypond_midi_input_handler.join() {
        Ok(_) => eprintln!("Lilypond MIDI input handling thread finished."),
        Err(e) => panic!("Lilypond MIDI input handling panicked: {:#?}", e),
    };
}

/// Parse subkeys for an input argument
///
/// Returns a vector of (`note,` `value`), where the `note` is a number and the
/// `value` is an arbitrary string with which to replace said `note`.
///
/// If any of the given `note`s cannot be parsed into a [u8], then the function
/// will return `None`.
fn parse_subkeys(regex: &Regex, s: &str) -> Option<Vec<(u8, String)>> {
    let mut result = Vec::new();
    for subcap in regex.captures_iter(s) {
        let subkey: u8 = match subcap
            .name("key")
            .expect("Valid named group")
            .as_str()
            .parse()
        {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Key is not a number: {e}");
                return None;
            }
        };
        let subvalue = subcap
            .name("value")
            .expect("Valid named group")
            .as_str()
            .into();
        result.push((subkey, subvalue))
    }
    Some(result)
}
