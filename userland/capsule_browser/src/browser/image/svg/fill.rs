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
use super::raster::Raster;

type P = [f32; 2];

/* Scanline fill of a set of subpaths already in device coordinates, over
 * the rows of the raster's band. Each subpath closes implicitly. Crossings
 * at the sample row carry a winding direction so both fill rules evaluate
 * from the same list. */
pub(super) fn fill_polys(r: &mut Raster, polys: &[Vec<P>], brush: &Brush, evenodd: bool) {
    let (mut y_min, mut y_max) = (f32::MAX, f32::MIN);
    for p in polys.iter().flatten() {
        (y_min, y_max) = (y_min.min(p[1]), y_max.max(p[1]));
    }
    if y_min > y_max {
        return;
    }
    let (band0, band1) = r.rows();
    let y0 = (y_min as i32).max(band0);
    let y1 = ((y_max + 1.0) as i32).min(band1);
    let mut xs: Vec<(f32, i32)> = Vec::new();
    for y in y0..y1 {
        let sy = y as f32 + 0.5;
        xs.clear();
        for poly in polys {
            let n = poly.len();
            for i in 0..n {
                let (a, b) = (poly[i], poly[(i + 1) % n]);
                if (a[1] <= sy) == (b[1] <= sy) {
                    continue;
                }
                let t = (sy - a[1]) / (b[1] - a[1]);
                xs.push((a[0] + t * (b[0] - a[0]), if b[1] > a[1] { 1 } else { -1 }));
            }
        }
        xs.sort_by(|p, q| p.0.total_cmp(&q.0));
        let (mut wind, mut inside, mut start) = (0i32, false, 0f32);
        for &(x, dir) in xs.iter() {
            let was = inside;
            wind += dir;
            inside = if evenodd { !was } else { wind != 0 };
            if !was && inside {
                start = x;
            } else if was && !inside {
                let (x0, x1) = ((start + 0.5) as i32, (x + 0.5) as i32);
                for px in x0.max(0)..x1.min(r.w as i32) {
                    if let Some(c) = brush.at(px, y) {
                        r.blend(px, y, c);
                    }
                }
            }
        }
    }
}
