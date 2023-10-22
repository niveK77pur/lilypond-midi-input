Standalone tool reading input from a MIDI device and converting them into LilyPond notes, with integration into other tools as a strong focus.

<!--toc:start-->
- [About](#about)
- [Non-goals](#non-goals)
- [Features](#features)
- [Installation](#installation)
- [Basic Usage](#basic-usage)
  - [Providing Options](#providing-options)
  - [Changing options](#changing-options)
- [Specifications for integration into editors](#specifications-for-integration-into-editors)
  - [Managing the process](#managing-the-process)
  - [stdin](#stdin)
  - [stdout](#stdout)
  - [stderr](#stderr)
- [Options](#options)
  - [Command Line Arguments](#command-line-arguments)
  - [Options for stdin](#options-for-stdin)
- [Integrations](#integrations)
- [See also](#see-also)
- [TODO](#todo)
<!--toc:end-->

# About

This is a tool specifically targeted at writing [LilyPond][lilypond] scores. Entering notes using a MIDI keyboard is very handy and can greatly speed up the process, for which I always used [Frescobaldi][frescobaldi]. There was an issue however &mdash; I already had a fully personalized setup for writing LilyPond in my text editor of choice, yet always found myself going back to Frescobaldi for the MIDI input; as a result, I ended up writing my scores in Frescobaldi, even beyond the MIDI input. (Frescobaldi is great though!)

`lilypond-midi-input` aims to bridge the gap between MIDI input for LilyPond notes, and any arbitrary text editor which supports async inputs. The idea is that this tool will listen for MIDI inputs from a device, and will transform them into corresponding LilyPond notes that can directly be inserted into your LilyPond files!

This is a standalone program which does just that: Read MIDI inputs from a device, and spit out LilyPond notes onto stdout. This will hopefully make integration into other editors easier. [Basic usage](#basic-usage) walks through how the program works. For those wishing to integrate this into their editors, please take a look at the [specifications](#specifications-for-integration-into-editors) on how to handle the input and output streams.

# Non-goals

Fully automate text input for LilyPond notes is not an objective for this tool. This means for example that adding note durations will not be handled here. Automatically detecting rhythm during playback is therefore also not an objective of this tool. Such features should be provided/created by wrappers.

Again, the main goal here is to provide translation of MIDI notes into LilyPond notes, and as a result make MIDI input easier to integrate into other editors.

# Features

*NB: Bullet points with an arrow right after can be clicked on and expanded to show a **demo video**!*

- <details>
    <summary>
      All notes on a keyboard are translated to LilyPond notes with <a href="https://lilypond.org/doc/v2.24/Documentation/notation/writing-pitches#absolute-octave-entry">absolute octave entry</a>
    </summary>

    A chromatic scale being played across the entire piano, with their corresponding lilypond notes being output.
    
    https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/f56b82b0-8d25-43cb-9653-f30012da2df3
  </details>
- <details>
    <summary>
      Specify musical key signatures to influence how accidentals (black keys) are interpreted
    </summary>
  
    Shows the following keys
    * C major
    * A minor (harmonic minor), note the G sharp note
    * B major, note all black keys being sharps
    * G sharp minor (harmonic minor), note the G natural being output as F double-sharp
    * C flat major, note all black keys being flats
    * B flat minor (harmonic minor)

    https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/b6e8df79-0eea-4f27-9adf-79887b6fb876
  </details>
- <details>
    <summary>
      Specify how to handle accidentals outside a key signature (fall back to sharps or flats)
    </summary>

    * Example in *F major* which has a B flat
    
      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/fa600949-3346-45a9-a2b0-f331e64904ae
      
    * Example in *G major* which has an F sharp
    
      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/7db393f2-7eec-40e2-94a7-ec9fb9152306
  </details>
- Different input modes
    - <details><summary><b>Single</b>: Input one note at a time</summary>

      - Shows a scale being played
      - Shows a chord being played and how it inserts only single notes (even if all are held)
      - Shows long held notes to highlight that notes are inserted as soon as key is **pressed**

      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/258f9ec6-b509-4982-8600-adbea683df14
      </details>
    - <details><summary><b>Chord</b>: Allow inputting chords by holding down multiple keys at once</summary>
 
      - Shows a chord being played and how it is inserted after releasing the keys
      - Shows notes being held, while pressing new ones and releasing others, highlighting that notes will be aggregated until everything is released
      - Shows long held notes to highlight notes are inserted as soon as all keys are **released**

      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/db2f246e-bd45-42fb-9ab0-37c15a0f7ec2
      </details>
    - <details><summary><b>PedalChord</b>: Behave like <b>Chord</b> when any piano pedal is pressed, otherwise behave like <b>Single</b></summary>

      - Shows chord being played without pedal, behaving like **Single**
      - Shows chord being with pedal, behaving like **Chord**
      
      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/f5e7d09a-3874-4d5d-b533-420e4f4ce08e
      </details>
    - <details><summary><b>PedalSingle</b>: Behave like <b>Single</b> when any piano pedal is pressed, otherwise behave like <b>Chord</b> (the opposite of how <b>PedalChord</b> behaves)</summary>

      - Shows chord being played without pedal, behaving like **Chord**
      - Shows chord being played with pedal, behaving like **Single**
      
      https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/e7476182-4f74-4e8a-91f5-d8d8d5784288
      </details>
- <details><summary>Specify custom alterations for notes within a scale/octave</summary>

  - Shows every C being replaced by `YO`
  - Shows every B being replaced by `BYE`
  
  https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/8e4933a1-9edc-4a48-8d2a-5bbee7f80976
  </details>
- <details><summary>Specify custom alterations across all notes of the MIDI device</summary>

  - Shows one specific C being replaced by `YO`
  - Shows one specific B being replaced by `BYE`

  https://github.com/niveK77pur/lilypond-midi-input/assets/10981161/dc6287a6-2146-400e-a8a3-68743d67aeb5
  </details>
- List all available MIDI input devices
- Specific handling of input/output for [integration into other editors](#specifications-for-integration-into-editors)
    - **stdout** for relevant ouptut
    - **stderr** for sharing messages from the tool
    - **stdin** to asynchronously take options to change [settings](#options-for-stdin) on-the-fly

See also points from [TODO](#todo).

# Installation

You will need [PortMidi](https://github.com/PortMidi/PortMidi) installed, regardless of the installation method. Note the `libportmidi-dev` package should only be needed for Ubuntu when building from source.

```shell
pacman -S portmidi # for arch
apt install libportmidi0 libportmidi-dev # for debian/ubuntu
```

## Pre-built binaries

The [latest release](https://github.com/niveK77pur/lilypond-midi-input/releases/latest) will contain pre-built binaries (different versions due to the PortMidi system library).
- [Debian](https://github.com/niveK77pur/lilypond-midi-input/releases/latest/download/lilypond-midi-input_debian), should also work on Ubuntu
- [Arch Linux](https://github.com/niveK77pur/lilypond-midi-input/releases/latest/download/lilypond-midi-input_archlinux)

Be sure to make the binaries available as `lilypond-midi-input` on your system, without the `_*` *extension*. That one was only useful to distinguish the different versions in the release assets.

## Build from source

You will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [PortMidi](https://github.com/PortMidi/PortMidi) installed to build the project.

```shell
cargo install --path . # inside this repository
```

# Basic Usage

A comprehensive overview of settings and features can be found using the help page. More information can be found [in a later section](#options).

```
lilypond-midi-input --help
```

First, you need to specify which MIDI input device this tool should listen to. You can use the following command to get a list of available input devices. Take note of the name for the device of interest, we need to give it to the program to actually run it.

```
$ lilypond-midi-input --list-devices
1) Input: Midi Through Port-0
3) Input: USB-MIDI MIDI 1
4) Input: out
```

Let's say we are interested in the input device listed as number 3 here. You can finally run the tool as follows. **NOTE** that the name must be an exact match! Leading and trailing spaces in the name are ignored.

```
lilypond-midi-input "USB-MIDI MIDI 1"
```

To exit, you can simply press `Ctrl+C`.

## Providing Options

As indicated by the `--help` page, you can pass various options via command line flags, which shall not be elaborated on further. It should be mentioned that using command line flags will set the options on start-up and also provides a bit more helpful error messages if arguments are invalid.

The next method discussed will launch the program (with its default values), and allow changing options later. Practically speaking, there really is no major difference between the two methods. If your editor cannot write to this program's stdin stream, you can use these flags as a workaround to relaunch with new settings.

## Changing options

This tool also allows changing/setting the options on-the-fly without restarting the program. To do this, you can directly type into the program's stdin! Meaning that while the program is running, you can simply type commands into the terminal.

Upon successful parsing and execution of the given setting, the program will write a message to stderr, either indicating success or possibly indicating errors. As far as possible, the program tries to inform what has happened (through stderr), as otherwise it is difficult to judge whether the provided settings in stdin where handled correctly or not.

All options here have long and short versions, which the latter are particularly useful when manually typing in the commands into the terminal. A list of options and their values can be found in a [later section](#options).

The settings are given in the following form. You can specify one option at a time, or you can provide multiple options at once. **NOTE** that options are space separated; so currently the values may not contain any spaces. A key that takes nested key-value pairs has its value given as `SUBKEY:SUBVALUE` and are comma separated (without spaces). Here are some examples to hopefully clarify.

```
KEY1=VALUE1
KEY3=SUBKEY1:SUBVALUE1,SUBKEY2:SUBVALUE2
KEY1=VALUE1 KEY2=VALUE2
KEY1=VALUE1 KEY2=VALUE2 KEY3=SUBKEY1:SUBVALUE1,SUBKEY2:SUBVALUE2
```

# Specifications for integration into editors

The interaction with this tool happens fully through stdin, stdout and stderr. Here is how each of these streams are used by this tool, allowing you to properly integrate it into your editor.

## Managing the process

Spawning the process is ideally done by your editor, so that it can properly manage all the input and output streams.

Specifics on how to interact with each stream is of course dependent on the editor and its capabilities. You can have a look at existing [integrations](#integrations) for some examples and inspiration.

Important to note is that the tool is not capable of exiting by itself (i.e. there is no `exit` command for example). That said, you should try to kill the process in question, which should ideally be done by your editor.

## stdin

As mentioned in [changing options](#changing-options), the stdin solely takes settings as key-value pairs. Upon successful parsing, the corresponding option will be set/updated internally. A corresponding message will also be written to stderr.

For options and their values, please check the [following section](#options); for usage examples please check the section on [changing options](#changing-options).

Also note that if the program is not responding to inputs being sent through stdin, it is possible that you have provided an invalid option which is simply not being parsed and captured. Or, it is possible that your editor also needs to add a newline at the end of the message, in order to trigger Rust to actually read the input line.

## stdout

This stream should only output data relevant to the task at hand. In the case of `--list-devices`, it will be the list of devices. In the case of a normal execution, stdout will only have LilyPond notes printed as you input notes through your MIDI keyboard.

That said, stdout can be taken as-is. A user could for example be prompted to pick a MIDI device based on the output of `--list-devices`. Most importantly, during normal execution the outputted LilyPond notes can be taken as-is in order to have them inserted into your text editor.

## stderr

This stream contains any other message/information that the tool wants to share but should not be taken as text input by the editor. Currently, this counts general information such as a startup message, and indications that values were updated correctly via stdin. In case an option via stdin was invalid, an error message will also be written to stderr.

Errors are printed using the `echoerr!` macro, while other information is printed using the `echoinfo!` macro, the definition of both are found in [this file][macros]. They prefix each line with a `!!` and `::` respectively. This allows your client/editor to filter the messages from stderr according to actual errors or simple information.

[macros]: ./src/lily/macros.rs

## Providing a list of options to the user

The program also provides a `--list-options` flag, which list all available values for a given argument to stdout. The options are space separated, and no particular effort is made towards providing a well typeset output (i.e. as a tabular); the editors should decide how to treat the information.

The first value in the line corresponds to the actual enum variant's name in the Rust code. The second value corresponds to the primary string from which the variant can be created. All following values are additional strings &mdash; usually shorthands &mdash; which can also be used to describe an enum variant. (See also [the table](#options-for-stdin)).

All the string values (without any `"` or `'`) can be used as-is to set an option via stdin. The second string value can be used to set options via the command line arguments.

Using this method to display choices to in the editor should be preferred as it avoids hardcoding the values. Further, if values should change, be added, or removed, it will require no intervention in the editor, as this tool can list its own options.

# Options

## Command Line Arguments

All flags and the values they can take are shown when running the program with the `--help` flag. Thus, they will not be further discussed.

Of importance to point out are the values expected by `--alerations` and `--global-alterations`. Both of these take a list of comma-separated subkey-subvalue pairs, which are mentioed in [a previous section](#changing-options). More concrete details are given in [the table](#options-for-stdin).

## Options for stdin

The option keys are the exact same as the command line flags but with the leading dashes removed. There are a few additional shorthands though. Also, the values it can take are a bit more broad compared to what the command line flags allow. Some of the values also allow shorthands. The following table describes the current options and their values. See also [the section on changing options](#changing-options) for examples on how to actually set them.

<table>
    <tr>
        <th colspan=2>Options</th>
        <th>Values</th>
        <th>Description</th>
        <th>Example</th>
    </tr>
    <tr>
        <th>Long</th>
        <th>Short</th>
        <th></th>
        <th></th>
        <th></th>
    </tr>
    <tr>
        <td>key</td>
        <td>k</td>
        <td>Can take all strings and enum variant names in the <a href="./src/lily/keysignature.rs">list of available keysignatures</a>.</td>
        <td>Affects how accidentals will be printed depending on the given key signature. In GMajor, an F♯/G♭ will always be printed as <code>fis</code> no matter the value of <code>accidentals</code>. This can be overriden by alterations.</td>
        <td><code>k=BFlatMajor</code> is equivalent to <code>key=besM</code>.</td>
    </tr>
    <tr>
        <td>accidentals</td>
        <td>a</td>
        <td>Can take all strings and enum variant names in the <a href="./src/lily/accidentals.rs">list of accidentals</a>.</td>
        <td>How to print accidentals that are not within the musical key? In the key of FMajor, <code>sharps</code> will print a G♯ (gis), whereas <code>flats</code> will print an A♭ (aes).</td>
        <td><code>a=sharps</code> is equivalent to <code>a=s</code></td>
    </tr>
    <tr>
        <td>mode</td>
        <td>m</td>
        <td>Can take all strings and enum variant names in the <a href="./src/lib.rs">list of input modes</a>.</td>
        <td>How to handle MIDI input? <code>Single</code> will only read one single note at a time as they are pressed. <code>Chord</code> will print a LilyPond chord after all notes were released. <code>PedalChord</code> merges both, behaving like <code>Chord</code> when any of the three pedals are pressed, and behaving like <code>Single</code> when all pedals are released. <code>PedalSingle</code> inverted the behaviour.</td>
        <td><code>mode=Pedal</code> is equivalent to <code>m=p</code></td>
    </tr>
    <tr>
        <td>alterations</td>
        <td>alt</td>
        <td>Subkey-subvalue pairs. I.e. <code>key:value</code> or <code>key1:value1,key2:value2,...</code>. The key must be an integer between 0 and 11 inclusive, the value is considered a string (may not contain spaces). Trailing <code>+</code> or <code>-</code> in the value can be used to adjust the octave up or down respectively. Multiple consecutive trailing <code>+</code> or <code>-</code> can be used to adjust multiple octaves.</td>
        <td>Set custom alterations within an octave; overrides special considerations for <code>key</code> signatures. Ottavation marks are still being set here. The numbers indicate each note in an octave, starting from C=0, C♯=1, D=2, ..., B=11</td>
        <td><code>0:hello,10:world</code> will make every note C output <code>hello</code> and every B♭ output <code>world</code>, together with their LilyPond ottavations (<code>'</code> or <code>,</code>). An alteration of <code>0:bis</code> will make the note produced by pressing a C always one octave too high; this can be remedied by doing <code>0:bis-</code>.</td>
    </tr>
    <tr>
        <td>global-alterations</td>
        <td>galt</td>
        <td>Same as <code>alterations</code>, without the integer constraint, and without the ottavation adjustments.</td>
        <td>Set custom alterations over all MIDI notes; further overrides <code>alterations</code> and key signatures. The numbers indicate the MIDI value attributed to said note. No ottavation marks (<code>'</code> or <code>,</code>) are applied.</td>
        <td><code>60:hello</code> will only make middle C print a <code>hello</code>.</td>
    </tr>
    <tr>
        <td>previous-chord</td>
        <td>pc</td>
        <td>Colon (<code>:</code>) separated list of <a href="https://lilypond.org/doc/v2.22/Documentation/notation/writing-pitches#absolute-octave-entry">absolute LilyPond note</a> strings. Or <code>clear</code> to unset the previous chord.</td>
        <td>Explicitely specify a chord which will yield <code>q</code> upon repeating. Useful when jumping around the file, and the tool does or does not return <code>q</code> appropriately.</td>
        <td><code>pc=c,:eis':g</code> will set <code><c, g eis'></code> as the previous chord. <code>pc=clear</code> will forget the previous chord.</td>
    </tr>
    <tr>
        <td>list</td>
        <td></td>
        <td>Long or short version of all other options. Alternatively <code>all</code> will list the all values.</td>
        <td>Not exactly an option, but allows listing values for options. Useful to see what the current state is.</td>
        <td><code>list=k</code> or <code>list=key</code> list the currently set key signature. <code>list=all</code> will list the current values of all options.</td>
    </tr>
</table>

# Integrations

I have written my own [Neovim plugin][nvim-midi] which uses this tool to allow inputting notes asynchronously using a MIDI keyboard in Neovim! It also follows Vim's modal philosophy and only inserts notes in *Insert mode*, and allows replacing notes in *Replace mode*!

# See also

- MIDI input for LilyPond, able to quantize notes on the fly using a metronome: <https://directory.fsf.org/wiki/Rumor>
- Another LilyPond MIDI input tool: <https://github.com/nicolodavis/lilypond-midi-input>
- A proper CLI midi player: <https://gitlab.com/dajoha/midiplay>

# TODO

- [ ] Generate notes for [relative octave entry](https://lilypond.org/doc/v2.24/Documentation/notation/writing-pitches#relative-octave-entry)
- [x] [Repeated chords](https://lilypond.org/doc/v2.24/Documentation/notation/single-voice#chord-repetition) should return `q`
- [x] List all currently set (global) alterations
- [x] List all options for a setting (avoids hardcoding them into editors)
- [x] Simple screencast to show how this looks in action (under [features](#features))
- [ ] Debug option/mode to see raw midi events
- [x] Specify ottavation for alterations (i.e. `0=bis` will cause the note to always be one octave too high)

[lilypond]: https://lilypond.org/
[frescobaldi]: https://frescobaldi.org/
[nvim-midi]: https://github.com/niveK77pur/midi-input.nvim
