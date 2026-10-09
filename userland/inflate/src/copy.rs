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

//! Matches in the fast loop, where 56 bits and `SLACK` output are assured.

use super::bits::Bits;
use super::copy_words::copy_match;
use super::huff::{Table, KIND, LIT};
use super::types::End;

/// Completes the length whose code gave entry `e`: its extra bits, the
/// distance code and its extra bits, then the copy. Returns the length.
#[inline(always)]
pub fn matched(
    s: &mut Bits,
    dist: &Table<256>,
    buf: &mut [u8],
    pos: usize,
    e: u32,
) -> Result<usize, End> {
    let len = (e >> 16) as usize + take(s, e);
    let d = dist.lookup(s.buf);
    if d & KIND != LIT || d & 0xFF == 0 {
        return Err(End::Corrupt);
    }
    s.drop_bits(d & 0xFF);
    let d = (d >> 16) as usize + take(s, d);
    if d > pos {
        return Err(End::Corrupt);
    }
    copy_match(buf, pos, d, len);
    Ok(len)
}

/// The extra bits of entry `e`.
#[inline(always)]
fn take(s: &mut Bits, e: u32) -> usize {
    let n = (e >> 8) & 0xF;
    let v = (s.buf & ((1u64 << n) - 1)) as usize;
    s.drop_bits(n);
    v
}
