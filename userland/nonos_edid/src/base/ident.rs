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

//! Who made the panel and which one it is (E-EDID 1.4 section 3.4).

use crate::edid::Edid;

impl Edid<'_> {
    /// Bytes 8-9, big-endian: three letters of five bits each, 1 for 'A'
    /// (section 3.4.1). A code outside A to Z reads as '?', so a garbled
    /// ID still prints as three characters.
    pub fn manufacturer(&self) -> [u8; 3] {
        let b = self.base();
        let v = u16::from_be_bytes([b[8], b[9]]);
        let letter = |x: u16| match (x & 0x1F) as u8 {
            n @ 1..=26 => b'@' + n,
            _ => b'?',
        };
        [letter(v >> 10), letter(v >> 5), letter(v)]
    }

    /// Bytes 10-11, little-endian (section 3.4.2).
    pub fn product(&self) -> u16 {
        let b = self.base();
        u16::from_le_bytes([b[10], b[11]])
    }

    /// Bytes 12-15, little-endian; 0 when the maker gives none (3.4.3).
    pub fn serial(&self) -> u32 {
        let b = self.base();
        u32::from_le_bytes([b[12], b[13], b[14], b[15]])
    }

    /// Bytes 18 and 19, the structure's version and revision (3.5).
    pub fn version(&self) -> (u8, u8) {
        let b = self.base();
        (b[18], b[19])
    }

    /// Byte 20 bit 7: a digital input (section 3.6.1).
    pub fn digital(&self) -> bool {
        self.base()[20] & 0x80 != 0
    }

    /// The revision is 1.3 or later, where a standard timing's aspect
    /// code 00 means 16:10 rather than 1:1 (section 3.9.1).
    pub fn at_least_1_3(&self) -> bool {
        let (v, r) = self.version();
        v > 1 || (v == 1 && r >= 3)
    }
}
