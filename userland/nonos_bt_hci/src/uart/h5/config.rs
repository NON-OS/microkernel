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

//! The configuration field (Vol 4, Part D, "Link Establishment"):
//! - bits 0-2, the sliding window size;
//! - bit 3, out-of-frame software flow control;
//! - bit 4, the data integrity check type, set for the 16-bit CRC;
//! - bits 5-7, the version.
//!
//! Each side states what it can do, and the link uses what both can: the
//! smaller window, and flow control or the check only when both state it.
//! A configuration response with no field is a peer that can do neither,
//! with a window of one.

/// What this host offers: a window of four, the CRC, and no out-of-frame
/// flow control, since its ports have hardware flow control or none.
pub const OURS: Agreed = Agreed { window: 4, oof: false, crc: true };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Agreed {
    pub window: u8,
    pub oof: bool,
    pub crc: bool,
}

impl Agreed {
    /// The field stating these, version 0.
    pub fn field(&self) -> u8 {
        self.window & 7 | u8::from(self.oof) << 3 | u8::from(self.crc) << 4
    }

    /// What the link uses, given this side's offer and the peer's field.
    /// A window of zero is not one a link can run, so it counts as one.
    pub fn with(&self, theirs: Option<u8>) -> Agreed {
        let Some(f) = theirs else { return Agreed { window: 1, oof: false, crc: false } };
        Agreed {
            window: self.window.min(f & 7).max(1),
            oof: self.oof && f & 0x08 != 0,
            crc: self.crc && f & 0x10 != 0,
        }
    }
}
