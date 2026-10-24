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

//! A screen composed as the compositor stacks it: the wallpaper, the shell's
//! desk, the window where the window manager places it, then the shell's
//! chrome; each layer over the last by its alpha.

use super::frame::Shot;
use crate::geometry::{clamp_to_display, Rect};

/// `src` over `dst`, ARGB, by `src`'s alpha.
fn over(dst: u32, src: u32) -> u32 {
    let a = src >> 24;
    let mix = |s: u32, d: u32| (s * a + d * (255 - a)) / 255;
    let ch = |v: u32, sh: u32| (v >> sh) & 0xFF;
    let (r, g, b) =
        (mix(ch(src, 16), ch(dst, 16)), mix(ch(src, 8), ch(dst, 8)), mix(ch(src, 0), ch(dst, 0)));
    0xFF00_0000 | r << 16 | g << 8 | b
}

/// Where the window manager opens `shot` as the first window on `w` x `h`.
pub fn placed(shot: &Shot, w: u32, h: u32) -> Rect {
    let at = shot.at;
    let asked = Rect { x: at.initial_x, y: at.initial_y, width: at.width, height: at.height };
    crate::cascade::cascade(w, h, 0, clamp_to_display(asked, w, h))
}

/// The screen, and the window's rectangle on it.
pub fn screen(w: u32, h: u32, layers: [&[u32]; 3], shot: &Shot) -> (Vec<u32>, Rect) {
    let [wall, desk, chrome] = layers;
    let mut px: Vec<u32> = wall.iter().zip(desk).map(|(&d, &s)| over(d, s)).collect();
    let r = placed(shot, w, h);
    for y in 0..r.height.min(h - r.y) {
        for x in 0..r.width.min(w - r.x) {
            let at = ((r.y + y) * w + r.x + x) as usize;
            px[at] = over(px[at], shot.pixels[(y * r.width + x) as usize]);
        }
    }
    px.iter_mut().zip(chrome).for_each(|(d, &s)| *d = over(*d, s));
    (px, r)
}
