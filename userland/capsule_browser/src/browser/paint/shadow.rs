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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::Fragment;

use super::box_page::TOP;
use super::corners::corners;

/* Paths spelled out so the children resolve beside this file however it
 * is itself included (host renderers include it by path). */
#[path = "shadow/gauss.rs"]
mod gauss;
#[path = "shadow/inset.rs"]
mod inset;
#[path = "shadow/outer.rs"]
mod outer;
#[path = "shadow/rrect.rs"]
mod rrect;

use rrect::RRect;

/* The screen area a shadow may touch: [x0, y0, x1, y1), inside the frame,
 * below the chrome and within the fragment's clip. */
type Window = [i32; 4];

/// Paint the fragment's box-shadow layers of one kind, the last layer
/// first so the first ends on top: outer shadows (`inset` false) go down
/// before the background, inset ones (`inset` true) after it.
pub(super) fn paint_shadow(
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    clip: Option<[i32; 4]>,
    inset: bool,
) {
    let Some(s) = f.shadow.as_ref() else { return };
    let mut win = [0, TOP, fb.width as i32, fb.height as i32];
    if let Some(c) = clip {
        win = [win[0].max(c[0]), win[1].max(c[1]), win[2].min(c[2]), win[3].min(c[3])];
    }
    if win[0] >= win[2] || win[1] >= win[3] || f.w <= 0 || f.h <= 0 {
        return;
    }
    let r = corners(f.radius, f.w, f.h).map(|v| v as f32);
    let (x, y, w, h) = (f.x as f32, sy as f32, f.w as f32, f.h as f32);
    let rect = RRect { x, y, w, h, r };
    for l in s.layers[..(s.n as usize).min(s.layers.len())].iter().rev() {
        if l.inset != inset {
            continue;
        }
        let a = ((l.color >> 24) * f.alpha as u32 + 127) / 255;
        let color = (a << 24) | (l.color & 0x00ff_ffff);
        let g = [l.dx as f32, l.dy as f32, l.blur as f32, l.spread as f32];
        if inset {
            inset::paint(fb, &rect, f.border, g, color, win);
        } else {
            outer::paint(fb, &rect, g, color, win);
        }
    }
}
