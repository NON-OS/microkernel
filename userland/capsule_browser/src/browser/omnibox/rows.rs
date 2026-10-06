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

/* Grow the band [y0, y1) of viewport rows until no painted box crosses
 * either edge, so repainting only the band draws each box exactly as a full
 * paint would: text and images draw only whole, so a box cut by a band edge
 * would otherwise go missing. `spans` are the boxes' (top, bottom) rows,
 * shadows and glyph overhang included. The viewport edges stop the growth,
 * as they clip a full paint too. After 16 passes the whole viewport is
 * returned rather than looping on. */
pub fn extend_rows(spans: &[(i32, i32)], y0: i32, y1: i32, view_h: i32) -> (i32, i32) {
    let (mut a, mut b) = (y0.max(0), y1.min(view_h));
    for _ in 0..16 {
        let (mut na, mut nb) = (a, b);
        for &(top, bottom) in spans {
            if top < b && bottom > a {
                na = na.min(top.max(0));
                nb = nb.max(bottom.min(view_h));
            }
        }
        if (na, nb) == (a, b) {
            return (a, b);
        }
        (a, b) = (na, nb);
    }
    (0, view_h)
}
