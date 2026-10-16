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

//! Report bits: fields are packed least significant bit first, from the
//! first octet on (HID 1.11, 5.8 and 6.2.2.7).

/// `size` bits from bit `at`, or `None` past the end of `data`.
pub fn read(data: &[u8], at: u32, size: u32) -> Option<u32> {
    let mut v = 0u32;
    for i in 0..size {
        let b = at.checked_add(i)?;
        let octet = *data.get(usize::try_from(b / 8).ok()?)?;
        v |= u32::from(octet >> (b % 8) & 1) << i;
    }
    Some(v)
}

/// `v`, `size` bits wide, read as two's complement.
pub fn signed(v: u32, size: u32) -> i64 {
    match size {
        0 => 0,
        1..=31 if v >> (size - 1) & 1 == 1 => i64::from(v) - (1i64 << size),
        32 => i64::from(v as i32),
        _ => i64::from(v),
    }
}

/// Sets bit `at`; a bit past the end of `data` is left alone.
pub fn set(data: &mut [u8], at: u32) {
    if let Some(octet) = usize::try_from(at / 8).ok().and_then(|i| data.get_mut(i)) {
        *octet |= 1 << (at % 8);
    }
}
