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

//! A colour filter's matrix over a rectangle: each pixel's levels to linear
//! light, mixed by the matrix in fixed point, clamped, and back to levels.
//! Alpha is kept.

use super::look::ONE;
use crate::display::srgb_rule::LIN_MAX;
use crate::state::damage::Rect;
use crate::sw_blitter::Surface;

pub fn filter(dst: Surface, rect: Rect, m: &[[i32; 3]; 3], lin: &[u16; 256], srgb: &[u8]) {
    if srgb.len() != LIN_MAX as usize + 1 {
        return;
    }
    let x1 = rect.x.saturating_add(rect.width).min(dst.width);
    if rect.x >= x1 {
        return;
    }
    let w = x1 - rect.x;
    let mix = |row: &[i32; 3], l: [i32; 3]| {
        let v = (row[0] * l[0] + row[1] * l[1] + row[2] * l[2] + ONE / 2) >> 12;
        srgb[v.clamp(0, LIN_MAX as i32) as usize]
    };
    for y in rect.y..rect.y.saturating_add(rect.height).min(dst.height) {
        let Some(va) = dst.row_start(y, rect.x, w) else { return };
        // SAFETY: `row_start` bounds checked the row for `w` pixels.
        let row = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, w as usize) };
        for px in row.iter_mut() {
            let [b, g, r, a] = px.to_le_bytes();
            let l = [lin[r as usize] as i32, lin[g as usize] as i32, lin[b as usize] as i32];
            *px = u32::from_le_bytes([mix(&m[2], l), mix(&m[1], l), mix(&m[0], l), a]);
        }
    }
}
