use portmidi::{InputPort, MidiEvent, PortMidi, PortMidiDeviceId};

mod errors;
mod types;

pub use errors::*;
pub use types::*;

/// An input port with which to read MIDI events
///
/// # Minimum working example
///
/// The following demonstrates how to use this struct. At the very core it is
/// used to read MIDI events from an input port. The [`portmidi`] context needs
/// to be manually created due to lifetime handling of that crate. Next, the
/// [`crate::list_devices`] helper function lists all available devices, which
/// the name can be used to to create a [`crate::MidiInputPort`]. One should
/// ideally also clear the port, as there may be pending messages which will be
/// output right upon starting the program. Finally, we listen to the port, and
/// can execute a callback function on each single event that is received; in
/// this case we simple print it encoded as a [`crate::MidiMessageType`].
///
/// ```no_run
/// use lilypond_midi_input as lmi;
///
/// const BUFFER_SIZE: usize = 1024;
///
/// fn main() {
///     // initialize the PortMidi context.
///     let context = portmidi::PortMidi::new().expect("At least one MIDI device available.");
///     let name = "USB-MIDI MIDI 1";
///
///     lmi::list_devices(&context);
///
///     let port = lmi::MidiInputPort::new(name, &context, BUFFER_SIZE)
///         .expect("Port name matches an existing port");
///
///     port.clear();
///
///     port.listen(|event| println!("{:?}", lmi::MidiMessageType::from(event)))
///         .expect("Polling for new messages works.");
/// }
/// ```
pub struct MidiInputPort<'a> {
    _name: &'a str,
    _context: &'a PortMidi, // Used for lifetime pinning
    port: InputPort<'a>,
    buffer_size: usize,
}

impl<'a> MidiInputPort<'a> {
    pub fn new(
        name: &'a str,
        context: &'a PortMidi,
        buffer_size: usize,
    ) -> Result<Self, LilypondMidiDeviceError> {
        // initialize the PortMidi context.
        // let context = PortMidi::new().expect("At least one MIDI device available.");

        let id = Self::get_device_id_by_name(name, context)?;

        // get the device info for the given id
        let info = context.device(id).unwrap();
        println!("Listening on: {}) {}", info.id(), info.name());

        // get the device's input port
        let port = context
            .input_port(info, buffer_size)
            .expect("An 'input' device was provided");

        Ok(Self {
            _name: name,
            _context: context,
            port,
            buffer_size,
        })
    }

    /// Get [PortMidiDeviceId] using the port's name
    ///
    /// [portmidi] only allows setting up a port using the [PortMidiDeviceId],
    /// which is cumbersome at best for an end user. Also, it is not as
    /// robust, as there is no guarantee that the same MIDI controller will
    /// have the same ID.
    ///
    /// Creating ports using a named reference makes this more resilient and
    /// nicer to work with for users of the library.
    ///
    /// # Panics
    ///
    /// Panics if the list of devices cannot be obtained.
    ///
    /// # Errors
    ///
    /// This function will return an error if the given name does not EXACTLY
    /// match the name of an available port.
    fn get_device_id_by_name(
        name: &str,
        context: &PortMidi,
    ) -> Result<PortMidiDeviceId, LilypondMidiDeviceError> {
        for dev in context.devices().expect("Can read info for all devices") {
            if dev.is_input() && name.trim() == dev.name().trim() {
                return Ok(dev.id());
            }
        }
        Err(LilypondMidiDeviceError::NamedDeviceNotFound(name.into()))
    }

    /// Function to clear all pending messages from the port.
    ///
    /// If the port is not emtpy upon starting this application, then there may
    /// be issues. This function is supposed to empty the port. If it turns
    /// out that the port is not emtpy at the end of this function call,
    /// please consult the following in an attempt to fix it.
    ///
    /// # Sleeping ...
    ///
    /// Note that we are using a sleep function after reading new information
    /// from the port. This is necessary, as otherwise the pending messages
    /// will not appear one after another &mdash; which seems to make it
    /// very difficult and inefficient to figure out if there are still
    /// pending messages left.
    ///
    /// To avoid spending a lot of time waiting for this function to finish, we
    /// set the sleep timer to as low as possible. It might be that the
    /// timer is set too low in which case the sleep has no effect and the
    /// messages will not appear one after the other. Try putting a little
    /// larger value in the timer until it works.
    ///
    /// If you reach a point where the timer has a big value like 10ms then the
    /// issue is probably not caused by the sleep (see next section).
    ///
    /// # Skipping ...
    ///
    /// You will see in the code that we are skipping the first `None` value
    /// that is obtained. Through experimentation we found out that the
    /// first value yielded in this manner is always `None`. Only after the
    /// first `None` are we getting the pending messages.
    ///
    /// If this function does not clear all pending messages, try inspecting the
    /// output of the port by using the following code inside the loop:
    ///
    /// ```ignore
    /// match port.read_n(self.buffer_size) {
    ///     Ok(events) => println!("{:?}", events),
    ///     Err(e) => panic!("Error {e}"),
    /// }
    /// ```
    ///
    /// This way you can see when your pending messages are appearing, in which
    /// case you can modify the if conditions to correctly skip the `None`
    /// values that don't indicate an emptied port.
    ///
    /// # Panics
    ///
    /// Panics if messages failed to be read from the port (see
    /// [portmidi::InputPort::read_n]). For now, it is considered an
    /// irrecoverable error because an uncleared port may lead to undesired
    /// behaviour (such as unwanted messages appearing at the start of the
    /// application). Further, if reading failed here, it may very well fail
    /// later down the road when it comes to reading the actual MIDI data.
    pub fn clear(&self) {
        let mut first_iteration = true;
        loop {
            match self.port.read_n(self.buffer_size) {
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
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// Listen to MIDI events and execute immutable callback function on the
    /// individual events
    ///
    /// The callback takes an immutable receiver, and thus may not mutate any
    /// state. If you are getting errors about borrowing mutable values due
    /// to an `Fn` closure, try the [crate::midi::MidiInputPort::listen_mut]
    /// instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`portmidi::InputPort::poll`]
    /// fails.
    pub fn listen(&self, event_callback: impl Fn(MidiEvent)) -> Result<(), portmidi::types::Error> {
        self.listen_mut(event_callback)
    }

    /// Listen to MIDI events and execute mutable callback function on the
    /// individual events
    ///
    /// The callback takes a mutable receiver and thus may mutate state.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`portmidi::InputPort::poll`]
    /// fails.
    pub fn listen_mut(
        &self,
        mut event_callback: impl FnMut(MidiEvent),
    ) -> Result<(), portmidi::types::Error> {
        while self.port.poll().is_ok() {
            if let Ok(Some(events)) = self.port.read_n(self.buffer_size) {
                for event in events {
                    event_callback(event);
                }
            }
            // there is no blocking receive method in PortMidi, therefore
            // we have to sleep some time to prevent a busy-wait loop
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        // check for possible errors from the port
        self.port.poll().map(|_| ())
    }
}

/// List all available MIDI input devices.
///
/// See [`crate::MidiInputPort`] for an example on how to use this. The
/// [portmidi::PortMidi] context must already exist beforehand.
///
/// ```
/// use lilypond_midi_input as lmi;
/// let context = portmidi::PortMidi::new().unwrap();
/// lmi::list_devices(&context);
/// ```
///
/// # Panics
///
/// Panics if the list of devices cannot be obtained.
pub fn list_input_devices(context: &PortMidi) {
    for dev in context.devices().expect("Can read info for all devices") {
        if dev.is_input() {
            println!("{}", dev);
        }
    }
}
