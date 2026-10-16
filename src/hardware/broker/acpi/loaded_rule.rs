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

//! A table AML loads at run time (F-93), checked before it is measured, and
//! the list grown by it, pure. Its record's instance carries LOADED, so it
//! never reads as a table the firmware's root lists; the same bytes loaded
//! again add nothing, and a full list refuses rather than drop one.

use super::table_measure::Measured;
use alloc::vec::Vec;

/// Larger than any table AML loads; past it the copy is refused.
pub const MAX_LOADED: usize = 1 << 20;
/// The instance of every run-time load's record.
pub const LOADED: u32 = 1 << 31;
const HEADER: usize = 36;

/// The table's signature when the bytes are one whole table: a header, the
/// header's length (offset 4) equal to what was handed, and every byte
/// summing to zero.
pub fn checked(bytes: &[u8]) -> Option<[u8; 4]> {
    let head = bytes.get(..HEADER)?;
    let len = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let sum = bytes.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    (len == bytes.len() && sum == 0).then(|| [head[0], head[1], head[2], head[3]])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admit {
    Added,
    /// The same table with the same bytes is listed already.
    Known,
    /// The list holds `max` records; nothing was added.
    Full,
}

pub fn admit(list: &mut Vec<Measured>, m: Measured, max: usize) -> Admit {
    if list.contains(&m) {
        Admit::Known
    } else if list.len() >= max {
        Admit::Full
    } else {
        list.push(m);
        Admit::Added
    }
}
