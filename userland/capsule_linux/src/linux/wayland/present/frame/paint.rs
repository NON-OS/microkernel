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

//! The frame painted around the guest's pixels; pure, for the host proofs.

use nonos_toolkit::decorations::{draw_frame_at, frame_bg, DecorationHit, FrameState};
use nonos_toolkit::paint::PaintBuffer;

use super::geometry::Around;

/// The title, the control under the pointer, and focus.
pub struct Look<'a> {
    pub title: &'a [u8],
    pub hover: DecorationHit,
    pub focused: bool,
}

/// `out` (`a.w` by `a.h`) with `client` (premultiplied ARGB, `stride` bytes
/// a row) over the frame's ground, the frame painted first when `chrome`.
/// False when `out` is too short or not word aligned.
pub fn paint(
    out: &mut [u8],
    a: Around,
    client: &[u8],
    (cw, ch, stride): (u32, u32, u32),
    (look, chrome): (&Look<'_>, bool),
    quarters: u32,
) -> bool {
    // SAFETY: any four bytes are a valid u32; only the aligned middle is used.
    let (head, words, _) = unsafe { out.align_to_mut::<u32>() };
    let n = (a.w as usize).saturating_mul(a.h as usize);
    if !head.is_empty() || words.len() < n {
        return false;
    }
    let mut fb =
        PaintBuffer { pixels: &mut words[..n], stride_words: a.w, width: a.w, height: a.h };
    if chrome {
        let state = FrameState { hover: look.hover, focused: look.focused };
        draw_frame_at(&mut fb, a.max, look.title, state, 0, quarters);
    }
    let ground = frame_bg();
    let (cw, ch) = (cw.min(a.w.saturating_sub(a.cx)), ch.min(a.h.saturating_sub(a.cy)));
    // An opaque pixel over the ground is itself, so each row is one pass
    // that tests no pixel (nonos_pixel_rows).
    let pixels = nonos_pixel_rows::words::bytes_mut(fb.pixels);
    for y in 0..ch as usize {
        let row = client.get(y * stride as usize..).unwrap_or(&[]);
        let at = ((a.cy as usize + y) * a.w as usize + a.cx as usize) * 4;
        let Some(dst) = pixels.get_mut(at..at + cw as usize * 4) else { break };
        nonos_pixel_rows::over_colour(dst, row, ground);
    }
    true
}
