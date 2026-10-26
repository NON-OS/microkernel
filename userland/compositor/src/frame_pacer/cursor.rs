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

use crate::state::damage::Rect;

pub use super::cursor_arrow::ARROW;

pub const FILL: u32 = 0xFFFF_FFFF;
pub const OUTLINE: u32 = 0xFF00_0000;

// A classic left pointer, one byte per pixel: 'X' is the black outline, 'o' the
// white interior, a space is transparent. The tip sits at the top-left so the
// hotspot lands on the reported pointer position.

/// The shape `code` names, its hot spot at (cx, cy), inside `clip`.
pub fn blit_shape(
    base_va: u64,
    stride: u32,
    (sw, sh): (u32, u32),
    (cx, cy): (u32, u32),
    code: u8,
    clip: Rect,
) {
    let shape = super::cursor_shapes::shape(code);
    let (clip_x1, clip_y1) = (clip.x + clip.width, clip.y + clip.height);
    let put = |px: u32, py: u32, argb: u32| {
        if px >= sw || py >= sh || px < clip.x || px >= clip_x1 || py < clip.y || py >= clip_y1 {
            return;
        }
        let va = base_va as usize + py as usize * stride as usize + px as usize * 4;
        unsafe { core::ptr::write_volatile(va as *mut u32, argb) };
    };
    for (row, line) in shape.rows.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            let argb = match pixel {
                b'X' => OUTLINE,
                b'o' => FILL,
                _ => continue,
            };
            let (x, y) = (
                (cx + col as u32).checked_sub(shape.hot.0),
                (cy + row as u32).checked_sub(shape.hot.1),
            );
            if let (Some(x), Some(y)) = (x, y) {
                put(x, y, argb);
            }
        }
    }
}
