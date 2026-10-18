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

//! The explicit integer conversions: ToInteger, ToBCD and FromBCD.

use super::hex::hex_value;
use super::{Object, Width};

impl Object {
    /// ToInteger (section 19.6.140): a string is read as hexadecimal after
    /// "0x" or "0X" and as decimal otherwise, up to the first character that
    /// is not a digit of its base; a buffer is read as Buffer to Integer.
    pub fn to_integer_explicit(&self, w: Width) -> Option<u64> {
        let Object::String(s) = self else { return self.as_integer(w) };
        let s = s.borrow();
        let (digits, base) = match s.as_slice() {
            [b'0', b'x' | b'X', rest @ ..] => (rest, 16),
            all => (all, 10),
        };
        let mut v = 0u64;
        for d in digits.iter().map_while(|c| hex_value(*c).filter(|d| *d < base)) {
            v = v.checked_mul(base)?.checked_add(d)?;
        }
        Some(w.mask(v))
    }
}

/// ToBCD (section 19.6.137): each decimal digit into a nibble, least
/// significant first; digits past the width are lost.
pub fn to_bcd(mut v: u64, w: Width) -> u64 {
    let mut out = 0u64;
    for i in 0..(w.bytes() * 2) as u32 {
        out |= (v % 10) << (4 * i);
        v /= 10;
    }
    out
}

/// FromBCD (section 19.6.51): None if a nibble is not a decimal digit.
pub fn from_bcd(v: u64, w: Width) -> Option<u64> {
    let mut out = 0u64;
    for i in (0..(w.bytes() * 2) as u32).rev() {
        let d = (v >> (4 * i)) & 0xF;
        if d > 9 {
            return None;
        }
        out = out * 10 + d;
    }
    Some(w.mask(out))
}
