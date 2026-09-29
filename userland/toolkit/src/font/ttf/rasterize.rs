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

use ab_glyph::OutlinedGlyph;

use super::raster::Raster;

/* Coverage for an outlined glyph, one byte per pixel of its box; `bias` is
added before truncating each sample, so 0.5 rounds and 0.0 truncates. */
pub(super) fn rasterize(og: &OutlinedGlyph, bias: f32) -> Option<Raster> {
    let bb = og.px_bounds();
    let (min_x, min_y) = (bb.min.x as i32, bb.min.y as i32);
    let w = (bb.max.x as i32 - min_x).max(0) as u32;
    let h = (bb.max.y as i32 - min_y).max(0) as u32;
    if w == 0 || h == 0 {
        return None;
    }
    let mut cov = alloc::vec![0u8; w as usize * h as usize];
    og.draw(|dx, dy, c| {
        if let Some(p) = cov.get_mut(dy as usize * w as usize + dx as usize) {
            *p = (c * 255.0 + bias) as u8;
        }
    });
    Some(Raster { min_x, min_y, w, h, cov })
}
