// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A monitor's DDC/CI end in the model, written from the message layout
//! alone and not from the driver's code: it checks each message the host
//! writes (source 0x51, length byte, checksum over the display's 0x6E),
//! answers a Get VCP with the control's maximum and value or "unsupported",
//! takes a Set VCP and gives its capabilities (`mccs_caps.rs`); a test can spoil a reply byte.

#[derive(Default, Clone)]
pub struct Mccs {
    pub controls: Vec<(u8, u16, u16)>, // code, value now, most it takes
    /// Every message written, as it arrived.
    pub got: Vec<Vec<u8>>,
    /// What the next read gives, and how far into it.
    pub reply: Vec<u8>,
    pub at: usize,
    /// One reply byte replaced: its index and value.
    pub spoil: Option<(usize, u8)>,
    /// The capabilities string, when not the one its controls make.
    pub caps: Option<Vec<u8>>,
}

impl Mccs {
    pub fn with(controls: &[(u8, u16, u16)]) -> Mccs {
        Mccs { controls: controls.to_vec(), ..Default::default() }
    }

    pub fn value(&self, code: u8) -> Option<u16> {
        self.controls.iter().find(|c| c.0 == code).map(|c| c.1)
    }

    pub fn take(&mut self, msg: &[u8]) {
        self.got.push(msg.to_vec());
        (self.reply, self.at) = (Vec::new(), 0);
        let sum = msg.iter().fold(0x6Eu8, |a, b| a ^ b);
        if msg.len() < 4 || msg[0] != 0x51 || msg[1] as usize != 0x80 + msg.len() - 3 || sum != 0 {
            return;
        }
        match (msg[2], msg.len()) {
            (0x01, 5) => self.reply = super::mccs_reply::get_reply(&self.controls, msg[3]),
            (0xF3, 6) => self.reply = super::mccs_caps::caps_reply(self, msg[3], msg[4]),
            (0x03, 7) => {
                let v = u16::from_be_bytes([msg[4], msg[5]]);
                if let Some(c) = self.controls.iter_mut().find(|c| c.0 == msg[3]) {
                    c.1 = v;
                }
            }
            _ => {}
        }
    }

    /// The next byte a read takes; past the reply, 0xFF.
    pub fn next_byte(&mut self) -> u8 {
        let b = match self.spoil {
            Some((i, v)) if i == self.at => v,
            _ => *self.reply.get(self.at).unwrap_or(&0xFF),
        };
        self.at += 1;
        b
    }
}
