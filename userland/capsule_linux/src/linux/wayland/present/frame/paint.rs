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
        draw_frame_at(&mut fb, false, look.title, state, 0, quarters);
    }
    let ground = frame_bg();
    let (cw, ch) = (cw.min(a.w.saturating_sub(a.cx)), ch.min(a.h.saturating_sub(a.cy)));
    for y in 0..ch as usize {
        let row = &client[y * stride as usize..];
        let at = (a.cy as usize + y) * a.w as usize + a.cx as usize;
        for (x, px) in row.chunks_exact(4).take(cw as usize).enumerate() {
            let src = u32::from_le_bytes([px[0], px[1], px[2], px[3]]);
            fb.pixels[at + x] = if src >> 24 == 0xFF { src } else { over(src, ground) };
        }
    }
    true
}

/// Premultiplied `src` over `dst` (wl_shm ARGB8888).
pub fn over(src: u32, dst: u32) -> u32 {
    let keep = 255 - (src >> 24);
    let mix = |shift: u32| {
        let (s, d) = ((src >> shift) & 0xFF, (dst >> shift) & 0xFF);
        (s + (d * keep + 127) / 255).min(255) << shift
    };
    mix(0) | mix(8) | mix(16) | mix(24)
}
