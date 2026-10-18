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

//! One item of a report descriptor at a time (HID 1.11, 6.2.2.2 and
//! 6.2.2.3): a short item is a prefix byte, its size in bits 1:0, type in
//! 3:2 and tag in 7:4, then 0, 1, 2 or 4 data bytes, least significant
//! first. A long item, prefix 0xFE, carries its own data length and is
//! stepped over, as no long item is defined. An item cut short by the end
//! of the descriptor is an error, never read past.

use super::limits::Refused;

pub const MAIN: u8 = 0;
pub const GLOBAL: u8 = 1;
pub const LOCAL: u8 = 2;
/// The type a long item comes back with: none of the three, so no state
/// takes it.
const LONG: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub kind: u8,
    pub tag: u8,
    /// Data bytes: 0, 1, 2 or 4.
    pub size: u8,
    pub data: u32,
}

impl Item {
    /// The data as a signed number of its own width, as the logical bounds
    /// are given.
    pub fn signed(self) -> i64 {
        match self.size {
            1 => i64::from(self.data as u8 as i8),
            2 => i64::from(self.data as u16 as i16),
            _ => i64::from(self.data as i32),
        }
    }
}

/// The item at `*at`, with `*at` moved past it; Ok(None) at the end.
pub fn next(desc: &[u8], at: &mut usize) -> Result<Option<Item>, Refused> {
    let Some(&prefix) = desc.get(*at) else { return Ok(None) };
    if prefix == 0xFE {
        let len = usize::from(*desc.get(*at + 1).ok_or(Refused)?);
        let end = *at + 3 + len;
        if end > desc.len() {
            return Err(Refused);
        }
        *at = end;
        return Ok(Some(Item { kind: LONG, tag: 0, size: 0, data: 0 }));
    }
    let size = [0usize, 1, 2, 4][usize::from(prefix & 3)];
    let bytes = desc.get(*at + 1..*at + 1 + size).ok_or(Refused)?;
    *at += 1 + size;
    let data = bytes.iter().rev().fold(0u32, |v, &b| v << 8 | u32::from(b));
    Ok(Some(Item { kind: (prefix >> 2) & 3, tag: prefix >> 4, size: size as u8, data }))
}
