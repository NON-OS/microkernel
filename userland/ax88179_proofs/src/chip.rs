// NONOS Operating System (AGPL-3.0-or-later)
//! A scripted AX88179: its descriptors as it shows them on SuperSpeed (as
//! recalled, unverified: vendor interface 0 with interrupt IN 81h, bulk IN
//! 82h, bulk OUT 03h), its PHY and FIFO answers, and writes refused on
//! demand.

use nonos_usbnet::mock::Call;

pub const DEVICE: [u8; 18] =
    [18, 1, 0, 3, 0xff, 0xff, 0, 9, 0x95, 0x0b, 0x90, 0x17, 0, 1, 1, 2, 3, 1];
pub const CONFIG: [u8; 57] = [
    9, 2, 57, 0, 1, 1, 0, 0xa0, 0x32, // configuration 1
    9, 4, 0, 0, 3, 0xff, 0xff, 0, 0, // vendor interface
    7, 5, 0x81, 3, 8, 0, 11, 6, 0x30, 0, 0, 8, 0, // status, interrupt IN
    7, 5, 0x82, 2, 0, 4, 0, 6, 0x30, 3, 0, 0, 0, // bulk IN, 1024
    7, 5, 0x03, 2, 0, 4, 0, 6, 0x30, 3, 0, 0, 0, // bulk OUT, 1024
];

/// What the chip answers: PHY registers 0 and 11h, the FIFO word, and the
/// vendor write (counted from 0) it refuses.
pub struct Chip {
    pub control: u16,
    pub phy: u16,
    pub phy_fails: bool,
    pub fifo: u32,
    pub refuse: Option<usize>,
    pub outs: usize,
}

pub fn healthy() -> Chip {
    Chip { control: 0x1140, phy: 0, phy_fails: false, fifo: 0, refuse: None, outs: 0 }
}

pub fn answer(c: &mut Chip, call: &Call) -> Result<Vec<u8>, i32> {
    match call {
        Call::In(s, _) if s.request_type == 0xC0 => match (s.request, s.value, s.index) {
            (2, 3, 0) => Ok(c.control.to_le_bytes().to_vec()),
            (2, 3, 0x11) if !c.phy_fails => Ok(c.phy.to_le_bytes().to_vec()),
            (0x81, 0x8C, 0) => Ok(c.fifo.to_le_bytes().to_vec()),
            _ => Err(-32),
        },
        Call::Out(s, _) if s.request_type == 0x40 => {
            c.outs += 1;
            if c.refuse == Some(c.outs - 1) {
                Err(-32)
            } else {
                Ok(vec![])
            }
        }
        _ => Ok(vec![]),
    }
}
