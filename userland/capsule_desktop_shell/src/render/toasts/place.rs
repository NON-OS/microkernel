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

//! Where the panel goes: its layer above the dock at the right, as tall as
//! the queue can grow, and inside it the panel, as wide as its widest toast.

use super::metrics::{toast_dock_gap, toast_max_width, toast_min_width, toast_right_inset};
use super::rows::{panel_height, row_chrome_width};
use crate::render::layout::{bottom_dock_rect, Rect};
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::UI_PX;
use crate::state::Context;

pub fn toast_rect(display_width: u32, display_height: u32) -> Rect {
    let dock = bottom_dock_rect(display_width, display_height);
    let height = panel_height();
    let width = toast_max_width().min(display_width.saturating_sub(toast_right_inset() * 2));
    let x = display_width.saturating_sub(width + toast_right_inset());
    let y = dock.y.saturating_sub(height + toast_dock_gap());
    Rect { x, y, width, height }
}

/// Where the panel sits inside `r`: right-aligned, as wide as its widest toast.
pub(super) fn panel_rect(ctx: &Context, r: Rect) -> Rect {
    let w = panel_width(ctx, r.width);
    Rect { x: r.x + (r.width - w), y: r.y, width: w, height: r.height }
}

pub(super) fn panel_width(ctx: &Context, layer_width: u32) -> u32 {
    let widest = ctx
        .toasts
        .iter_live()
        .map(|t| measure_aa_bytes(&t.text[..t.len], UI_PX))
        .max()
        .unwrap_or(0);
    (widest + row_chrome_width()).clamp(toast_min_width().min(layer_width), layer_width)
}
