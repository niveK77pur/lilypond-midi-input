use pm::{InputPort, PortMidi, PortMidiDeviceId};
use portmidi as pm;

use std::thread;
use std::time::Duration;

const BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
enum Error {
    NamedDeviceNotFound(String),
}

fn list_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn get_device_id_by_name(name: &str, pm: &PortMidi) -> Result<PortMidiDeviceId, Error> {
    for dev in pm.devices().expect("Can read info for all devices") {
        if dev.is_input() && name.trim() == dev.name().trim() {
            return Ok(dev.id());
        }
    }
    Err(Error::NamedDeviceNotFound(name.into()))
}

/// Function to clear all pending messages from the port.
///
/// If the port is not emtpy upon starting this application, then there may be
/// issues. This function is supposed to empty the port. If it turns out that
/// the port is not emtpy at the end of this function call, please consult the
/// following in an attempt to fix it.
///
/// # Sleeping ...
///
/// Note that we are using a sleep function after reading new information from
/// the port. This is necessary, as otherwise the pending messages will not
/// appear one after another &mdash; which seems to make it very difficult and
/// inefficient to figure out if there are still pending messages left.
///
/// To avoid spending a lot of time waiting for this function to finish, we set
/// the sleep timer to as low as possible. It might be that the timer is set too
/// low in which case the sleep has no effect and the messages will not appear
/// one after the other. Try putting a little larger value in the timer until it
/// works.
///
/// If you reach a point where the timer has a big value like 10ms then the
/// issue is probably not caused by the sleep (see next section).
///
/// # Skipping ...
///
/// You will see in the code that we are skipping the first `None` value that is
/// obtained. Through experimentation we found out that the first value yielded
/// in this manner is always `None`. Only after the first `None` are we getting
/// the pending messages.
///
/// If this function does not clear all pending messages, try inspecting the
/// output of the port by using the following code inside the loop:
///
/// ```
/// match port.read_n(BUFFER_SIZE) {
///     Ok(events) => println!(events),
///     Err(e) => panic!("Error {e}"),
/// }
/// ```
///
/// This way you can see when your pending messages are appearing, in which case
/// you can modify the if conditions to correctly skip the `None` values that
/// don't indicate an emptied port.
///
/// # Panics
///
/// Panics if messages failed to be read from the port (see
/// [portmidi::InputPort::read_n]). For now, it is considered an irrecoverable
/// error because an uncleared port may lead to undesired behaviour (such as
/// unwanted messages appearing at the start of the application). Further, if
/// reading failed here, it may very well fail later down the road when it comes
/// to reading the actual MIDI data.
fn clear_port(port: &InputPort) {
    let mut first_iteration = true;
    loop {
        match port.read_n(BUFFER_SIZE) {
            Ok(events) => match events {
                Some(_) => (),
                None => {
                    if first_iteration {
                        // the first message always seems to be None
                        // we need to ignore it in case notes follow
                        first_iteration = false;
                        continue;
                    } else {
                        // port is clear if we encounter another None
                        break;
                    }
                }
            },
            Err(e) => panic!("Failed to read events. Error: {e}"),
        }
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    // initialize the PortMidi context.
    let context = pm::PortMidi::new().expect("At least one MIDI device available.");
    // let name = "out";
    let name = "USB-MIDI MIDI 1";

    list_devices(&context);

    let id = get_device_id_by_name(name, &context).unwrap();

    // get the device info for the given id
    let info = context.device(id).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    // get the device's input port
    let in_port = context
        .input_port(info, BUFFER_SIZE)
        .expect("An input device was provided");

    clear_port(&in_port);

    while in_port.poll().is_ok() {
        if let Ok(Some(event)) = in_port.read_n(BUFFER_SIZE) {
            println!("{:?}", event);
        } else {
            println!("No message");
        }
        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(Duration::from_millis(10));
    }
}
