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

//! The logo ready to show: already turned and sized for the surface, so each
//! repaint of the splash is a clipped copy of rows.

use alloc::vec::Vec;

use super::place::Spot;

pub struct Shown {
    pub spot: Spot,
    /// `spot.w` by `spot.h`, 0xAARRGGBB, top row first.
    pub pixels: Vec<u32>,
}

impl Shown {
    /// Copy into a surface of `w` by `h` pixels, `spx` pixels a row; what
    /// falls past an edge is left out.
    pub fn draw(&self, buf: &mut [u32], spx: usize, w: u32, h: u32) {
        let s = self.spot;
        let x0 = s.x.max(0);
        let x1 = (s.x + s.w as i64).min(w as i64);
        if x0 >= x1 {
            return;
        }
        let y0 = s.y.max(0);
        let y1 = (s.y + s.h as i64).min(h as i64);
        let n = (x1 - x0) as usize;
        for y in y0..y1 {
            let from = (y - s.y) as usize * s.w as usize + (x0 - s.x) as usize;
            let to = y as usize * spx + x0 as usize;
            if let (Some(d), Some(f)) = (buf.get_mut(to..to + n), self.pixels.get(from..from + n)) {
                d.copy_from_slice(f);
            }
        }
    }
}
