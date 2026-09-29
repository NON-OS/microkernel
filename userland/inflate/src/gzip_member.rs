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

//! One gzip member: header, deflate stream, CRC-32 and ISIZE trailer.

use alloc::vec::Vec;

use super::crc32::crc32;
use super::gzip_header::body_at;
use super::inflate_raw::run;
use super::types::{End, Inflated};

/// A member ends with a CRC32 and an ISIZE.
const TRAILER: usize = 8;

/// The member at the start of `d`, its output capped at `cap`. `used`
/// covers the trailer only when the member is complete.
pub(super) fn member(d: &[u8], cap: usize) -> Inflated {
    let start = match body_at(d) {
        Ok(s) => s,
        Err(end) => return Inflated { out: Vec::new(), end, used: 0 },
    };
    let mut r = run(&d[start..], cap, size_hint(d));
    r.used += start;
    if r.end != End::Complete {
        return r;
    }
    let Some(t) = d.get(r.used..r.used + TRAILER) else {
        r.end = End::Truncated;
        r.used = d.len();
        return r;
    };
    let word = |i: usize| u32::from_le_bytes([t[i], t[i + 1], t[i + 2], t[i + 3]]);
    if word(0) != crc32(&r.out) || word(4) != r.out.len() as u32 {
        r.end = End::Corrupt;
        return r;
    }
    r.used += TRAILER;
    r
}

/// ISIZE of the last member in `d`, which for the usual single member is
/// this one's size, bounded by the most DEFLATE can expand the input.
fn size_hint(d: &[u8]) -> usize {
    let t = &d[d.len().saturating_sub(4)..];
    let size = t.iter().rev().fold(0usize, |v, &b| v << 8 | usize::from(b));
    size.min(d.len().saturating_mul(1032))
}
