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

//! Function 0 of any `_DSM` (section 9.1.1, recalled, unverified): the
//! functions firmware offers for one UUID and revision, bit n for function
//! n. Bit 0 says whether any function is offered at all; with it clear,
//! none is, whatever the other bits say. The specification asks for a
//! Buffer, read little-endian; an Integer is taken too, as some firmware
//! returns one, and anything else offers nothing. No caller asks past
//! function 63, so bytes past the eighth are not read.

use crate::object::Object;

/// The mask function 0 answered, or 0 when it offers nothing.
pub fn offered_mask(answer: Option<&Object>) -> u64 {
    let mask = match answer {
        Some(Object::Buffer(b)) => {
            b.borrow().iter().take(8).rev().fold(0, |m, x| m << 8 | u64::from(*x))
        }
        Some(Object::Integer(n)) => *n,
        _ => 0,
    };
    if mask & 1 != 0 {
        mask
    } else {
        0
    }
}

/// Whether `mask` offers function `f`.
pub fn offers(mask: u64, f: u64) -> bool {
    f < 64 && mask & 1 != 0 && (mask >> f) & 1 != 0
}
