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

//! The items of a report descriptor (HID 1.11, 6.2.2.2): a prefix octet
//! holding the data size (0, 1, 2 or 4 octets), the type and the tag, then
//! the data, little-endian.

use super::error::MapError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Main,
    Global,
    Local,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub kind: Type,
    pub tag: u8,
    pub size: u8,
    pub data: u32,
}

impl Item {
    /// The data as a signed number, extended from its own size.
    pub fn signed(&self) -> i32 {
        match self.size {
            1 => i32::from(self.data as u8 as i8),
            2 => i32::from(self.data as u16 as i16),
            _ => self.data as i32,
        }
    }
}

/// The item at `at` and where the next begins, or `None` at the end.
pub fn item(map: &[u8], at: usize) -> Result<Option<(Item, usize)>, MapError> {
    let Some(&prefix) = map.get(at) else { return Ok(None) };
    let size = [0u8, 1, 2, 4][usize::from(prefix & 0x03)];
    let kind = match prefix >> 2 & 0x03 {
        0 => Type::Main,
        1 => Type::Global,
        2 => Type::Local,
        // Type 3 is reserved, and is how a long item's prefix, 0xFE,
        // reads (HID 1.11, 6.2.2.3).
        _ => return Err(MapError::Unknown),
    };
    let start = at + 1;
    let end = start + usize::from(size);
    let bytes = map.get(start..end).ok_or(MapError::Short)?;
    let data = bytes.iter().rev().fold(0u32, |acc, &b| acc << 8 | u32::from(b));
    Ok(Some((Item { kind, tag: prefix >> 4, size, data }, end)))
}
