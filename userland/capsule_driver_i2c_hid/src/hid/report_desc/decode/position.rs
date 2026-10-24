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

//! An absolute coordinate, counted from its logical minimum.

use crate::hid::report_desc::read_bits::{read_bits, sign_extend};
use crate::hid::report_desc::Field;

/// Field `f` of `body` as (position, range): read signed when its logical
/// minimum is negative (HID 1.11, 6.2.2.7), less that minimum, and held to
/// 0..=range, where range is the logical maximum less the minimum.
pub(crate) fn position(body: &[u8], f: &Field) -> (u32, i32) {
    let raw = read_bits(body, f.bit_offset, f.bit_size);
    let value = match f.logical_min < 0 {
        true => i64::from(sign_extend(raw, f.bit_size)),
        false => i64::from(raw),
    };
    let (min, max) = (i64::from(f.logical_min), i64::from(f.logical_max));
    let range = (max - min).clamp(0, i64::from(i32::MAX));
    ((value - min).clamp(0, range) as u32, range as i32)
}
