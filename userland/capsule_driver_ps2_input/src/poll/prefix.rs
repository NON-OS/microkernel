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

//! The prefixes in front of a scan code. E0 marks the next byte as an
//! extended key. E1 opens Pause, which sends E1 1D 45 E1 9D C5 on a press:
//! each half comes out whole as a make or break of the 45 it ends in, so the
//! 1D never reads as Left Ctrl nor the 45 as Num Lock. 0xFA and 0xFE answer
//! a command wherever they land, even inside Pause, whose bytes they never are.

use crate::keymap::flags::{FLAG_BREAK, FLAG_E0_PREFIX, FLAG_E1_PREFIX};

const E0_PREFIX: u8 = 0xE0;
const E1_PREFIX: u8 = 0xE1;
const ACK: u8 = 0xFA;
const RESEND: u8 = 0xFE;
const BREAK_BIT: u8 = 0x80;

/// What one byte turned out to be.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scan {
    /// A key's make or break code with its FLAG_* bits.
    Key { scancode: u8, flags: u8 },
    /// The keyboard's ACK or RESEND to a command.
    Reply(u8),
    /// A prefix, or a byte inside the Pause sequence: nothing yet.
    Pending,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Prefix {
    e0: bool,
    /// Bytes of an E1 sequence still to come.
    e1_left: u8,
}

impl Prefix {
    /// No prefix pending: what a keyboard's stream starts in.
    pub const IDLE: Prefix = Prefix { e0: false, e1_left: 0 };

    pub fn take(&mut self, byte: u8) -> Scan {
        if matches!(byte, ACK | RESEND) {
            return Scan::Reply(byte);
        }
        let brk = if byte & BREAK_BIT != 0 { FLAG_BREAK } else { 0 };
        if self.e1_left > 0 {
            self.e1_left -= 1;
            return match self.e1_left {
                0 => Scan::Key { scancode: byte, flags: FLAG_E1_PREFIX | brk },
                _ => Scan::Pending,
            };
        }
        match byte {
            E0_PREFIX => self.e0 = true,
            E1_PREFIX => *self = Prefix { e0: false, e1_left: 2 },
            _ => {
                let e0 = if core::mem::take(&mut self.e0) { FLAG_E0_PREFIX } else { 0 };
                return Scan::Key { scancode: byte, flags: brk | e0 };
            }
        }
        Scan::Pending
    }
}
