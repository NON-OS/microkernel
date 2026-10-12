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

//! The reserved band the capture indicator lives in: a strip at the top
//! right of the canvas, drawn after every layer and the cursor's place in
//! every frame that touches it, so no surface can cover it, move it or draw
//! a lookalike in it. Camera on is a green mark, microphone on an amber
//! one, a screenshot just taken a blue one; off is a dim mark, so the band
//! is always there to look at.

use crate::state::damage::Rect;
use crate::state::indicator::{Indicators, KIND_CAMERA, KIND_MICROPHONE};
use crate::sw_blitter::{fill_rect, Surface};

pub const BAND_W: u32 = 64;
pub const BAND_H: u32 = 16;
const MARK: u32 = 10;
pub const BAND_ARGB: u32 = 0xFF00_0000;
pub const CAMERA_ON: u32 = 0xFF2E_CC40;
pub const MICROPHONE_ON: u32 = 0xFFFF_9F1A;
pub const MARK_OFF: u32 = 0xFF3A_3A3A;
pub const SCREEN_ON: u32 = 0xFF3D_8BFF;

/// The band on a canvas `w` x `h`.
pub fn band(w: u32, h: u32) -> Rect {
    let (bw, bh) = (BAND_W.min(w), BAND_H.min(h));
    Rect { x: w - bw, y: 0, width: bw, height: bh }
}

/// Draw the part of the band inside `clip` onto `dst`.
pub fn draw(dst: Surface, clip: Rect, ind: &Indicators, captured: bool) {
    let b = band(dst.width, dst.height);
    let cam = Rect { x: b.x + 7, y: 3, width: MARK, height: MARK };
    let mic = Rect { x: b.x + 27, y: 3, width: MARK, height: MARK };
    let scr = Rect { x: b.x + 47, y: 3, width: MARK, height: MARK };
    let color = |kind, on| if ind.shown(kind) { on } else { MARK_OFF };
    let parts = [
        (b, BAND_ARGB),
        (cam, color(KIND_CAMERA, CAMERA_ON)),
        (mic, color(KIND_MICROPHONE, MICROPHONE_ON)),
        (scr, if captured { SCREEN_ON } else { MARK_OFF }),
    ];
    for (r, argb) in parts {
        if let Some(r) = meet(r, clip) {
            fill_rect(dst, r, argb);
        }
    }
}

fn meet(a: Rect, b: Rect) -> Option<Rect> {
    let (x0, y0) = (a.x.max(b.x), a.y.max(b.y));
    let x1 = (a.x + a.width).min(b.x.saturating_add(b.width));
    let y1 = (a.y + a.height).min(b.y.saturating_add(b.height));
    (x0 < x1 && y0 < y1).then(|| Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
}
