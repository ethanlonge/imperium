use midir::MidiOutputConnection;

pub trait Keyboard {
    const HEADER: [u8; 6];
    const FOOTER: [u8; 1];

    fn get_dawout_port(&mut self) -> &mut MidiOutputConnection;

    fn open(&mut self) -> Result<(), ()> {
        let msg: [u8; 13] = [240, 0, 32, 107, 127, 66, 2, 0, 64, 17, 127, 0, 24]; // Pose as Live

        self.get_dawout_port().send(&msg).map_err(|_e| ())
    }

    fn close(&mut self) -> Result<(), ()> {
        let msg: [u8; 13] = [240, 0, 32, 107, 127, 66, 2, 0, 64, 17, 0, 0, 247]; // Stop posing as Live

        self.get_dawout_port().send(&msg).map_err(|_e| ())
    }
}

pub trait KeyboardLcd: Keyboard {
    const LCD_TEXT_HEADER: [u8; 4];
    const LCD_TEXT_NEWLINE: [u8; 2];

    fn update_text(&mut self, line1: Option<String>, line2: Option<String>) -> Result<(), ()> {
        let mut msg = Vec::new();
        msg.extend(Self::HEADER);
        msg.extend(Self::LCD_TEXT_HEADER);

        if let Some(line) = line1 {
            msg.extend(
                line.chars()
                .map(|c| {
                    c as u8
                })
            );
        }

        msg.extend(Self::LCD_TEXT_NEWLINE);

        if let Some(line) = line2 {
            msg.extend(
                line.chars()
                    .map(|c| {
                        c as u8
                    })
            );
        }

        msg.push(0);

        msg.extend(Self::FOOTER);

        self.get_dawout_port().send(msg.as_slice()).map_err(|_e| ())
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

fn colorbit_to_hex(num: u8) -> u8 {
    let num: f64 = num.into();

    let num = num / 255f64;
    let num = num * 31f64;

    num.floor() as u8
}

pub trait KeyboardPad: Keyboard {
    const PAD_COUNT: u8;
    const PAD_LIGHT_HEADER: [u8; 3];

    fn pad_light_up(&mut self, no: u8, color: Option<&Color>) -> Result<(), ()> {
        if no > Self::PAD_COUNT || no < 1 {
            return Err(())
        }

        let mut msg = Vec::new();
        msg.extend(Self::HEADER);
        msg.extend(Self::PAD_LIGHT_HEADER);

        msg.push(0x6F + no);

        if let Some(color) = color {
            msg.extend([colorbit_to_hex(color.r), colorbit_to_hex(color.g), colorbit_to_hex(color.b)]);
        } else {
            msg.extend([0, 0, 0]);
        }

        msg.extend(Self::FOOTER);

        self.get_dawout_port().send(msg.as_slice()).map_err(|_e| ())
    }
}