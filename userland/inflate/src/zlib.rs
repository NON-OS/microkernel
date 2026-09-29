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

//! zlib (RFC 1950): a two-byte header, DEFLATE, and a big-endian Adler-32.

use alloc::vec::Vec;

use super::adler32::adler32;
use super::inflate_raw::raw_partial;
use super::tables::MAX_OUT;
use super::types::{End, Inflated};

/// The whole stream's output, or `None` unless it decodes and checks.
pub fn zlib(data: &[u8]) -> Option<Vec<u8>> {
    zlib_partial(data, MAX_OUT).complete()
}

/// Decodes as much as it can, stopping at `cap` output bytes. A preset
/// dictionary (FDICT) or a window over 32 KiB is not a stream this decodes.
pub fn zlib_partial(data: &[u8], cap: usize) -> Inflated {
    let stop = |end| Inflated { out: Vec::new(), end, used: 0 };
    let (Some(&cmf), Some(&flg)) = (data.first(), data.get(1)) else {
        let bad = data.first().is_some_and(|&c| c & 0x0f != 8 || c >> 4 > 7);
        return stop(if bad { End::Corrupt } else { End::Truncated });
    };
    let check = (u16::from(cmf) << 8 | u16::from(flg)) % 31;
    if cmf & 0x0f != 8 || cmf >> 4 > 7 || check != 0 || flg & 0x20 != 0 {
        return stop(End::Corrupt);
    }
    let mut r = raw_partial(&data[2..], cap);
    r.used += 2;
    if r.end != End::Complete {
        return r;
    }
    let Some(t) = data.get(r.used..r.used + 4) else {
        r.end = End::Truncated;
        r.used = data.len();
        return r;
    };
    if u32::from_be_bytes([t[0], t[1], t[2], t[3]]) != adler32(&r.out) {
        r.end = End::Corrupt;
        return r;
    }
    r.used += 4;
    r
}
