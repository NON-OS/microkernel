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

//! The 128 byte blocks an EDID is made of (VESA E-EDID 1.4 section 2.2).

/// Every EDID block, the base and each extension, is this long.
pub const BLOCK: usize = 128;

/// E-EDID 1.4 section 3.3, bytes 0-7 of the base block.
pub const HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

/// E-EDID 1.4 section 3.11: the last byte of a block makes the sum of
/// all 128 bytes zero, modulo 256. Extensions carry the same rule.
pub fn checksum_ok(b: &[u8]) -> bool {
    b.len() == BLOCK && b.iter().fold(0u8, |s, &v| s.wrapping_add(v)) == 0
}

/// Block `i` of `raw`, when all of it is there.
pub fn block(raw: &[u8], i: usize) -> Option<&[u8; BLOCK]> {
    let start = i.checked_mul(BLOCK)?;
    raw.get(start..start.checked_add(BLOCK)?)?.try_into().ok()
}

/// The extension blocks whose checksum holds, in order. A block that
/// fails is passed over so one bad extension does not hide the next.
pub struct Extensions<'a> {
    pub(crate) raw: &'a [u8],
    pub(crate) next: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for Extensions<'a> {
    type Item = &'a [u8; BLOCK];

    fn next(&mut self) -> Option<Self::Item> {
        while self.next <= self.end {
            let i = self.next;
            self.next += 1;
            match block(self.raw, i) {
                Some(b) if checksum_ok(b) => return Some(b),
                Some(_) => continue,
                None => return None,
            }
        }
        None
    }
}
