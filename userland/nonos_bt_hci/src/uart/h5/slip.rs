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

//! The Three-wire UART's SLIP layer (Vol 4, Part D, "SLIP Layer"). Each
//! packet is
//! sent between two 0xC0 octets. Inside it, 0xC0 is sent as 0xDB 0xDC
//! and 0xDB as 0xDB 0xDD. When out-of-frame software flow control is in
//! use, 0x11 and 0x13 are also sent escaped, as 0xDB 0xDE and 0xDB 0xDF,
//! so neither appears in the stream except as flow control.

use alloc::vec::Vec;

pub const END: u8 = 0xc0;
pub const ESC: u8 = 0xdb;
const XON: u8 = 0x11;
const XOFF: u8 = 0x13;

/// The octet each escape code stands for, if `oof` allows it.
pub fn unescape(code: u8, oof: bool) -> Option<u8> {
    match code {
        0xdc => Some(END),
        0xdd => Some(ESC),
        0xde if oof => Some(XON),
        0xdf if oof => Some(XOFF),
        _ => None,
    }
}

/// `packet` framed for the wire.
pub fn encode(packet: &[u8], oof: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(packet.len() + 2);
    out.push(END);
    for &b in packet {
        match b {
            END => out.extend_from_slice(&[ESC, 0xdc]),
            ESC => out.extend_from_slice(&[ESC, 0xdd]),
            XON if oof => out.extend_from_slice(&[ESC, 0xde]),
            XOFF if oof => out.extend_from_slice(&[ESC, 0xdf]),
            _ => out.push(b),
        }
    }
    out.push(END);
    out
}
