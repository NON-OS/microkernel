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

/* A cursor over the catalogue's bytes, every length checked. */

use alloc::string::String;

pub struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Reader { bytes, at: 0 }
    }

    pub fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.bytes.len())?;
        let out = &self.bytes[self.at..end];
        self.at = end;
        Some(out)
    }

    pub fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }

    pub fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }

    pub fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }

    /* Printable ASCII of the length the prefix gives, at most `max`. */
    pub fn text(&mut self, wide: bool, max: usize) -> Option<String> {
        let n = if wide { self.u16()? as usize } else { self.u8()? as usize };
        let b = self.take(n).filter(|b| n <= max && b.iter().all(|c| (0x20..0x7f).contains(c)))?;
        String::from_utf8(b.to_vec()).ok()
    }

    pub fn is_empty(&self) -> bool {
        self.at == self.bytes.len()
    }
}
