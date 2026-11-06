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


use super::radius;
use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn fill_round(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, argb: u32) {
        self.fill_round_clipped(x, i64::from(y), w, h, r, argb);
    }

    /// The same shape with its top at `top`, which may lie above the buffer:
    /// only the rows inside the buffer are painted, so a shape cut by an edge
    /// keeps its own corners rather than gaining new ones there.
    pub fn fill_round_clipped(&mut self, x: u32, top: i64, w: u32, h: u32, r: u32, argb: u32) {
        if w == 0 || h == 0 {
            return;
        }
        let r = radius::clamp_radius(w, h, r);
        let a = (argb >> 24) & 0xFF;
        let rgb = argb & 0x00FF_FFFF;
        for row in 0..h {
            let Some(y) = self.row_at(top, row) else { continue };
            if r == 0 || (row >= r && row < h - r) {
                self.blend_rect(x, y, w, 1, argb);
                continue;
            }
            for col in 0..w {
                let cov = radius::coverage(col, row, w, h, r);
                if cov == 0 {
                    continue;
                }
                let ea = a * cov / 255;
                self.blend_px(x + col, y, (ea << 24) | rgb);
            }
        }
    }

    /// The buffer row that row `row` of a shape topped at `top` lands on, or
    /// None when it falls outside the buffer.
    pub(super) fn row_at(&self, top: i64, row: u32) -> Option<u32> {
        let y = top + i64::from(row);
        (y >= 0 && y < i64::from(self.height)).then(|| y as u32)
    }
}
