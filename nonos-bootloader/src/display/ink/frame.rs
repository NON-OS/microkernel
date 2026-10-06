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

//! The brand's frame: a thin rounded outline in cyan with a soft glow, that
//! can be drawn part way, clockwise from the top centre, as it draws itself
//! in the recording. Integer only.

use super::frame_math::{boundary_distance, perimeter_position};
use crate::display::fx::mix;
use crate::display::gop::{get_pixel, put_pixel};

/// The outline of the box at (x, y), w by h, corner radius r, `stroke` wide,
/// drawn up to `progress` thousandths of its length.
pub fn draw_frame(x: u32, y: u32, w: u32, h: u32, r: u32, stroke: u32, color: u32, progress: u32) {
    let glow = stroke * 6;
    let (x0, y0) = (x.saturating_sub(glow), y.saturating_sub(glow));
    let (cx, cy) = ((2 * x + w) as i64, (2 * y + h) as i64);
    for py in y0..y + h + glow {
        for px in x0..x + w + glow {
            /* Coordinates doubled so the centre lands on whole numbers. */
            let (dx, dy) = (2 * px as i64 + 1 - cx, 2 * py as i64 + 1 - cy);
            if progress < 1000 && perimeter_position(dx, dy) > progress as i64 {
                continue;
            }
            let d16 = boundary_distance(dx, dy, w as i64, h as i64, r as i64);
            let half16 = stroke as i64 * 8;
            let line = (half16 + 8 - d16).clamp(0, 16) * 255 / 16;
            let g = (glow as i64 * 16 - d16).max(0) * 255 / (glow as i64 * 16).max(1);
            let a = line.max(g * g / 255 * 70 / 255) as u32;
            if a > 0 {
                put_pixel(px, py, mix(get_pixel(px, py), color, a));
            }
        }
    }
}
