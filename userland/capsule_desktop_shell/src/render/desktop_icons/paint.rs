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

//! Draw each desktop entry: its mark, the centred name, and a dimmer line
//! naming what the entry is.

use super::cell_rect::cell_rect;
use super::meta::paint_meta;
use super::metrics::icon;
use super::pill::pill_pad;
use super::rename::paint_rename;
use super::tile::scrim;
use crate::render::icons::draw_fs_icon;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::palette;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{line_h, px, LABEL_PX};
use crate::state::Context;

const LABEL_FG: u32 = palette::TEXT;

const GLYPH_GAP_LOGICAL: u32 = 7;

pub fn paint_desktop_icons(ctx: &Context) {
    for (i, item) in ctx.desktop_items.iter().enumerate() {
        let (cx, cy, cw, _) = cell_rect(ctx, i);
        let label_y = cy + icon() + px(GLYPH_GAP_LOGICAL);
        let renaming = ctx.desk_ui.rename == Some(i);
        if !renaming {
            scrim(ctx, item, cx, cy, cw, label_y);
        }
        let icon_x = cx + (cw - icon()) / 2;
        draw_fs_icon(ctx, icon_x, cy, icon(), item.is_dir);
        if renaming {
            paint_rename(ctx, cx, cw, label_y);
        } else {
            paint_label(ctx, &item.name, cx, cw, label_y);
            paint_meta(ctx, item, cx, cw, label_y + line_h(LABEL_PX));
        }
    }
}

/// The icon being dragged, under the cursor. It is drawn with the chrome, so
/// it stays in sight over any window it is dragged across.
pub fn paint_drag_ghost(ctx: &Context) {
    if ctx.desk_ui.drag_moved {
        if let Some(item) = ctx.desk_ui.drag_from.and_then(|i| ctx.desktop_items.get(i)) {
            let gx = ctx.desk_ui.drag_x.saturating_sub(icon() / 2);
            let gy = ctx.desk_ui.drag_y.saturating_sub(icon() / 2);
            draw_fs_icon(ctx, gx, gy, icon(), item.is_dir);
        }
    }
}

// The name, centred in the cell.
fn paint_label(ctx: &Context, name: &str, cx: u32, cw: u32, label_y: u32) {
    let shown = truncate_to_width(name, LABEL_PX, cw.saturating_sub(pill_pad() * 2));
    let x = cx + cw.saturating_sub(measure_aa(shown, LABEL_PX)) / 2;
    text_aa(ctx, x, label_y, shown, LABEL_FG, LABEL_PX);
}
