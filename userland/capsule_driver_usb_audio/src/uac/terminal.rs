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

//! What the device plays into, from its Output Terminals (USB Audio 1.0
//! 4.3.2.2 as recalled, unverified; terminal types from "Universal Serial Bus Device Class
//! Definition for Terminal Types" 1.0), so the settings page names it.

use nonos_usbnet::desc::walk;

use super::alt::{CLASS_AUDIO, CS_INTERFACE, INTERFACE};

const SUBCLASS_CONTROL: u8 = 0x01;
const OUTPUT_TERMINAL: u8 = 0x03;
/// A terminal that leads back to the host: a capture path, not an output.
const USB_STREAMING: u16 = 0x0101;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Outputs {
    pub speaker: bool,
    /// Worn on the head: headphones, a headset, a head-mounted display.
    pub headphone: bool,
    pub line: bool,
}

/// Every output the audio function in `config` drives. A terminal type
/// this does not know counts as a line out; a function that names none
/// does too, since its sound still leaves the device somewhere.
pub fn outputs(config: &[u8]) -> Outputs {
    let mut out = Outputs::default();
    let mut control = false;
    for d in walk(config) {
        if d[1] == INTERFACE && d.len() >= 9 {
            control = d[5] == CLASS_AUDIO && d[6] == SUBCLASS_CONTROL;
        } else if control && d[1] == CS_INTERFACE && d.len() >= 9 && d[2] == OUTPUT_TERMINAL {
            match u16::from_le_bytes([d[4], d[5]]) {
                USB_STREAMING => {}
                0x0302 | 0x0303 | 0x0402 => out.headphone = true,
                0x0301 | 0x0304..=0x0307 | 0x0403 | 0x0404 => out.speaker = true,
                _ => out.line = true,
            }
        }
    }
    if out == Outputs::default() {
        out.line = true;
    }
    out
}
