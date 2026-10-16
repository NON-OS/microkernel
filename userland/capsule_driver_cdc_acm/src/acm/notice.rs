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
//! What the device says on its notification endpoint (CDC PSTN 1.2,
//! 6.5.4, as recalled, unverified): an 8-byte header shaped like a setup
//! packet, then its data. SERIAL_STATE's two bytes hold the lines the
//! device sees and the errors since it last said.

const FROM_INTERFACE_CLASS: u8 = 0xA1;
const SERIAL_STATE: u8 = 0x20;

/// UART state bits: carrier (DCD), DSR, a break seen, ring, and framing,
/// parity and overrun errors. The first two follow the line; the rest are
/// events, kept until the client reads them.
pub const DCD: u16 = 1 << 0;
pub const DSR: u16 = 1 << 1;
pub const LEVELS: u16 = DCD | DSR;

/// The state a SERIAL_STATE notification in `n` carries, when it is about
/// communications interface `comm` (its wIndex).
pub fn serial_state(n: &[u8], comm: u8) -> Option<u16> {
    let head = n.get(..8)?;
    let index = u16::from_le_bytes([head[4], head[5]]);
    let length = u16::from_le_bytes([head[6], head[7]]);
    if head[0] != FROM_INTERFACE_CLASS || head[1] != SERIAL_STATE || length < 2 {
        return None;
    }
    if index != u16::from(comm) {
        return None;
    }
    Some(u16::from_le_bytes([*n.get(8)?, *n.get(9)?]) & 0x7F)
}

/// The state a client reads: the lines as they stand, and every event
/// since it last read.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct State {
    pub levels: u16,
    pub events: u16,
}

impl State {
    pub fn seen(&mut self, s: u16) {
        self.levels = s & LEVELS;
        self.events |= s & !LEVELS;
    }

    /// The state as it reads now, and the events forgotten.
    pub fn take(&mut self) -> u16 {
        let s = self.levels | self.events;
        self.events = 0;
        s
    }
}
