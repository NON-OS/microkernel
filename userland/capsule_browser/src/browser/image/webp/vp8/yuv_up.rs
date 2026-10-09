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

/// One chroma sample at luma column `x` from rows `t` (above) and `c`
/// (below): the corner cases and diagonals of libwebp's upsampler.
pub(super) fn up(t: &[u8], c: &[u8], x: usize, w: usize, lower: bool) -> i32 {
    let s = |row: &[u8], i: usize| row[i] as i32;
    let edge = |i: usize| {
        if lower {
            (3 * s(c, i) + s(t, i) + 2) >> 2
        } else {
            (3 * s(t, i) + s(c, i) + 2) >> 2
        }
    };
    if x == 0 {
        return edge(0);
    }
    let k = x.div_ceil(2);
    if x == w - 1 && w.is_multiple_of(2) {
        return edge(k - 1);
    }
    let (tl, tt, l, uv) = (s(t, k - 1), s(t, k), s(c, k - 1), s(c, k));
    let avg = tl + tt + l + uv + 8;
    let (d12, d03) = ((avg + 2 * (tt + l)) >> 3, (avg + 2 * (tl + uv)) >> 3);
    match (x % 2 == 1, lower) {
        (true, false) => (d12 + tl) >> 1,
        (false, false) => (d03 + tt) >> 1,
        (true, true) => (d03 + l) >> 1,
        (false, true) => (d12 + uv) >> 1,
    }
}
