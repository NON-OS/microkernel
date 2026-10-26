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

use super::cursor::{FILL, OUTLINE};
use super::cursor_shapes::shape;

pub const SIDE: usize = 64;
pub const SLOT: usize = SIDE * SIDE * 4;

/// The most rows or columns any shape has: the up-and-down arrow's 21.
const MAX_EXTENT: usize = 21;
const MAX_SCALE: usize = SIDE / MAX_EXTENT;

/// The width and height of shape `code` at `scale`, a whole number.
pub fn shape_side(scale: u32, code: u8) -> (u16, u16) {
    let (s, rows) = ((scale as usize).clamp(1, MAX_SCALE), shape(code).rows);
    let w = rows.iter().map(|l| l.len()).max().unwrap_or(0);
    ((w * s) as u16, (rows.len() * s) as u16)
}

/// Its hot spot at `scale`.
pub fn shape_hot(scale: u32, code: u8) -> (u16, u16) {
    let (s, hot) = (scale.clamp(1, MAX_SCALE as u32), shape(code).hot);
    ((hot.0 * s) as u16, (hot.1 * s) as u16)
}

/// Shape `code` at `scale` into a clear slot, its top-left at the slot's.
pub fn draw_shape(slot: &mut [u8], scale: u32, code: u8) -> Option<()> {
    let slot = slot.get_mut(..SLOT)?;
    slot.fill(0);
    let s = (scale as usize).clamp(1, MAX_SCALE);
    for (row, line) in shape(code).rows.iter().enumerate() {
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
