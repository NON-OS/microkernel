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

use super::avg::{avg2, avg3};

/// The diagonal 4x4 modes vertical-right (5) and vertical-left (7), and
/// through pred4_h the horizontal ones; each is a list of values and the
/// (column, row) cells that take them.
pub(super) fn pred4_ext(mode: u8, t: &[u8; 9], l: &[u8; 4], o: &mut [u8; 16]) {
    let (x, a) = (t[0], |i: usize| t[1 + i]);
    let (i, j, k) = (l[0], l[1], l[2]);
    let cells: &[(u8, &[(usize, usize)])] = match mode {
        5 => &[
            (avg2(x, a(0)), &[(0, 0), (1, 2)]),
            (avg2(a(0), a(1)), &[(1, 0), (2, 2)]),
            (avg2(a(1), a(2)), &[(2, 0), (3, 2)]),
            (avg2(a(2), a(3)), &[(3, 0)]),
            (avg3(k, j, i), &[(0, 3)]),
            (avg3(j, i, x), &[(0, 2)]),
            (avg3(i, x, a(0)), &[(0, 1), (1, 3)]),
            (avg3(x, a(0), a(1)), &[(1, 1), (2, 3)]),
            (avg3(a(0), a(1), a(2)), &[(2, 1), (3, 3)]),
            (avg3(a(1), a(2), a(3)), &[(3, 1)]),
        ],
        7 => &[
            (avg2(a(0), a(1)), &[(0, 0)]),
            (avg2(a(1), a(2)), &[(1, 0), (0, 2)]),
            (avg2(a(2), a(3)), &[(2, 0), (1, 2)]),
            (avg2(a(3), a(4)), &[(3, 0), (2, 2)]),
            (avg3(a(0), a(1), a(2)), &[(0, 1)]),
            (avg3(a(1), a(2), a(3)), &[(1, 1), (0, 3)]),
            (avg3(a(2), a(3), a(4)), &[(2, 1), (1, 3)]),
            (avg3(a(3), a(4), a(5)), &[(3, 1), (2, 3)]),
            (avg3(a(4), a(5), a(6)), &[(3, 2)]),
            (avg3(a(5), a(6), a(7)), &[(3, 3)]),
        ],
        _ => return super::predict4_h::pred4_h(mode, t, l, o),
    };
    for &(v, at) in cells {
        for &(c, r) in at {
            o[r * 4 + c] = v;
        }
    }
}
