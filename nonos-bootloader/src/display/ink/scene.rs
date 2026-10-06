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

//! The two-column scene every full screen of the loader shares: the brand
//! frame on the left, a column of content on the right, both centred on the
//! band above a footer, and the footer under a full-width rule.

use super::style::{unit, Style};
use super::text::metrics;
use super::mark::mark_size;
use crate::display::gop::get_dimensions;

#[derive(Clone, Copy)]
pub struct Scene {
    pub u: u32,
    /// The brand frame's box: x, y, width, height.
    pub frame: (u32, u32, u32, u32),
    pub col_x: u32,
    pub col_w: u32,
    /// Where the right column starts, given its height.
    pub col_y: u32,
    pub footer_y: u32,
}

/// The scene for a right column `content_h` tall.
pub fn scene(content_h: u32) -> Scene {
    let (w, h) = get_dimensions();
    let u = unit();
    let (mono, body) = (metrics(Style::Mono), metrics(Style::Body));
    let footer_y = h.saturating_sub(mono.line * 2 + 8 * u);
    /* The frame is the brand tile: 9 wide to 10 tall, the Ø 40% of its width. */
    let fw = mark_size().0 * 5 / 2;
    let fh = fw * 10 / 9;
    let gap = 16 * u;
    let col_w = (body.px * 34).min(w.saturating_sub(fw + gap + 16 * u));
    let left = w.saturating_sub(fw + gap + col_w) / 2;
    Scene {
        u,
        frame: (left, footer_y.saturating_sub(fh + mono.line * 3) / 2, fw, fh),
        col_x: left + fw + gap,
        col_w,
        col_y: footer_y.saturating_sub(content_h) / 2,
        footer_y,
    }
}
