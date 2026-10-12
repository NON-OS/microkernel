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

//! Fields read out of a report body, least significant bit first (HID 1.11,
//! 5.8). A field that runs past the body has no value, so a short report is
//! never read as zeros.

use crate::descriptors::report::NONE;

/// A report's ID, 0 when the interface uses none, and its body after it.
pub fn split(ids: bool, raw: &[u8]) -> Option<(u8, &[u8])> {
    match (ids, raw.split_first()) {
        (false, _) => Some((0, raw)),
        (true, first) => first.map(|(&id, body)| (id, body)),
    }
}

/// The `size` bits (1 to 32) at bit `at`, or None past the body.
pub fn bits(body: &[u8], at: u16, size: u8) -> Option<u32> {
    let (at, size) = (usize::from(at), usize::from(size));
    if !(1..=32).contains(&size) || at + size > body.len() * 8 {
        return None;
    }
    let mut v = 0u32;
    for k in 0..size {
        let bit = at + k;
        v |= u32::from(body[bit / 8] >> (bit % 8) & 1) << k;
    }
    Some(v)
}

/// The field as a number, sign extended when its logical range goes below
/// zero.
pub fn value(body: &[u8], at: u16, size: u8, min: i64) -> Option<i64> {
    let raw = bits(body, at, size)?;
    if min >= 0 {
        return Some(i64::from(raw));
    }
    let shift = 32 - u32::from(size);
    Some(i64::from(((raw << shift) as i32) >> shift))
}

/// A one-bit field; a field the layout does not have is never set.
pub fn flag(body: &[u8], at: u16) -> Option<bool> {
    if at == NONE {
        return Some(false);
    }
    bits(body, at, 1).map(|b| b != 0)
}

/// Whether any of the `count` slots of `size` bits from `at` holds `v`.
pub fn holds(body: &[u8], at: u16, size: u8, count: u8, min: i64, v: i64) -> Option<bool> {
    let mut found = false;
    for i in 0..u16::from(count) {
        found |= value(body, at + i * u16::from(size), size, min)? == v;
    }
    Some(found)
}
