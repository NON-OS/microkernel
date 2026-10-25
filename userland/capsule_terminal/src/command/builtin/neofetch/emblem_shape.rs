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

//! The NONOS emblem `neofetch` draws: a Ø in three dimensions, an image of
//! 40 by 30 pixels drawn two to a cell with half blocks in 24-bit colour. The
//! face is lit from the upper left, its edges caught by the light, and the
//! side falls back to the lower right; every edge is sampled four by four
//! and blended into the theme's background, so it has no stair steps.

/// Width in pixels, and so in cells; height in pixels, two to a row.
pub const EMBLEM_W: usize = 40;
pub const EMBLEM_H: usize = 30;

const CX: f32 = 18.5;
const CY: f32 = 14.0;
/// The ring's radii and thickness, across and down. A half block is about
/// seven pixels wide and ten high, so the figures across are larger.
const RX: f32 = 15.5;
const RY: f32 = 11.5;
const RING_X: f32 = 4.6;
const RING_Y: f32 = 3.4;
/// The slash: its direction (50 degrees above the horizontal), half width,
/// and half length, past the ring at both ends.
const UX: f32 = 0.642_8;
const UY: f32 = -0.766_0;
const SLASH_W: f32 = 1.94;
const SLASH_L: f32 = 15.8;
/// How far the side reaches in pixels down; 1.4 times that across, 45 degrees.
pub const DEPTH: f32 = 3.0;
const ACROSS: f32 = 1.4;

/// Whether a point is on the Ø's face.
pub fn inside(px: f32, py: f32) -> bool {
    let (dx, dy) = (px - CX, py - CY);
    let outer = (dx / RX) * (dx / RX) + (dy / RY) * (dy / RY);
    let (ix, iy) = (RX - RING_X, RY - RING_Y);
    let inner = (dx / ix) * (dx / ix) + (dy / iy) * (dy / iy);
    let along = dx * UX + dy * UY;
    let across = -dx * UY + dy * UX;
    (outer <= 1.0 && inner >= 1.0) || (abs(across) <= SLASH_W && abs(along) <= SLASH_L)
}

fn abs(v: f32) -> f32 {
    if v < 0.0 {
        -v
    } else {
        v
    }
}

/// How much of pixel `(x, y)` the face covers when moved by `shift` down and
/// to the right, in sixteenths.
pub fn cover(x: i32, y: i32, shift: f32) -> u32 {
    let mut n = 0;
    for i in 0..4 {
        for j in 0..4 {
            let px = x as f32 - shift * ACROSS + (i as f32 + 0.5) / 4.0;
            n += u32::from(inside(px, y as f32 - shift + (j as f32 + 0.5) / 4.0));
        }
    }
    n
}
