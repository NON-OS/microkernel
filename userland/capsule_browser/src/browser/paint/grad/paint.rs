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

use alloc::vec;

use nonos_app_skeleton::PaintBuffer;

use super::cache::{shape, CACHE, MAX_RASTER_BYTES};
use super::composite::composite;
use super::painter::Painter;
use super::raster::raster;

/* Paint the gradient `src` over the box [x, y, w, h], source-over, limited
 * to `clip` ([x0, y0, x1, y1), which also keeps it off the browser chrome)
 * and the framebuffer; a box only partly on screen paints the part that
 * shows. A box small enough is rendered once and kept, so a repaint copies
 * it; a larger one renders just its visible rows. Returns false when `src`
 * is not a gradient this module draws. */
pub(crate) fn paint_gradient(
    fb: &mut PaintBuffer,
    src: &str,
    rect: [i32; 4],
    clip: [i32; 4],
) -> bool {
    let mut guard = CACHE.lock();
    let cache = &mut *guard;
    let Some(g) = shape(&mut cache.shapes, src) else { return false };
    let [x, y, w, h] = rect;
    let x0 = x.max(clip[0]).max(0);
    let y0 = y.max(clip[1]).max(0);
    let x1 = (x as i64 + w as i64).min(clip[2] as i64).min(fb.width as i64) as i32;
    let y1 = (y as i64 + h as i64).min(clip[3] as i64).min(fb.height as i64) as i32;
    if w <= 0 || h <= 0 || x0 >= x1 || y0 >= y1 {
        return true;
    }
    let (cols, skip) = ((x1 - x0) as usize, (x0 - x) as usize);
    if (w as usize).saturating_mul(h as usize).saturating_mul(4) <= MAX_RASTER_BYTES {
        let px = raster(&mut cache.rasters, src, g, w, h);
        for py in y0..y1 {
            let at = (py - y) as usize * w as usize + skip;
            composite(fb, py, x0, &px[at..at + cols]);
        }
    } else {
        let p = Painter::new(g, w, h);
        let mut row = vec![0u32; cols];
        for py in y0..y1 {
            p.row(py - y, x0 - x, &mut row);
            composite(fb, py, x0, &row);
        }
    }
    true
}
