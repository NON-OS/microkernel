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

use super::cache::{Raster, MAX_RASTER_BYTES};
use super::painter::Painter;
use super::shape::Shape;

/* The whole w x h box for `src`, painted on a miss; the caller has checked
 * that it fits under the byte cap. Least recently used boxes make room. */
pub(super) fn raster<'a>(
    rasters: &'a mut Vec<Raster>,
    src: &str,
    g: &Shape,
    w: i32,
    h: i32,
) -> &'a [u32] {
    if let Some(i) = rasters.iter().position(|r| r.1 == w && r.2 == h && r.0 == src) {
        let hit = rasters.remove(i);
        rasters.insert(0, hit);
    } else {
        let p = Painter::new(g, w, h);
        let mut px = alloc::vec![0u32; w as usize * h as usize];
        for (y, row) in px.chunks_exact_mut(w as usize).enumerate() {
            p.row(y as i32, 0, row);
        }
        let need = px.len() * 4;
        while rasters.iter().map(|r| r.3.len() * 4).sum::<usize>() + need > MAX_RASTER_BYTES {
            rasters.pop();
        }
        rasters.insert(0, (src.into(), w, h, px));
    }
    &rasters[0].3
}
