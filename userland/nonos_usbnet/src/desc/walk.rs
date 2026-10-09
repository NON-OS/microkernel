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

//! Walking a configuration descriptor one descriptor at a time. The bytes
//! come from the device, so a length that runs short or past the end stops
//! the walk rather than being trusted.

/// Each descriptor in `raw`, its bLength bytes, in order.
pub struct Walk<'a> {
    raw: &'a [u8],
    pos: usize,
}

pub fn walk(raw: &[u8]) -> Walk<'_> {
    Walk { raw, pos: 0 }
}

impl<'a> Iterator for Walk<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        let len = *self.raw.get(self.pos)? as usize;
        if len < 2 || self.pos + len > self.raw.len() {
            return None;
        }
        let d = &self.raw[self.pos..self.pos + len];
        self.pos += len;
        Some(d)
    }
}
