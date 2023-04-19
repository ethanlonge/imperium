use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use midir::os::unix::VirtualInput;
use crate::keyboard::{Keyboard, KeyboardLcd, KeyboardPad};
use super::prelude as consts;

pub struct KLE_61 {
    daw_out_conn: MidiOutputConnection
}

impl KLE_61 {
    pub fn new() -> Result<Self, ()> {
        let midi_out = MidiOutput::new("kle61_out")
            .map_err(|_e| ())?;

        let daw_out_ports = midi_out.ports();

        let daw_out_port = daw_out_ports
            .iter()
            .filter(|e| -> bool {
                let name = midi_out.port_name(&e);

                match name {
                    Ok(s) => s.eq("Arturia KeyLab Essential 61 DAW Out"),
                    Err(_) => false
                }
            })
            .last();

        if daw_out_port.is_none() {
            return Err(());
        }

        let daw_out_conn = midi_out.connect(daw_out_port.unwrap(), "kle61_daw_out");

        if daw_out_conn.is_err() {
            return Err(());
        }

        let daw_out_conn = daw_out_conn.unwrap();

        Ok(KLE_61 {
            daw_out_conn
        })
    }

    pub fn new_fake() -> Result<(MidiInputConnection<()>, Self), ()> {
        let midi_in = MidiInput::new("kle61_fake_out")
            .map_err(|_e| ())?;

        let vo = midi_in.create_virtual("Arturia KeyLab Essential 61 DAW Out", |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        }, ()).map_err(|_e| ())?;

        Ok((vo, Self::new()?))
    }
}

impl Keyboard for KLE_61 {
    const HEADER: [u8; 6] = consts::HEADER;
    const FOOTER: [u8; 1] = consts::FOOTER;

    fn get_dawout_port(&mut self) -> &mut MidiOutputConnection {
        &mut self.daw_out_conn
    }
}

impl KeyboardLcd for KLE_61 {
    const LCD_TEXT_HEADER: [u8; 4] = consts::LCD_TEXT_HEADER;
    const LCD_TEXT_NEWLINE: [u8; 2] = consts::LCD_TEXT_NEWLINE;
}

impl KeyboardPad for KLE_61 {
    const PAD_COUNT: u8 = 8;
    const PAD_LIGHT_HEADER: [u8; 3] = consts::PAD_LIGHT_HEADER;
}