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

/* index gb18030 ranges: (pointer, code point) pairs, u32 little-endian,
ascending, the first at pointer 0. */
static RANGES: &[u8] = include_bytes!("index/gb18030_ranges.idx");

fn range(k: usize) -> (u32, u32) {
    let b = &RANGES[k * 8..k * 8 + 8];
    (u32::from_le_bytes([b[0], b[1], b[2], b[3]]), u32::from_le_bytes([b[4], b[5], b[6], b[7]]))
}

/* The index gb18030 ranges code point for a four-byte pointer: none in
the gaps the standard leaves, U+E7C7 at 7457, else the offset from
the last range starting at or before the pointer. */
pub fn ranges_code_point(pointer: u32) -> Option<u32> {
    if (pointer > 39419 && pointer < 189000) || pointer > 1237575 {
        return None;
    }
    if pointer == 7457 {
        return Some(0xE7C7);
    }
    let (mut lo, mut hi) = (0, RANGES.len() / 8);
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if range(mid).0 <= pointer {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (start, code) = range(lo);
    Some(code + pointer - start)
}
