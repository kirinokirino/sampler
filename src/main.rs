use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput, MidiInputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    let in_port = choose_input_port(&mut midi_in, "UMX 61")?;
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {
            let (status, channel) = (message[0] & 0xF0, message[0] & 0x0F);
            let (note, volume) = (message[1] & 0x7F, message[2] & 0x7F);
            match status {
                0x80 => {
                    // note-off
                    println!(
                        "note-off, note: {}, volume: {}, channel: {}",
                        note, volume, channel
                    );
                }
                0x90 => {
                    // note-on
                    println!(
                        "note-on, note: {}, volume: {}, channel: {}",
                        note, volume, channel
                    );
                }
                _ => (),
            }
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

pub fn choose_input_port(
    midi_in: &mut MidiInput,
    expected_port_name: &str,
) -> Result<MidiInputPort, Box<dyn Error>> {
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                let name = midi_in.port_name(p).unwrap();
                println!("{}: {}", i, name);
                if name.contains(expected_port_name) {
                    println!("Found port {name}");
                    return Ok(p.clone());
                }
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };
    Ok(in_port.clone())
}
