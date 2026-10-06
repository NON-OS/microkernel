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

//! One short item of a report descriptor at a time (HID 1.11, 6.2.2.2):
//! the prefix byte gives the size, type and tag, and up to four data bytes
//! follow in little-endian order. Long items (prefix 0xFE) are stepped over,
//! since none of them is needed. A truncated item ends the walk, so a
//! malformed descriptor can only ever yield fewer items, never read past it.

use super::read_le::read_le;

/// The item types in prefix bits 3:2.
pub(super) const TYPE_MAIN: u8 = 0;
pub(super) const TYPE_GLOBAL: u8 = 1;
pub(super) const TYPE_LOCAL: u8 = 2;

pub(super) struct Item {
    pub btype: u8,
    pub tag: u8,
    pub data: u32,
}

/// The item starting at `*i`, moving `*i` past it; `None` at the end of the
/// descriptor or at an item cut short by it.
pub(super) fn next(desc: &[u8], i: &mut usize) -> Option<Item> {
    while *i < desc.len() {
        let b0 = desc[*i];
        *i += 1;
        if b0 == 0xFE {
            // Long item: [size][tag][data...].
            let &dsize = desc.get(*i)?;
            *i = i.saturating_add(2 + dsize as usize);
            continue;
        }
        let size = match b0 & 0x03 {
            3 => 4,
            k => k as usize,
        };
        if *i + size > desc.len() {
            return None;
        }
        let data = read_le(&desc[*i..*i + size]);
        *i += size;
        return Some(Item { btype: (b0 >> 2) & 0x03, tag: (b0 >> 4) & 0x0F, data });
    }
    None
}
