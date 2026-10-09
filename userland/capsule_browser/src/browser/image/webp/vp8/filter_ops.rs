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

use super::filter_taps::{filter2, sclip1, sclip2};

/// Which filter an edge takes: the simple one, or the normal one on a
/// macroblock edge (six taps) or an inner edge (four taps).
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kind {
    Simple,
    Mb,
    Inner,
}

/// Filter `count` positions across one edge (RFC 6386 15): position i is
/// at `at + i * along`, and `step` crosses the edge. `t` is the edge
/// limit, `it` the interior limit and `hev` the high-variance threshold.
pub(super) fn edge(
    px: &mut [u8],
    (at, step, along): (usize, usize, usize),
    count: usize,
    kind: Kind,
    (t, it, hev): (i32, i32, i32),
) {
    let t2 = 2 * t + 1;
    for i in 0..count {
        let p = at + i * along;
        let g = |k: isize| px[(p as isize + k * step as isize) as usize] as i32;
        let (p3, p2, p1, p0, q0, q1, q2, q3) = (g(-4), g(-3), g(-2), g(-1), g(0), g(1), g(2), g(3));
        if 4 * (p0 - q0).abs() + (p1 - q1).abs() > t2 {
            continue;
        }
        if kind == Kind::Simple {
            filter2(px, p, step, (p1, p0, q0, q1));
            continue;
        }
        let smooth =
            [p3 - p2, p2 - p1, p1 - p0, q3 - q2, q2 - q1, q1 - q0].iter().all(|d| d.abs() <= it);
        if !smooth {
            continue;
        }
        if (p1 - p0).abs() > hev || (q1 - q0).abs() > hev {
            filter2(px, p, step, (p1, p0, q0, q1));
        } else if kind == Kind::Mb {
            let a = sclip1(3 * (q0 - p0) + sclip1(p1 - q1));
            let (a1, a2, a3) = ((27 * a + 63) >> 7, (18 * a + 63) >> 7, (9 * a + 63) >> 7);
            let vals = [p2 + a3, p1 + a2, p0 + a1, q0 - a1, q1 - a2, q2 - a3];
            for (k, v) in (-3isize..3).zip(vals) {
                px[(p as isize + k * step as isize) as usize] = v.clamp(0, 255) as u8;
            }
        } else {
            let a = 3 * (q0 - p0);
            let (a1, a2) = (sclip2((a + 4) >> 3), sclip2((a + 3) >> 3));
            let a3 = (a1 + 1) >> 1;
            let vals = [p1 + a3, p0 + a2, q0 - a1, q1 - a3];
            for (k, v) in (-2isize..2).zip(vals) {
                px[(p as isize + k * step as isize) as usize] = v.clamp(0, 255) as u8;
            }
        }
    }
}
