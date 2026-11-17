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

//! The Browse info panel: what the cursor is on, described, and the controls
//! that act on it. Its strip is reserved out of the content width by
//! `layout::info_x`, the same function the list and the grid subtract, so the
//! panel can never overlap a row. Every band it draws is placed by `info_geom`,
//! which `info_hit` reads back, so no control is drawn where a click is not
//! looked for.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect, Separator};

use super::info_cache::cached_dir;
use super::info_dir::dir_body;
use super::info_file::file_body;
use super::info_geom::info_geom;
use super::info_perms::paint_perms;
use super::info_quick_paint::paint_quick;
use super::info_sizes::{PAD, SUB_PX, TITLE_PX};
use super::info_tags::tag_band;
use super::layout::{font, hair, info_x, metrics, px};
use super::recents_group::parent_of;
use super::state::State;

pub fn paint_info(state: &State, fb: &mut PaintBuffer) {
    let x = info_x(fb.width);
    let (m, r, title_px, sub_px) = (metrics(), roles(), font(TITLE_PX), font(SUB_PX));
    let h = fb.height.saturating_sub(m.first_row_y + m.footer_h + px(8));
    Card::new().draw(fb, Rect { x, y: m.first_row_y, w: m.info_w, h });
    let (Some(entry), Some(g)) =
        (state.entries.get(state.cursor), info_geom(state, fb.width, fb.height))
    else {
        let (pen, quiet) = ((x + px(PAD)) as i32, r.quiet.argb());
        let y = (m.first_row_y + px(20)) as i32;
        let _ = fb.text_ttf(pen, y, "Nothing selected", quiet, sub_px);
        return;
    };
    let title = text::truncate(entry.label.trim_end_matches('/'), g.w, title_px);
    text::left(fb, g.x, g.title_y as i32, &title, r.text, title_px);
    let loc = text::truncate(parent_of(&entry.full_path), g.w, sub_px);
    text::left(fb, g.x, g.loc_y as i32, &loc, r.muted, sub_px);
    Separator::new().draw(fb, Rect { x: g.x, y: g.rule_y, w: g.w, h: hair() });
    paint_quick(state, fb, &g);
    if entry.is_dir {
        dir_body(fb, cached_dir(state, entry.full_path.as_str()), g.x, g.body_top, g.w);
    } else {
        file_body(fb, entry, g.x, g.body_top, g.w);
    }
    let names = state.tags.tags_for(entry.full_path.as_str());
    tag_band(state, fb, &g, &names);
    paint_perms(state, fb, &g, entry);
}
