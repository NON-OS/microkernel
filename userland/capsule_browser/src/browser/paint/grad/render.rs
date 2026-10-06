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

use nonos_app_skeleton::PaintBuffer;

use super::composite::over;
use super::painter::{Kind, Painter};
use super::radial::radial_row;

impl Painter {
    /* Row y of the gradient box, from column x0 for out.len() pixels. A
     * linear row steps its table index in 16.16 fixed point; one whose index
     * does not move along x (0 and 180 degrees) is a single color fill. */
    pub fn row(&self, y: i32, x0: i32, out: &mut [u32]) {
        match self.kind {
            Kind::Linear { base, dy, step } => {
                let n = self.n as i64;
                let at = |t: i64| self.lut[((t + 0x8000) >> 16).clamp(0, n) as usize];
                let mut t = ((y as f32 * dy + base) * 65536.0) as i64 + step * x0 as i64;
                if step == 0 {
                    out.fill(at(t));
                    return;
                }
                for o in out.iter_mut() {
                    *o = at(t);
                    t += step;
                }
            }
            Kind::Radial { w, h, r2 } => radial_row(self, [w, h], r2, [x0, y], out),
        }
    }
}

/* Source-over one gradient sample onto the framebuffer. */
pub(crate) fn put_pixel(fb: &mut PaintBuffer, x: i32, y: i32, argb: u32) {
    if x < 0 || y < 0 || x as u32 >= fb.width || y as u32 >= fb.height {
        return;
    }
    let idx = y as usize * fb.stride_words as usize + x as usize;
    let Some(dst) = fb.pixels.get_mut(idx) else { return };
    *dst = over(*dst, argb);
}
