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

//! The communications interface's notifications (CDC 1.2, 6.3, as recalled,
//! unverified): an 8-byte header shaped like a setup packet, bmRequestType
//! A1h, bNotification, wValue, wIndex naming the interface, and wLength
//! bytes after it. NETWORK_CONNECTION (00h) carries none and wValue 0 or
//! 1; CONNECTION_SPEED_CHANGE (2Ah) carries 8, two 32-bit bit rates.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Notice {
    Link(bool),
    Speed,
    /// Another notification, well formed, that this driver has no use for.
    Other,
}

const FROM_INTERFACE: u8 = 0xA1;
const NETWORK_CONNECTION: u8 = 0x00;
const SPEED_CHANGE: u8 = 0x2A;

/// What `n` says to interface `comm`; `None` when it is not as long as its
/// own header says, names another interface, or breaks its kind's shape.
pub fn notice(n: &[u8], comm: u8) -> Option<Notice> {
    let h = n.get(..8)?;
    let field = |at: usize| u16::from_le_bytes([h[at], h[at + 1]]);
    let (value, index, len) = (field(2), field(4), field(6) as usize);
    if h[0] != FROM_INTERFACE || index != comm as u16 || n.len() != 8 + len {
        return None;
    }
    match (h[1], len) {
        (NETWORK_CONNECTION, 0) if value <= 1 => Some(Notice::Link(value == 1)),
        (SPEED_CHANGE, 8) => Some(Notice::Speed),
        (NETWORK_CONNECTION | SPEED_CHANGE, _) => None,
        _ => Some(Notice::Other),
    }
}
