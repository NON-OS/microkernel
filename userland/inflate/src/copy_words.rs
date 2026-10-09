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

//! LZ77 match copies inside the output window.

/// Copies `len` bytes from `dist` back to `pos`, where `buf` has at least
/// `len + 16` bytes past `pos`. From a word back, whole words are copied,
/// each reading only bytes already written; a run of one byte is a fill.
#[inline(always)]
pub fn copy_match(buf: &mut [u8], pos: usize, dist: usize, len: usize) {
    let b = &mut buf[pos - dist..pos + len + 16];
    if dist >= 16 {
        words::<16>(b, dist, len);
    } else if dist >= 8 {
        words::<8>(b, dist, len);
    } else if dist == 1 {
        let v = b[0];
        b[1..=len].fill(v);
    } else {
        for i in 0..len {
            b[dist + i] = b[i];
        }
    }
}

/// `W`-byte words from the start of `b` to `dist` on, `W <= dist`.
#[inline(always)]
fn words<const W: usize>(b: &mut [u8], dist: usize, len: usize) {
    let mut i = 0;
    while i < len {
        let mut w = [0u8; W];
        w.copy_from_slice(&b[i..i + W]);
        b[dist + i..dist + i + W].copy_from_slice(&w);
        i += W;
    }
}
