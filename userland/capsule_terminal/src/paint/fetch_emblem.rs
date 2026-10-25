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

//! The 3D Ø on the splash at the screen's own resolution, `neofetch`'s shape
//! and light sampled per screen pixel, worked out once a size and kept.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_app_skeleton::PaintBuffer;

use crate::command::builtin::neofetch::{emblem_light::shade_at, emblem_shape::*};

/// The emblem as drawn (ARGB, 0 clear) and the size and background it is for.
#[derive(Default)]
pub struct Art {
    pub key: (u32, u32, u32),
    pub px: Vec<u32>,
}

/// Draw it 200 pixels high, the facts' height, at `(x, y)` when it fits
/// left of `right`, as the splash does.
pub fn beside(fb: &mut PaintBuffer, at: (u32, u32, u32), bg: u32, art: &RefCell<Art>) {
    if at.0 + width_for(200) <= at.2 {
        draw_emblem(fb, at.0, at.1, 200, bg, art);
    }
}

/// Width for a height: the emblem's pixels are seven across to ten down.
pub fn width_for(h: u32) -> u32 {
    h * (EMBLEM_W as u32 * 7) / (EMBLEM_H as u32 * 10)
}

/// The emblem `h` pixels high over `bg` (0xRRGGBB), one ARGB value a pixel.
pub fn render(h: u32, bg: u32) -> Vec<u32> {
    let w = width_for(h);
    let size = (EMBLEM_W as f32 / w as f32, EMBLEM_H as f32 / h as f32);
    let (rgb, mut px) = ([bg >> 16 & 0xFF, bg >> 8 & 0xFF, bg & 0xFF], Vec::new());
    for v in 0..h {
        for u in 0..w {
            let at = shade_at(u as f32 * size.0, v as f32 * size.1, size, rgb);
            px.push(at.map_or(0, |c| 0xFF00_0000 | c[0] << 16 | c[1] << 8 | c[2]));
        }
    }
    px
}

/// Draw it at `(x, y)` from `art`, worked out again for a new size or theme.
pub fn draw_emblem(fb: &mut PaintBuffer, x: u32, y: u32, h: u32, bg: u32, art: &RefCell<Art>) {
    let (mut art, w) = (art.borrow_mut(), width_for(h));
    if art.key != (w, h, bg) {
        (art.px, art.key) = (render(h, bg), (w, h, bg));
    }
    for v in 0..h.min(fb.height.saturating_sub(y)) {
        for u in 0..w.min(fb.width.saturating_sub(x)) {
            match art.px[(v * w + u) as usize] {
                0 => {}
                c => fb.pixels[((y + v) * fb.stride_words + x + u) as usize] = c,
            }
        }
    }
}
