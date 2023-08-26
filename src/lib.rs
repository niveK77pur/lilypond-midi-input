use portmidi::{InputPort, MidiEvent, PortMidi, PortMidiDeviceId};

mod errors;
mod types;

pub use errors::*;
pub use types::*;

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
    /// match port.read_n(self.buffer_size) {
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

    pub fn listen(&self, event_callback: fn(MidiEvent)) -> Result<(), portmidi::types::Error> {
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

pub fn list_devices(context: &PortMidi) {
    for dev in context.devices().unwrap() {
        println!("{}", dev);
    }
}
