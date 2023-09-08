Standalone tool reading input from a MIDI device and converting them into LilyPond notes, with integration into other tools as a strong focus.

<!--toc:start-->
- [About](#about)
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
- [TODO](#todo)
<!--toc:end-->

# About

This is a tool specifically targeted at writing [LilyPond][lilypond] scores. Entering notes using a MIDI keyboard is very handy and can greatly speed up the process, for which I always used [Frescobaldi][frescobaldi]. There was an issue however &mdash; I already had a fully personalized setup for writing LilyPond in my text editor of choice, yet always found myself going back to Frescobaldi for the MIDI input; as a result, I ended up writing my scores in Frescobaldi, even beyond the MIDI input. (Frescobaldi is great though!)

`lilypond-midi-input` aims to bridge the gap between MIDI input for LilyPond notes, and any arbitrary text editor which supports async inputs. The idea is that this tool will listen for MIDI inputs from a device, and will transform them into corresponding LilyPond notes that can directly be inserted into your LilyPond files!

This is a standalone program which does just that: Read MIDI inputs from a device, and spit out LilyPond notes onto stdout. This will hopefully make integration into other editors easier. [Basic usage](#basic-usage) walks through how the program works. For those wishing to integrate this into their editors, please take a look at the [specifications](#specifications-for-integration-into-editors) on how to handle the input and output streams.

# Installation

As of now, you have to clone this repository and install it manually using the following command. You will need [cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html), nothing else should be required.

```shell
cargo install --path .
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

# Options

## Command Line Arguments

All flags and the values they can take are shown when running the program with the `--help` flag. Thus, they will not be further discussed.

Of importance to point out are the values expected by `--alerations` and `--global-alterations`. Both of these take a list of comma-separated subkey-subvalue pairs, which are mentioed in [a previous section](#changing-options). More concrete details are given in [the table](#options-for-stdin).

## Options for stdin

The option keys are the exact same as the command line flags but with the leading dashes removed. There are a few additional shorthands though. Also, the values it can take are a bit more broad compared to what the command line flags allow. Some of the values also allow shorthands. The following table describes the current options and their values.

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
        <td>How to handle MIDI input? <code>Single</code> will only read one single note at a time as they are pressed. <code>Chord</code> will print a LilyPond chord after all notes were released. <code>Pedal</code> merges both, behaving like <code>Chord</code> when any of the three pedals are pressed, and behaving like <code>Single</code> when all pedals are released.</td>
        <td><code>mode=Pedal</code> is equivalent to <code>m=p</code></td>
    </tr>
    <tr>
        <td>alterations</td>
        <td>alt</td>
        <td>Subkey-subvalue pairs. I.e. <code>key:value</code> or <code>key1:value1,key2:value2,...</code>. The key must be an integer between 0 and 11 inclusive, the value is considered a string (may not contain spaces).</td>
        <td>Set custom alterations within an octave; overrides special considerations for <code>key</code> signatures. Ottavation marks are still being set here. The numbers indicate each note in an octave, starting from C=0, C♯=1, D=2, ..., B=11</td>
        <td><code>0:hello,10:world</code> will make every note C output <code>hello</code> and every B♭ output <code>world</code>, together with their LilyPond ottavations (<code>'</code> or <code>,</code>).</td>
    </tr>
    <tr>
        <td>global-alterations</td>
        <td>galt</td>
        <td>Same as <code>alterations</code>, without the integer constraint.</td>
        <td>Set custom alterations over all MIDI notes; further overrides <code>alterations</code> and key signatures. The numbers indicate the MIDI value attributed to said note. No ottavation marks (<code>'</code> or <code>,</code>) are applied.</td>
        <td><code>60:hello</code> will only make middle C print a <code>hello</code>.</td>
    </tr>
</table>

# Integrations

I have written my own [Neovim plugin][nvim-midi] which uses this tool to allow inputting notes asynchronously using a MIDI keyboard in Neovim! It also follows Vim's modal philosophy and only inserts notes in *Insert mode*, and allows replacing notes in *Replace mode*!

# TODO

- [ ] Generate notes for [relative octave entry](https://lilypond.org/doc/v2.24/Documentation/notation/writing-pitches#relative-octave-entry)
- [ ] [Repeated chords](https://lilypond.org/doc/v2.24/Documentation/notation/single-voice#chord-repetition) should return `q`
- [ ] List all currently set (global) alterations
- [ ] Simple screencast to show how this looks in action
- [ ] Debug option/mode to see raw midi events

[lilypond]: https://lilypond.org/
[frescobaldi]: https://frescobaldi.org/
[nvim-midi]: https://github.com/niveK77pur/nvim-midi-input
