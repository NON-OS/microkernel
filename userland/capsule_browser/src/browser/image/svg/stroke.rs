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

use super::brush::Brush;
use super::dash::dashed;
use super::fill::fill_polys;
use super::geom::{band, disc, dist, normal};
use super::pen::Pen;
use super::raster::Raster;
use super::stroke_join::{cap, join};

type P = [f32; 2];

/* Stroke polylines already in device space with a device-space pen: each
 * dash of each subpath becomes a quad per segment plus its joins and caps,
 * all wound the same way and filled once with the nonzero rule, so their
 * union paints and overlaps never double a translucent stroke. */
pub(super) fn stroke_polys(r: &mut Raster, polys: &[Vec<P>], brush: &Brush, pen: &Pen) {
    let hw = (pen.width / 2.0).max(0.35);
    let mut parts: Vec<Vec<P>> = Vec::new();
    for poly in polys {
        let closed = poly.len() > 2 && dist(poly[0], poly[poly.len() - 1]) < 1e-3;
        for line in dashed(poly, pen) {
            outline(&line, hw, pen, closed && pen.dashes == 0, &mut parts);
        }
    }
    fill_polys(r, &parts, brush, false);
}

fn outline(line: &[P], hw: f32, pen: &Pen, closed: bool, parts: &mut Vec<Vec<P>>) {
    let mut pts: Vec<P> = Vec::with_capacity(line.len());
    for &p in line {
        if pts.last().is_none_or(|q| dist(*q, p) > 1e-4) {
            pts.push(p);
        }
    }
    let n = pts.len();
    if n < 2 {
        /* A zero-length subpath still shows its round or square cap. */
        match (pts.first(), pen.cap) {
            (Some(&p), 1) => parts.push(disc(p, hw)),
            (Some(&p), 2) => parts.push(band([p[0] - hw, p[1]], [p[0] + hw, p[1]], [0.0, hw])),
            _ => {}
        }
        return;
    }
    for w in pts.windows(2) {
        parts.push(band(w[0], w[1], normal(w[0], w[1], hw)));
    }
    for i in 1..n - 1 {
        join(pts[i - 1], pts[i], pts[i + 1], hw, pen, parts);
    }
    if closed {
        join(pts[n - 2], pts[0], pts[1], hw, pen, parts);
    } else {
        cap(pts[1], pts[0], hw, pen.cap, parts);
        cap(pts[n - 2], pts[n - 1], hw, pen.cap, parts);
    }
}
