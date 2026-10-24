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
//! A field's value, and what it means: an axis scaled from its logical
//! range to -32767..32767, the hat a direction 0 (up) to 7 clockwise and
//! -1 centred (a 4-way hat gives 0, 2, 4, 6).

use super::layout::Field;

/// Field `f` of a report's data, sign-extended where its range is signed;
/// `None` past the data that came.
pub fn read_field(data: &[u8], f: &Field) -> Option<i32> {
    let (off, size) = (f.bit_offset as usize, f.bit_size as usize);
    if off.checked_add(size)? > data.len() * 8 {
        return None;
    }
    let v =
        (0..size).fold(0u64, |v, b| v | ((data[(off + b) / 8] >> ((off + b) % 8) & 1) as u64) << b);
    let negative = f.min < 0 && size < 64 && v >> (size - 1) & 1 != 0;
    // An unsigned field of 32 bits can hold more than an i32: past it, the
    // value is past any logical maximum, so it saturates there rather
    // than wrapping to a negative one at the range's other end.
    Some(if negative { (v as i64 - (1i64 << size)) as i32 } else { v.min(i32::MAX as u64) as i32 })
}

pub fn scale(v: i32, f: &Field) -> i32 {
    if f.max <= f.min {
        return 0;
    }
    let span = f.max as i64 - f.min as i64;
    ((v.clamp(f.min, f.max) as i64 - f.min as i64) * 65534 / span - 32767) as i32
}

pub fn hat(v: i32, f: &Field) -> i32 {
    // An inverted range holds no value, so the range test alone refuses
    // every value of it; positions is then never 0 or less below.
    let positions = f.max as i64 - f.min as i64 + 1;
    if v < f.min || v > f.max {
        return -1;
    }
    ((v as i64 - f.min as i64) * 8 / positions) as i32
}
