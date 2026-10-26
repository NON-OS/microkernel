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

use super::canvas::Canvas;

pub fn fill_rect(cv: &mut Canvas, x: u32, y: u32, rw: u32, rh: u32, argb: u32) {
    let (buf, spx, w, h) = (&mut *cv.buf, cv.spx, cv.w, cv.h);
    let x1 = (x + rw).min(w);
    for yy in y..(y + rh).min(h) {
        let row = yy as usize * spx;
        for xx in x..x1 {
            let idx = row + xx as usize;
            if idx < buf.len() {
                buf[idx] = argb;
            }
        }
    }
}
