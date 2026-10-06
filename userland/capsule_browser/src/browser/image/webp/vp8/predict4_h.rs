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

/// The diagonal 4x4 modes horizontal-down (8) and horizontal-up (9).
pub(super) fn pred4_h(mode: u8, t: &[u8; 9], l: &[u8; 4], o: &mut [u8; 16]) {
    let (x, a) = (t[0], |i: usize| t[1 + i]);
    let (i, j, k, ll) = (l[0], l[1], l[2], l[3]);
    let cells: &[(u8, &[(usize, usize)])] = match mode {
        8 => &[
            (avg2(i, x), &[(0, 0), (2, 1)]),
            (avg2(j, i), &[(0, 1), (2, 2)]),
            (avg2(k, j), &[(0, 2), (2, 3)]),
            (avg2(ll, k), &[(0, 3)]),
            (avg3(a(0), a(1), a(2)), &[(3, 0)]),
            (avg3(x, a(0), a(1)), &[(2, 0)]),
            (avg3(i, x, a(0)), &[(1, 0), (3, 1)]),
            (avg3(j, i, x), &[(1, 1), (3, 2)]),
            (avg3(k, j, i), &[(1, 2), (3, 3)]),
            (avg3(ll, k, j), &[(1, 3)]),
        ],
        _ => &[
            (avg2(i, j), &[(0, 0)]),
            (avg2(j, k), &[(2, 0), (0, 1)]),
            (avg2(k, ll), &[(2, 1), (0, 2)]),
            (avg3(i, j, k), &[(1, 0)]),
            (avg3(j, k, ll), &[(3, 0), (1, 1)]),
            (avg3(k, ll, ll), &[(3, 1), (1, 2)]),
            (ll, &[(3, 2), (2, 2), (0, 3), (1, 3), (2, 3), (3, 3)]),
        ],
    };
    for &(v, at) in cells {
        for &(c, r) in at {
            o[r * 4 + c] = v;
        }
    }
}
