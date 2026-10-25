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
//! A report descriptor's short items, one at a time (HID 1.11, 6.2.2.2,
//! as recalled, unverified): a prefix byte whose low two bits give 0, 1, 2
//! or 4 data bytes, little-endian. Long items are stepped over; a cut item
//! ends the walk.

pub struct Item {
    /// The prefix with its size bits cleared: tag and type.
    pub tag: u8,
    pub size: usize,
    pub value: u32,
}

impl Item {
    /// The value as a signed number of its own width.
    pub fn signed(&self) -> i32 {
        match self.size {
            1 => self.value as u8 as i8 as i32,
            2 => self.value as u16 as i16 as i32,
            _ => self.value as i32,
        }
    }
}

const LONG_ITEM: u8 = 0xFE;

pub fn items(report: &[u8]) -> impl Iterator<Item = Item> + '_ {
    let mut i = 0usize;
    core::iter::from_fn(move || loop {
        let prefix = *report.get(i)?;
        if prefix == LONG_ITEM {
            i += 3 + *report.get(i + 1)? as usize;
            continue;
        }
        let size = [0, 1, 2, 4][(prefix & 0x03) as usize];
        let data = report.get(i + 1..i + 1 + size)?;
        i += 1 + size;
        let value = data.iter().rev().fold(0u32, |v, &b| v << 8 | b as u32);
        return Some(Item { tag: prefix & 0xFC, size, value });
    })
}
