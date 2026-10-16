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

//! The model monitor's capabilities string and its replies, from the
//! layout: a string naming every control it has (the input source with
//! two inputs, the power mode with on and off), cut into replies of at
//! most 32 bytes from the offset asked, each framed as a VCP reply is,
//! and an empty one past the end.

use super::mccs::Mccs;

/// The string the monitor's controls make.
pub fn caps_text(m: &Mccs) -> Vec<u8> {
    let mut s = b"(prot(monitor)type(lcd)model(M1)cmds(01 02 03 F3)vcp(".to_vec();
    for c in &m.controls {
        s.extend(format!("{:02X} ", c.0).bytes());
        if c.0 == 0x60 {
            s.extend(b"(0F 11) ");
        }
    }
    s.extend(b"D6(01 04))mccs_ver(2.2))");
    s
}

pub fn caps_reply(m: &Mccs, hi: u8, lo: u8) -> Vec<u8> {
    let text = m.caps.clone().unwrap_or_else(|| caps_text(m));
    let at = (u16::from_be_bytes([hi, lo]) as usize).min(text.len());
    let part = &text[at..(at + 32).min(text.len())];
    let mut r = vec![0x6E, 0x80 | (3 + part.len()) as u8, 0xE3, hi, lo];
    r.extend_from_slice(part);
    r.push(r.iter().fold(0x50u8, |a, b| a ^ b));
    r
}
