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


use super::mixer::over;
use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn blend_px(&mut self, x: u32, y: u32, argb: u32) {
        let a = (argb >> 24) & 0xFF;
        if a == 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.stride_words + x) as usize;
        if idx >= self.pixels.len() {
            return;
        }
        if a == 0xFF {
            self.pixels[idx] = argb | 0xFF00_0000;
            return;
        }
        let dst = self.pixels[idx];
        self.pixels[idx] = over(dst, argb);
    }

    pub fn blend_rect(&mut self, x: u32, y: u32, w: u32, h: u32, argb: u32) {
        let a = (argb >> 24) & 0xFF;
        if a == 0 || w == 0 || h == 0 {
            return;
        }
        if a == 0xFF {
            self.fill_rect(x, y, w, h, argb | 0xFF00_0000);
            return;
        }
        let x1 = x.saturating_add(w).min(self.width);
        let y1 = y.saturating_add(h).min(self.height);
        if x >= x1 {
            return;
        }
        // Each row's span taken once, as blend_px would reach it pixel by
        // pixel: the pixels past the end of the buffer are left alone.
        let stride = self.stride_words as usize;
        for py in y..y1 {
            let at = py as usize * stride;
            let (from, to) = (at + x as usize, (at + x1 as usize).min(self.pixels.len()));
            let Some(row) = self.pixels.get_mut(from..to) else { return };
            for px in row {
                *px = over(*px, argb);
            }
        }
    }
}
