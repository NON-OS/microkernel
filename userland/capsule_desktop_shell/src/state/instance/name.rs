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

//! The name an instance in a given slot is registered under.

/// Longest service name the shell builds; the kernel caps spawn names at 48.
pub const INSTANCE_NAME_MAX: usize = 48;

/// The name of `base`'s window in `slot`: slot 0 is the base name itself,
/// slot N is "base.N". None when the name would not fit.
pub fn instance_name<'a>(
    base: &[u8],
    slot: u32,
    buf: &'a mut [u8; INSTANCE_NAME_MAX],
) -> Option<&'a [u8]> {
    if base.len() > INSTANCE_NAME_MAX {
        return None;
    }
    buf[..base.len()].copy_from_slice(base);
    if slot == 0 {
        return Some(&buf[..base.len()]);
    }
    let mut digits = [0u8; 10];
    let mut n = 0;
    let mut v = slot;
    while v > 0 {
        digits[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    let end = base.len() + 1 + n;
    if end > INSTANCE_NAME_MAX {
        return None;
    }
    buf[base.len()] = b'.';
    for (i, d) in digits[..n].iter().rev().enumerate() {
        buf[base.len() + 1 + i] = *d;
    }
    Some(&buf[..end])
}
