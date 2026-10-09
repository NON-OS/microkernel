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

//! The frame's geometry: distance to a rounded box's edge, and how far round
//! the edge a point lies.

/// Distance, in 1/16 px, from the point (dx, dy) to the edge of a w by h box
/// with corner radius r centred on the origin. dx and dy are doubled.
pub fn boundary_distance(dx: i64, dy: i64, w: i64, h: i64, r: i64) -> i64 {
    let (qx, qy) = (dx.abs() * 8 - (w - 2 * r) * 8, dy.abs() * 8 - (h - 2 * r) * 8);
    let (ox, oy) = (qx.max(0), qy.max(0));
    let outside = ((ox * ox + oy * oy) as u64).isqrt() as i64 + qx.max(qy).min(0);
    (outside - r * 16).abs()
}

/// How far round the box clockwise from the top centre the point lies, in
/// thousandths. A diamond angle: monotonic, which is all a reveal needs.
pub fn perimeter_position(dx: i64, dy: i64) -> i64 {
    let s = (dx.abs() + dy.abs()).max(1);
    /* 0 at the top, 250 at the right, 500 at the bottom, 750 at the left. */
    let quarter = |a: i64| a * 250 / s;
    match (dx >= 0, dy < 0) {
        (true, true) => quarter(dx),
        (true, false) => 250 + quarter(dy),
        (false, false) => 500 + quarter(-dx),
        (false, true) => 750 + quarter(-dy),
    }
}
