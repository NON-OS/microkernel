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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Progress, Rect};

use super::home_cat_geom::CatCell;
use super::home_stor_geom::{stor_cells, STOR_PLACES};
use super::home_stor_text::{bar_pct, detail, size_text};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{font, px};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const PAD: u32 = 12;
const ICON: u32 = 18;
const BAR_H: u32 = 6;
const TITLE_PX: f32 = 15.0;
const SUB_PX: f32 = 13.0;

/// Draws the storage row from the same cells `home_hit` tests against. Every
/// figure on it comes from the cached `usage` and `dirstat` reads taken before
/// paint, never from a call made inside it.
pub fn paint_storage_row(state: &State, fb: &mut PaintBuffer) {
    for cell in stor_cells(state.win_w, state.win_h) {
        card(state, fb, &cell);
    }
}

fn card(state: &State, fb: &mut PaintBuffer, c: &CatCell) {
    let (title, path) = STOR_PLACES[c.idx];
    let (r, pad, icon, title_px, sub_px) =
        (roles(), px(PAD), px(ICON), font(TITLE_PX), font(SUB_PX));
    let muted = r.muted.argb();
    let at = Rect { x: c.x, y: c.y, w: c.w, h: c.h };
    Card::new().state(look(state, Target::Stor(c.idx))).draw(fb, at);
    let (inner, tx, ty) = (c.w.saturating_sub(pad * 2), c.x + pad, c.y + pad);
    draw(fb, Icon::Cube, tx, ty, icon, r.accent.argb());
    let size = size_text(state, c.idx, path);
    let room = inner.saturating_sub(icon + text::width(size.as_str(), sub_px) + pad * 2);
    let cut = text::truncate(title, room, title_px);
    let _ = fb.text_ttf((tx + icon + px(10)) as i32, ty as i32, &cut, r.text.argb(), title_px);
    let right = (c.x + c.w).saturating_sub(pad);
    text::right(fb, right, ty as i32, size.as_str(), r.muted, sub_px);
    let track = Rect { x: tx, y: ty + px(24), w: inner, h: px(BAR_H) };
    Progress::new(bar_pct(state, c.idx, path).min(100), 100).draw(fb, track);
    let line = detail(state, c.idx, path);
    let shown = text::truncate(line.as_str(), inner, sub_px);
    let _ = fb.text_ttf(tx as i32, (ty + px(34)) as i32, &shown, muted, sub_px);
}
