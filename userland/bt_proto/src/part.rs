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

//! PASSKEY's reply: what pairing asks of the user (Vol 3, Part H, 2.3.5.1).

use super::ops::{PART_CONFIRM, PART_ENTER, PART_NONE, PART_SHOW};

/// The largest passkey: six decimal digits (Vol 3, Part H, 2.3.5.3).
pub const PASSKEY_MAX: u32 = 999_999;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Asked {
    #[default]
    None,
    /// Six digits to type on the device being paired, then Enter there.
    Show(u32),
    /// The six digits the device shows, to be typed here.
    Enter,
    /// Six digits both sides show, to be confirmed the same or not.
    Confirm(u32),
}

impl Asked {
    /// The kind octet and the digits as a little-endian word. Digits over
    /// six places, or digits where the kind carries none, are no reply.
    pub fn read(data: &[u8]) -> Option<Self> {
        let [kind, a, b, c, d] = *data else { return None };
        let v = u32::from_le_bytes([a, b, c, d]);
        match kind {
            PART_NONE if v == 0 => Some(Asked::None),
            PART_SHOW if v <= PASSKEY_MAX => Some(Asked::Show(v)),
            PART_ENTER if v == 0 => Some(Asked::Enter),
            PART_CONFIRM if v <= PASSKEY_MAX => Some(Asked::Confirm(v)),
            _ => None,
        }
    }

    pub fn write(&self) -> [u8; 5] {
        let (kind, v) = match *self {
            Asked::None => (PART_NONE, 0),
            Asked::Show(n) => (PART_SHOW, n),
            Asked::Enter => (PART_ENTER, 0),
            Asked::Confirm(n) => (PART_CONFIRM, n),
        };
        let w = v.to_le_bytes();
        [kind, w[0], w[1], w[2], w[3]]
    }
}
