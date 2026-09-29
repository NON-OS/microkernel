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

//! An FSE table description (RFC 8878 4.1.1): the accuracy log, then one
//! normalized count per symbol, -1 meaning "less than one".

use alloc::vec::Vec;

use super::fwd::Fwd;

/// The accuracy log, the counts, and the bytes the description took.
pub fn counts(d: &[u8], max_log: u8, max_sym: usize) -> Option<(u8, Vec<i16>, usize)> {
    let mut r = Fwd::new(d);
    let log = r.read(4)? as u8 + 5;
    if log > max_log {
        return None;
    }
    let mut norm: Vec<i16> = Vec::new();
    let mut remaining: i32 = (1 << log) + 1;
    let mut threshold: i32 = 1 << log;
    let mut bits = u32::from(log) + 1;
    let mut after_zero = false;
    while remaining > 1 && norm.len() <= max_sym {
        if after_zero {
            let zeros = r.zero_run()?;
            if norm.len() + zeros > max_sym {
                return None;
            }
            norm.resize(norm.len() + zeros, 0);
        }
        let max = (2 * threshold - 1) - remaining;
        let low = r.peek(bits - 1) as i32;
        let value = if low < max {
            r.skip(bits - 1)?;
            low
        } else {
            let v = r.read(bits)? as i32;
            if v >= threshold {
                v - max
            } else {
                v
            }
        };
        let count = value - 1;
        remaining -= count.abs();
        if remaining < 1 {
            return None;
        }
        norm.push(count as i16);
        after_zero = count == 0;
        while remaining < threshold {
            bits -= 1;
            threshold >>= 1;
        }
    }
    (remaining == 1).then(|| (log, norm, r.bytes()))
}
