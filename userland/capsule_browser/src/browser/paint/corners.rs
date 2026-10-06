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

/// The corner radii of a `w` x `h` box, each at most half the shorter side.
pub(super) fn corners(radius: [u16; 4], w: i32, h: i32) -> [i32; 4] {
    let lim = (w / 2).min(h / 2).max(0);
    radius.map(|v| (v as i32).min(lim))
}

/* How far in from the side row `row` of a corner of radius `r` starts: the
 * widest span whose pixel centres stay inside the circle. */
pub(super) fn inset(r: i32, row: i32) -> i32 {
    if row >= r {
        return 0;
    }
    let (dy, rr) = ((r - row - 1) as i64, r as i64);
    r - isqrt(rr * rr - dy * dy) as i32
}

/* Floor of the square root, by Newton's method. */
fn isqrt(v: i64) -> i64 {
    let (mut r, mut next) = (v.max(0), (v.max(0) + 1) / 2);
    while next < r {
        r = next;
        next = (r + v / r) / 2;
    }
    r
}
