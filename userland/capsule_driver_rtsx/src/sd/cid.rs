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

//! The card's identity, its CID (SD Physical Layer 5.2): what tells one
//! card from another of the same size, so a card swapped into the slot is
//! never served as the one before it. The product serial (PSN) is told to
//! the installer's erase confirmation and to nothing else (C5): Debug
//! prints none of the CID, and no log line carries it. Pure.

use super::bits::unstuff;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cid([u32; 4]);

impl core::fmt::Debug for Cid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Cid(..)")
    }
}

impl Cid {
    /// The R2 words as the response parser gives them, word 0 the highest.
    pub const fn new(words: [u32; 4]) -> Self {
        Self(words)
    }

    /// PNM, bits 103:64: five ASCII characters, each one that is not
    /// printable given as '?', so no control byte reaches a screen.
    pub fn product(&self) -> [u8; 5] {
        let mut out = [0u8; 5];
        for (i, b) in out.iter_mut().enumerate() {
            let c = unstuff(&self.0, 96 - 8 * i as u32, 8) as u8;
            *b = if c.is_ascii_graphic() || c == b' ' { c } else { b'?' };
        }
        out
    }

    /// PSN, bits 55:24, as eight upper-case hex digits.
    pub fn serial(&self) -> [u8; 8] {
        let psn = unstuff(&self.0, 24, 32);
        let mut out = [0u8; 8];
        for (i, d) in out.iter_mut().enumerate() {
            *d = b"0123456789ABCDEF"[(psn >> (28 - 4 * i)) as usize & 0xF];
        }
        out
    }
}
