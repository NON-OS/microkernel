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

use super::shape::Shape;
use super::sqrt::isqrt;
use super::stops::table;
use super::trig::axis;

/* Largest color table: 64 KiB of ARGB. */
const MAX_LUT: usize = 16384;

/* A gradient set up for one box size. The stop colors are sampled once into
 * `lut` (n + 1 entries, entry i at position i / n along the gradient), with
 * about two entries per pixel of gradient line, so every pixel is an integer
 * table index instead of a float stop search. */
pub(super) struct Painter {
    pub lut: Vec<u32>,
    pub n: usize,
    pub kind: Kind,
}

pub(super) enum Kind {
    /* Table index at (x, y) is y * dy + base (as floats), plus x * step in
     * 16.16 fixed point. */
    Linear { base: f32, dy: f32, step: i64 },
    /* The box size (w, h) and its squared diagonal: pixel (x, y) sits at
     * doubled offset (2x - w, 2y - h) from the centre, radius at the corner. */
    Radial { w: i64, h: i64, r2: u128 },
}

impl Painter {
    pub fn new(shape: &Shape, w: i32, h: i32) -> Painter {
        let (w, h) = (w.max(1), h.max(1));
        match shape {
            Shape::Linear(g) => {
                /* The corners' projections on the axis span the gradient line. */
                let (ux, uy) = axis(g.angle);
                let corners = [(0.0, 0.0), (w as f32, 0.0), (0.0, h as f32), (w as f32, h as f32)];
                let proj = corners.map(|(cx, cy)| cx * ux + cy * uy);
                let lo = proj.iter().copied().fold(f32::MAX, f32::min);
                let span = (proj.iter().copied().fold(f32::MIN, f32::max) - lo).max(1.0);
                /* Two table entries per pixel of gradient line, rounded up. */
                let m = 2.0 * span;
                let n = (m as usize + (m > (m as usize) as f32) as usize).clamp(2, MAX_LUT);
                let k = n as f32 / span;
                let s = ux * k * 65536.0;
                let step = (if s < 0.0 { s - 0.5 } else { s + 0.5 }) as i64;
                let kind = Kind::Linear { base: -lo * k, dy: uy * k, step };
                Painter { lut: table(&g.stops, n), n, kind }
            }
            Shape::Radial(stops) => {
                let (w, h) = (w as i64, h as i64);
                let r2 = ((w * w) as u128 + (h * h) as u128).max(4);
                let n = (isqrt(r2) as usize).clamp(2, MAX_LUT);
                Painter { lut: table(stops, n), n, kind: Kind::Radial { w, h, r2 } }
            }
        }
    }
}
