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

//! The arrow drawn into a primary's cursor slot for the device's cursor
//! plane to show: 64 x 64 ARGB8888, transparent around the arrow, each
//! arrow pixel a square of `scale` screen pixels so the plane's arrow is the
//! size the composited one would be on that screen.

use super::cursor::{ARROW, FILL, OUTLINE};

/// The slot's side in pixels and its bytes (`nonos_gfx_wire` CURSOR).
pub const SIDE: usize = 64;
pub const SLOT: usize = SIDE * SIDE * 4;

/// The largest square per arrow pixel that keeps the 19-row arrow inside
/// the slot; a screen scaled further shows a smaller arrow, not a cut one.
const MAX_SCALE: usize = SIDE / 19;

/// The drawn arrow's width and height at `scale`.
pub fn shape_side(scale: u32) -> (u16, u16) {
    let s = (scale as usize).clamp(1, MAX_SCALE);
    let w = ARROW.iter().map(|l| l.len()).max().unwrap_or(0);
    ((w * s) as u16, (ARROW.len() * s) as u16)
}

/// Draw the arrow into `slot`; None when the slot is short.
pub fn draw_shape(slot: &mut [u8], scale: u32) -> Option<()> {
    let slot = slot.get_mut(..SLOT)?;
    slot.fill(0);
    let s = (scale as usize).clamp(1, MAX_SCALE);
    for (row, line) in ARROW.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            let argb = match pixel {
                b'X' => OUTLINE,
                b'o' => FILL,
                _ => continue,
            };
            for y in row * s..(row + 1) * s {
                for x in col * s..(col + 1) * s {
                    let at = (y * SIDE + x) * 4;
                    slot[at..at + 4].copy_from_slice(&argb.to_le_bytes());
                }
            }
        }
    }
    Some(())
}
