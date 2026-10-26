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

//! The panel painted: a dark ground, a row per toast with its level's
//! accent, and its text cut to the row.

use super::place::panel_rect;
use super::rows::{accent_width, panel_edge_inset, row_gap, row_height, text_inset};
use crate::render::fill::fill_rect;
use crate::render::layout::Rect;
use crate::render::measure_aa::truncate_to_width;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{top_y_centered, valid_str, UI_PX};
use crate::state::Context;

const PANEL_ARGB: u32 = 0xFF0E_1218;
const ROW_ARGB: u32 = 0xFF1B_2030;
const TEXT_ARGB: u32 = 0xFFCF_E6E9;

pub(super) fn paint_toasts(ctx: &Context, r: Rect) {
    let panel = panel_rect(ctx, r);
    let (panel_x, panel_w) = (panel.x, panel.width);
    let row_h = row_height();
    let fill = |x, y, width, height, argb| fill_rect(ctx, Rect { x, y, width, height }, argb);
    fill(panel_x, r.y, panel_w, r.height, PANEL_ARGB);
    for (i, toast) in ctx.toasts.iter_live().enumerate() {
        let row_y = r.y + row_gap() + i as u32 * (row_h + row_gap());
        let inset = panel_edge_inset();
        let row_w = panel_w.saturating_sub(inset * 2);
        fill(panel_x + inset, row_y, row_w, row_h, ROW_ARGB);
        fill(panel_x + inset, row_y, accent_width(), row_h, toast.level.tint());
        let text_x = panel_x + inset + accent_width() + text_inset();
        let text_max_w = (panel_x + panel_w).saturating_sub(text_x + text_inset());
        let label = truncate_to_width(valid_str(&toast.text[..toast.len]), UI_PX, text_max_w);
        text_aa(ctx, text_x, top_y_centered(row_y, row_h, UI_PX), label, TEXT_ARGB, UI_PX);
    }
}
