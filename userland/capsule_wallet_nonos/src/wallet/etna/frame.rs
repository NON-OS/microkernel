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

//! The one frame every wallet screen sits in, as ScreenFrame.swift draws
//! it. The photograph and the content scroll together between a fixed bar
//! and a fixed foot, so the frame is drawn in two passes: `begin` lays the
//! ground and the scrolled photograph and says where content goes, the
//! screen draws, then `end` lays the bar, the footer and the status line
//! over whatever ran past, which is the clip.

use nonos_app_skeleton::PaintBuffer;

use super::band::band;
use super::frame_bar::{band_height, bar};
use super::frame_foot::{foot, foot_height};
use super::frame_spec::{FrameLayout, FrameSpec};
use super::parts::failure::failure;
use super::rect::Rect;
use super::tokens::{BAR_H, COLUMN, GAP, INK, ROOM, SIDE};

pub fn column_x(fb: &PaintBuffer) -> u32 {
    fb.width.saturating_sub(COLUMN) / 2
}

/// Ground, photograph and error banner, shifted up by `spec.scroll`.
pub fn begin(fb: &mut PaintBuffer, spec: &FrameSpec) -> FrameLayout {
    let mut out = FrameLayout::default();
    fb.fill_rect(0, 0, fb.width, fb.height, INK);
    let x0 = column_x(fb);
    let top = (BAR_H + 1) as i64 - i64::from(spec.scroll);
    let mut y = top;
    if let Some(which) = spec.backdrop {
        let h = band_height(fb, spec);
        if y + i64::from(h) > 0 {
            band(fb, x0, y, which, h);
        }
        y += i64::from(h);
    }
    y += i64::from(ROOM);
    let (cx, cw) = (x0 + SIDE, COLUMN - 2 * SIDE);
    if let Some(text) = spec.failure {
        let fy = y.max(0) as u32;
        let (h, dismiss) = failure(fb, cx, fy, cw, text);
        out.dismiss = Some(dismiss);
        y += i64::from(h + GAP);
    }
    let bottom = fb.height.saturating_sub(foot_height(spec));
    let cy = y.max(0) as u32;
    out.content = Rect::new(cx, cy, cw, bottom.saturating_sub(cy + ROOM));
    out.content_bottom = bottom;
    out
}

/// Bar, footer and status line, over the content.
pub fn end(fb: &mut PaintBuffer, spec: &FrameSpec, out: &mut FrameLayout) {
    let x0 = column_x(fb);
    fb.fill_rect(x0, 0, COLUMN, BAR_H + 1, INK);
    out.back = bar(fb, x0, spec);
    foot(fb, x0, spec, out);
}
