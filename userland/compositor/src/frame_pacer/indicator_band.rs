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
//! a lookalike in it. It is the menu bar's colour, so it sits inside the
//! bar. Its marks are glyphs (band_glyph_art.rs): a green camera, an amber
//! microphone, a blue screen for a screenshot just taken, and a white lock
//! while the trusted band is held (a prompt or lock with it is the
//! system's; one without it, drawn by a window to look the same, is not).
//! Off, each is dim, so the band is always there to look at. Under them the
//! bottom two rows are red while device DMA is not confined by hardware.

use crate::state::damage::Rect;
use crate::state::indicator::{Indicators, KIND_CAMERA, KIND_MICROPHONE};
use crate::sw_blitter::{fill_rect, Surface};

use super::band_glyphs::{draw_glyph, CAMERA, LOCK, MICROPHONE, SCREEN};
use super::band_meet::meet;

pub const BAND_W: u32 = 84;
pub const BAND_H: u32 = 16;
pub const BAND_ARGB: u32 = 0xFF04_080C; // BAR, made opaque
/// The shell's palette tokens (capsule_desktop_shell render/palette.rs), so
/// the band and the bar it sits in are one colour set.
pub const CAMERA_ON: u32 = 0xFF35_D6A0; // POSITIVE
pub const MICROPHONE_ON: u32 = 0xFFE8_B75C; // WARN
pub const MARK_OFF: u32 = 0xFF4C_5D68; // TEXT_MUTED
pub const SCREEN_ON: u32 = 0xFF22_C3F0; // ACCENT
pub const TRUSTED_ON: u32 = 0xFFE6_F1F6; // TEXT
pub const DMA_UNCONFINED: u32 = 0xFFE4_574F; // NEGATIVE
const UNDERLINE: u32 = 2;

/// The band on a canvas `w` x `h`.
pub fn band(w: u32, h: u32) -> Rect {
    let (bw, bh) = (BAND_W.min(w), BAND_H.min(h));
    Rect { x: w - bw, y: 0, width: bw, height: bh }
}

/// Draw the part of the band inside `clip` onto `dst`.
pub fn draw(dst: Surface, clip: Rect, ind: &Indicators, captured: bool, trusted: bool, dma: bool) {
    let b = band(dst.width, dst.height);
    let rows = UNDERLINE.min(b.height);
    let under = Rect { x: b.x, y: b.height - rows, width: b.width, height: rows };
    let under_argb = if dma { DMA_UNCONFINED } else { BAND_ARGB };
    for (r, argb) in [(b, BAND_ARGB), (under, under_argb)] {
        if let Some(r) = meet(r, clip) {
            fill_rect(dst, r, argb);
        }
    }
    let color = |kind, on| if ind.shown(kind) { on } else { MARK_OFF };
    let marks = [
        (&CAMERA, color(KIND_CAMERA, CAMERA_ON)),
        (&MICROPHONE, color(KIND_MICROPHONE, MICROPHONE_ON)),
        (&SCREEN, if captured { SCREEN_ON } else { MARK_OFF }),
        (&LOCK, if trusted { TRUSTED_ON } else { MARK_OFF }),
    ];
    for (i, (g, argb)) in marks.into_iter().enumerate() {
        draw_glyph(dst, g, (b.x + 7 + 20 * i as u32, 3), argb, clip);
    }
}
