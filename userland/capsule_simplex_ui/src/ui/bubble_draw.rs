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

//! Drawing a bubble on its side: ours on the right in the accent's tint,
//! theirs on the left on the pane's colour.

use nonos_app_skeleton::PaintBuffer;

use super::bubble::{width, Bubble};
use super::metrics::{Metrics, BODY_PX, LINE, PAD, RADIUS, SMALL_LINE, SMALL_PX};
use super::theme::Colors;

/// Draws `b` with its top at `y`, against the edge of the area `[x, x + w)`
/// its side belongs to.
pub fn draw(fb: &mut PaintBuffer, b: &Bubble, x: u32, w: u32, y: u32, m: Metrics, c: &Colors) {
    let pad = m.px(PAD);
    let bx = if b.sent { x + w - pad - b.w } else { x + pad };
    let fill = if b.sent { c.sent } else { c.pane };
    fb.fill_round(bx, y, b.w, b.h, m.px(RADIUS), fill);
    let mut ty = y + pad;
    if !b.author.is_empty() {
        fb.text_ttf((bx + pad) as i32, ty as i32 - 2, &b.author, c.accent, SMALL_PX);
        ty += m.px(SMALL_LINE);
    }
    for line in &b.lines {
        fb.text_ttf((bx + pad) as i32, ty as i32, line, c.text, BODY_PX);
        ty += m.px(LINE);
    }
    let meta_x = bx + b.w - pad - width(&b.meta, SMALL_PX);
    let tone = if b.failed { c.fail } else { c.quiet };
    fb.text_ttf(meta_x as i32, ty as i32, &b.meta, tone, SMALL_PX);
}
