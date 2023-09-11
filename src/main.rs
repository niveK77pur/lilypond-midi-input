use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex},
};

use clap::{arg, command, value_parser, ArgAction};
use lilypond_midi_input::{
    echoerr, echoinfo,
    lily::{self, LilyAccidental, LilyKeySignature},
    midi::{self, list_input_devices},
    output, InputMode, ListOptions, MidiNote,
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
            arg!(--"list-options" <argument> "List available options for a given argument")
                .exclusive(true)
                .action(ArgAction::Set)
                .value_parser(["key", "accidentals", "mode"]),
        ])
        .get_matches();
    let re_keyval =
        Regex::new(r"(?<key>[[:alnum:]-]+)=(?<value>[^[:space:]]+)").expect("Regex is valid");
    let re_subkeyval =
        Regex::new(r"(?<key>[[:alnum:]-]+):(?<value>[^,]+)").expect("Regex is valid");

    // initialize the PortMidi context.
    let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");

    if *matches.get_one::<bool>("list-devices").unwrap_or(&false) {
        list_input_devices(&context);
        return;
    } else if let Some(arg) = matches.get_one::<String>("list-options") {
        match arg.as_str() {
            "key" => LilyKeySignature::list_options(),
            "accidentals" => LilyAccidental::list_options(),
            "mode" => InputMode::list_options(),
            _ => echoerr!("Invalid argument specified for listing."),
        }
        return;
    }

    let lily_parameters: Arc<Mutex<lily::LilyParameters>> = Arc::new(Mutex::new(
        match lily::LilyParameters::new(
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
        ) {
            Ok(p) => p,
            Err(e) => {
                echoerr!("An invalid parameter was given: {:?}", e);
                return;
            }
        },
    ));

    let parameters = Arc::clone(&lily_parameters);
    let lilypond_midi_input_handler = std::thread::spawn(move || {
        let name = matches
            .get_one::<String>("DEVICE")
            .expect("Device was given");

        let port = match midi::MidiInputPort::new(name, &context, BUFFER_SIZE) {
            Ok(p) => p,
            Err(e) => {
                echoerr!("Given port name does not exist: {:?}", e);
                return;
            }
        };

        port.clear();

        // track notes to be put into a chord
        let mut notes: BTreeSet<MidiNote> = BTreeSet::new();
        // track notes being pressed to know when everything was released
        let mut pressed: BTreeSet<MidiNote> = BTreeSet::new();
        // track pedals being pressed to know when everything was released
        let mut pedals: BTreeSet<MidiNote> = BTreeSet::new();
        // track last chord inserted (to insert a 'q' on repetition)
        let mut last_chord: Option<BTreeSet<MidiNote>> = None;
        port.listen_mut(|event| {
            let params = parameters.lock().expect("Received the mutex lock");
            let use_chords: bool = match params.mode() {
                InputMode::Single => false,
                InputMode::Chord => true,
                InputMode::PedalChord => !pedals.is_empty(),
                InputMode::PedalSingle => pedals.is_empty(),
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
                                output!("{lilynote}")
                            }
                            std::cmp::Ordering::Greater => {
                                let chord: String = notes
                                    .iter()
                                    .map(|note| lily::LilyNote::new(*note, &params).to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ");
                                match last_chord.as_ref() == Some(&notes) {
                                    true => output!("q"),
                                    false => {
                                        output!("<{}>", chord);
                                        last_chord = Some(notes.clone());
                                    }
                                }
                                notes.clear();
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
                        output!("{lilynote}")
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
                        Ok(v) => {
                            echoinfo!("Update key={:?}", v);
                            v
                        }
                        Err(e) => match e {
                            lily::LilypondNoteError::OutsideOctave(_) => {
                                panic!("This error will not occur here.")
                            }
                            lily::LilypondNoteError::InvalidKeyString(key) => {
                                echoerr!("Invalid key provided: {key}");
                                continue;
                            }
                        },
                    }),
                    "accidentals" | "a" => params.set_accidentals(match value.try_into() {
                        Ok(v) => {
                            echoinfo!("Update accidentals={:?}", v);
                            v
                        }
                        Err(e) => match e {
                            lily::LilypondAccidentalError::InvalidAccidentalString(a) => {
                                echoerr!("Invalid accidental provided: {a}");
                                continue;
                            }
                        },
                    }),
                    "mode" | "m" => params.set_mode(match value.try_into() {
                        Ok(m) => {
                            echoinfo!("Update mode={:?}", m);
                            m
                        }
                        Err(e) => match e {
                            lilypond_midi_input::InputModeError::InvalidModeString(mode) => {
                                echoerr!("Invalid mode provided: {mode}");
                                continue;
                            }
                        },
                    }),
                    "alterations" | "alt" => match value {
                        "clear" => {
                            params.clear_alterations();
                            echoinfo!("Cleared all alterations");
                        }
                        _ => match parse_subkeys(&re_subkeyval, value) {
                            Some(alts) => {
                                if alts.is_empty() {
                                    echoinfo!("No alterations were parsed/given");
                                }
                                for alt in alts {
                                    let (note, value) = alt;
                                    match params.add_alteration(note, value.clone()) {
                                        Ok(_) => {
                                            echoinfo!("Update alteration={:?}:{:?}", note, value);
                                        }
                                        Err(e) => {
                                            echoerr!("Invalid alteration was given: {:?}", e)
                                        }
                                    };
                                }
                            }
                            None => echoerr!("One of the keys is not a number"),
                        },
                    },
                    "global-alterations" | "galt" => match value {
                        "clear" => {
                            params.clear_global_alterations();
                            echoinfo!("Cleared all global alterations");
                        }
                        _ => match parse_subkeys(&re_subkeyval, value) {
                            Some(galts) => {
                                if galts.is_empty() {
                                    echoinfo!("No global alterations were parsed/given");
                                }
                                for galt in galts {
                                    let (note, value) = galt;
                                    eprintln!(">> Update global-alteration={:?}:{:?}", note, value);
                                    params.add_global_alteration(note, value);
                                }
                            }
                            None => echoerr!("One of the keys is not a number"),
                        },
                    },
                    _ => echoerr!("An invalid/unknown key was specified: {key}"),
                }
            }
        }
    });

    match lilypond_midi_input_handler.join() {
        Ok(_) => echoinfo!("Lilypond MIDI input handling thread finished."),
        Err(e) => panic!("Lilypond MIDI input handling panicked: {:#?}", e),
    };
}

/// Parse subkeys for an input argument
///
/// Returns a vector of (`note,` `value`), where the `note` is a number and the
/// `value` is an arbitrary string with which to replace said `note`.
///
/// If any of the given `note`s cannot be parsed into a [MidiNote], then the
/// function will return `None`.
fn parse_subkeys(regex: &Regex, s: &str) -> Option<Vec<(MidiNote, String)>> {
    let mut result = Vec::new();
    for subcap in regex.captures_iter(s) {
        let subkey: MidiNote = match subcap
            .name("key")
            .expect("Valid named group")
            .as_str()
            .parse()
        {
            Ok(n) => n,
            Err(_) => {
                echoerr!(
                    "Key is not an unsigned number: {}",
                    subcap.name("key").unwrap().as_str()
                );
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
