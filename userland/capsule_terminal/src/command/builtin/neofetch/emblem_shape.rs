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

//! The NONOS emblem: the logo's slashed zero in three dimensions, 34 by 30
//! pixels drawn two to a cell in half blocks. The zero is tall with flattened
//! sides, the slash slim and just past it; edges are sampled four by four.

pub const EMBLEM_W: usize = 34;
pub const EMBLEM_H: usize = 30;

/// A half block is about 7 by 10 screen pixels; the shape is in screen pixels.
const PW: f32 = 7.0;
const PH: f32 = 10.0;
const CX: f32 = 15.5;
const CY: f32 = 14.0;
/// The zero's half width and half height, and its stroke.
const ZW: f32 = 84.0;
const ZH: f32 = 124.0;
const STROKE: f32 = 22.0;
/// The slash: its direction (52 degrees up), half width and half length.
const UX: f32 = 0.615_7;
const UY: f32 = -0.788_0;
const SLASH_W: f32 = 10.0;
const SLASH_L: f32 = 150.0;
/// How far the side reaches in pixels down; 1.4 times that across, 45 degrees.
pub const DEPTH: f32 = 3.0;
const ACROSS: f32 = 1.4;

/// Within the outline halfway between an ellipse and a squircle:
/// 0.45 (u + v) + 0.55 sqrt(u² + v²) <= 1, with no square root taken.
fn within(x: f32, y: f32, a: f32, b: f32) -> bool {
    let (u, v) = ((x / a) * (x / a), (y / b) * (y / b));
    let room = (1.0 - 0.45 * (u + v)) / 0.55;
    room >= 0.0 && u * u + v * v <= room * room
}

/// Whether a point, in the emblem's pixels, is on the face.
pub fn inside(px: f32, py: f32) -> bool {
    let (x, y) = ((px - CX) * PW, (py - CY) * PH);
    let zero = within(x, y, ZW, ZH) && !within(x, y, ZW - STROKE, ZH - STROKE);
    let (along, across) = (x * UX + y * UY, -x * UY + y * UX);
    zero || (across.abs() <= SLASH_W && along.abs() <= SLASH_L)
}

/// How much of pixel `(x, y)` the face covers when moved by `shift` down and
/// to the right, in sixteenths.
pub fn cover(x: i32, y: i32, shift: f32) -> u32 {
    cover_at(x as f32, y as f32, (1.0, 1.0), shift)
}

/// The same for any area: the `size` across and down from `(fx, fy)`, in the
/// emblem's pixels, so the splash can draw it at the screen's own resolution.
pub fn cover_at(fx: f32, fy: f32, size: (f32, f32), shift: f32) -> u32 {
    let mut n = 0;
    for i in 0..4 {
        for j in 0..4 {
            let px = fx - shift * ACROSS + size.0 * (i as f32 + 0.5) / 4.0;
            n += u32::from(inside(px, fy - shift + size.1 * (j as f32 + 0.5) / 4.0));
        }
    }
    n
}
