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
//! The line's settings and the class requests that carry them (CDC PSTN
//! 1.2, 6.3.10 to 6.3.13, as recalled, unverified): LineCoding is seven
//! bytes, the rate in bits per second, then stop bits, parity, data bits.

use nonos_usbnet::Setup;

const SET_LINE_CODING: u8 = 0x20;
const SET_CONTROL_LINE_STATE: u8 = 0x22;
const SEND_BREAK: u8 = 0x23;
/// SET_CONTROL_LINE_STATE's wValue bits.
pub const DTR: u16 = 1 << 0;
pub const RTS: u16 = 1 << 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Line {
    pub baud: u32,
    /// 0 one stop bit, 1 one and a half, 2 two.
    pub stop: u8,
    /// 0 none, 1 odd, 2 even, 3 mark, 4 space.
    pub parity: u8,
    /// 5, 6, 7, 8 or 16.
    pub data: u8,
}

/// 115200 8N1, which most devices start at and most consoles expect.
pub const DEFAULT: Line = Line { baud: 115_200, stop: 0, parity: 0, data: 8 };

impl Line {
    pub fn valid(&self) -> bool {
        self.baud > 0 && self.stop <= 2 && self.parity <= 4 && matches!(self.data, 5..=8 | 16)
    }

    pub fn bytes(&self) -> [u8; 7] {
        let b = self.baud.to_le_bytes();
        [b[0], b[1], b[2], b[3], self.stop, self.parity, self.data]
    }

    pub fn from_bytes(b: &[u8]) -> Option<Line> {
        let b: &[u8; 7] = b.get(..7)?.try_into().ok()?;
        let baud = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        Some(Line { baud, stop: b[4], parity: b[5], data: b[6] }).filter(Line::valid)
    }
}

pub fn set_line(comm: u8) -> Setup {
    Setup::class(false, SET_LINE_CODING, 0, comm)
}

/// DTR and RTS as `lines` says (DTR | RTS bits).
pub fn control_lines(comm: u8, lines: u16) -> Setup {
    Setup::class(false, SET_CONTROL_LINE_STATE, lines & (DTR | RTS), comm)
}

/// A break of `ms` milliseconds; 0xFFFF holds it until a break of 0.
pub fn send_break(comm: u8, ms: u16) -> Setup {
    Setup::class(false, SEND_BREAK, ms, comm)
}
