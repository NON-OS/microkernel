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
//! A window's frame at a scale: its measures, its pixels and its title's rows.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::{draw_frame_at, titlebar_rect_at, DecorationHit, FrameState};

pub(super) const W: u32 = 900;

pub(super) const H: u32 = 600;

pub(super) const SCALES: [u32; 4] = [4, 5, 6, 8];

pub(super) const AT_REST: FrameState = FrameState { hover: DecorationHit::None, focused: true };

/// The shell's `px`, without its latch: tests run at once on threads.
pub(super) fn px(v: u32, q: u32) -> u32 {
    (v * q + 2) / 4
}

/// Paints the frame in `state` and reads back its pixels.
pub(super) fn paint_in(q: u32, title: &[u8], state: FrameState) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw_frame_at(&mut fb, false, title, state, 0, q);
    px
}

/// Paints the focused frame with nothing hovered.
pub(super) fn paint(q: u32, title: &[u8]) -> Vec<u32> {
    paint_in(q, title, AT_REST)
}

/// The columns and rows of the title bar the title's ink reaches.
pub(super) fn title_ink(q: u32, title: &[u8]) -> ((u32, u32), (u32, u32)) {
    let (plain, titled) = (paint(q, b""), paint(q, title));
    let bar = titlebar_rect_at(W, H, false, q);
    let (mut cols, mut rows) = ((u32::MAX, 0), (u32::MAX, 0));
    for y in 0..H {
        for x in 0..W {
            let i = (y * W + x) as usize;
            if plain[i] != titled[i] {
                assert!(y >= bar.y && y < bar.y + bar.h, "title ink at row {y} outside the bar");
                (cols.0, cols.1) = (cols.0.min(x), cols.1.max(x));
                (rows.0, rows.1) = (rows.0.min(y), rows.1.max(y));
            }
        }
    }
    assert!(rows.0 <= rows.1, "the title is drawn at {q}");
    (cols, rows)
}
