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

//! Night light where the display has no colour ramp of its own: every
//! composed pixel of a rectangle looked up through the three ramps, before
//! the indicator band is drawn so the band keeps its true colours.

use crate::state::damage::Rect;
use crate::sw_blitter::Surface;

/// Tint `rect` of `dst` through `ramps` (red, green, blue).
pub fn tint(dst: Surface, rect: Rect, ramps: &[[u8; 256]; 3]) {
    let x1 = rect.x.saturating_add(rect.width).min(dst.width);
    if rect.x >= x1 {
        return;
    }
    let w = x1 - rect.x;
    for y in rect.y..rect.y.saturating_add(rect.height).min(dst.height) {
        let Some(va) = dst.row_start(y, rect.x, w) else { return };
        // SAFETY: `row_start` bounds checked the row for `w` pixels.
        let row = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, w as usize) };
        for px in row.iter_mut() {
            let [b, g, r, a] = px.to_le_bytes();
            let (r, g, b) = (ramps[0][r as usize], ramps[1][g as usize], ramps[2][b as usize]);
            *px = u32::from_le_bytes([b, g, r, a]);
        }
    }
}
