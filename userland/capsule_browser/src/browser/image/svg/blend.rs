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

use super::raster::Raster;

impl Raster {
    /// Composite an ARGB colour over one device pixel.
    pub fn blend(&mut self, x: i32, y: i32, color: u32) {
        let Some(idx) = self.index(x, y) else { return };
        let dst = &mut self.px[idx];
        let sa = color >> 24;
        if sa == 0xFF {
            *dst = color;
            return;
        }
        if sa == 0 {
            return;
        }
        /* Straight-alpha source-over: colours weighted by their alphas. */
        let (inv, da) = (255 - sa, *dst >> 24);
        let a = sa + da * inv / 255;
        let ch = |s: u32, d: u32| (s * sa * 255 + d * da * inv) / (a * 255);
        let r = ch((color >> 16) & 0xFF, (*dst >> 16) & 0xFF);
        let g = ch((color >> 8) & 0xFF, (*dst >> 8) & 0xFF);
        let b = ch(color & 0xFF, *dst & 0xFF);
        *dst = a << 24 | r << 16 | g << 8 | b;
    }
}
