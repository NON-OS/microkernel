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

//! The accesses a field takes: one per access-width unit it touches.

/// One access: the unit's byte offset in the region, and which of its bits
/// belong to the field and where they go in the field's value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    /// Byte offset of the unit, a multiple of the width.
    pub byte: u64,
    /// First field bit inside the unit.
    pub shift: u32,
    /// How many field bits the unit holds.
    pub bits: u32,
    /// Where those bits start in the field's value.
    pub at: u32,
}

/// The units a field of `bit_len` bits at `bit_offset` touches, at a width
/// of `width` bytes, in rising address order.
pub fn plan(bit_offset: u32, bit_len: u32, width: u32) -> impl Iterator<Item = Unit> {
    let unit_bits = (width * 8) as u64;
    let (start, end) = (bit_offset as u64, bit_offset as u64 + bit_len as u64);
    let first = start / unit_bits;
    let last = if bit_len == 0 { first } else { (end - 1) / unit_bits + 1 };
    (first..last).map(move |u| {
        let lo = (u * unit_bits).max(start);
        let hi = ((u + 1) * unit_bits).min(end);
        Unit {
            byte: u * unit_bits / 8,
            shift: (lo - u * unit_bits) as u32,
            bits: (hi - lo) as u32,
            at: (lo - start) as u32,
        }
    })
}
