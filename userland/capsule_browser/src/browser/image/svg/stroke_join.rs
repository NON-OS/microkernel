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

use alloc::vec::Vec;

use super::geom::{band, disc, dist, normal, turned};
use super::pen::Pen;

type P = [f32; 2];

/// The join at `b` between segments a-b and b-c: a disc when round, else
/// the bevel triangle on the outer side, grown to the miter point when
/// the join is mitered and the miter stays within the limit.
pub(super) fn join(a: P, b: P, c: P, hw: f32, pen: &Pen, parts: &mut Vec<Vec<P>>) {
    if pen.join == 1 {
        parts.push(disc(b, hw));
        return;
    }
    let (n1, n2) = (normal(a, b, hw), normal(b, c, hw));
    let cross = (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]);
    let s = if cross > 0.0 { -1.0 } else { 1.0 };
    let side = |n: P| [b[0] + s * n[0], b[1] + s * n[1]];
    let (p1, p2) = (side(n1), side(n2));
    let m = [n1[0] + n2[0], n1[1] + n2[1]];
    let mlen = dist([0.0, 0.0], m);
    let reach = if mlen > 1e-6 { 2.0 * hw * hw / mlen } else { f32::MAX };
    if pen.join == 0 && reach / hw <= pen.miter {
        let tip = [b[0] + s * m[0] / mlen * reach, b[1] + s * m[1] / mlen * reach];
        parts.push(turned([b, p1, tip, p2].to_vec()));
    } else {
        parts.push(turned([b, p1, p2].to_vec()));
    }
}

/// The cap at `end` of a line arriving from `from`: nothing for butt, a
/// disc for round, a half-square past the end for square.
pub(super) fn cap(from: P, end: P, hw: f32, kind: u8, parts: &mut Vec<Vec<P>>) {
    let n = normal(from, end, hw);
    match kind {
        1 => parts.push(disc(end, hw)),
        2 => parts.push(band(end, [end[0] + n[1], end[1] - n[0]], n)),
        _ => {}
    }
}
