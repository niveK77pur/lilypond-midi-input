use super::OctaveEntryError;

make_lily_str_map!(
    /// The octave entry mode to use for note generation
    OctaveEntry;
    OctaveEntryError::InvalidOctaveEntryString;
    Absolute, "absolute", "a";
    Relative, "relative", "r";
);
