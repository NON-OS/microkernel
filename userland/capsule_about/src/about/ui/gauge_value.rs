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

//! The value inside a gauge's ring. It sits in the ring's hole: a long one
//! ("1920x1080") at the full size ran across the stroke, so it steps down
//! until it clears the hole, and a size still too wide at the smallest step
//! that keeps it legible is stacked on two lines at its "x".

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::line_height;

use crate::about::theme::TITLE;

use super::metrics::{TILE_RING_R, TILE_RING_T, VALUE_PX};
use super::text;

const STEPS: [f32; 4] = [VALUE_PX, 24.0, 21.0, 19.0];
const STACKED_PX: f32 = 19.0;

pub fn paint(fb: &mut PaintBuffer, cx: u32, cy: u32, value: &[u8]) {
    let hole = (TILE_RING_R - TILE_RING_T) * 2 - 4;
    if let Some(px) = STEPS.into_iter().find(|&px| text::width(fb, value, px) <= hole) {
        centred(fb, cx, cy, value, px, 0);
        return;
    }
    let Some(at) = value.iter().position(|&b| b == b'x') else {
        centred(fb, cx, cy, value, STACKED_PX, 0);
        return;
    };
    let half = line_height(STACKED_PX).max(1) / 2;
    centred(fb, cx, cy, &value[..at], STACKED_PX, -half);
    centred(fb, cx, cy, &value[at + 1..], STACKED_PX, half);
}

// One line centred on the ring, moved `dy` from its middle.
fn centred(fb: &mut PaintBuffer, cx: u32, cy: u32, line: &[u8], px: f32, dy: i32) {
    let top = text::top_of((cy - TILE_RING_R) as i32 + dy, TILE_RING_R * 2, px);
    let w = text::width(fb, line, px);
    text::line(fb, cx.saturating_sub(w / 2), top, line, TITLE, px);
}
