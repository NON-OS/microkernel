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

//! The band that appears over the listing once entries are checked: what is
//! selected, how big it is, and what can be done to it. Every box it draws comes
//! from `sel_geom` and `sel_slots`, which `sel_hit` tests against.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect, State as Look};

use super::header_slots::strip;
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{font, px};
use super::sel_action_btn::action;
use super::sel_geom::{action_y, band, summary_y, SelBand, PAD, SUM_PX};
use super::sel_slots::slots;
use super::sel_summary::summary;
use super::state::State;

/// Draws nothing at all when the selection is empty, so the listing keeps its
/// whole height until there is something for the band to act on.
pub fn paint_band(state: &State, fb: &mut PaintBuffer) {
    if state.selected.is_empty() {
        return;
    }
    let b = band(fb.width, fb.height);
    let on = Look { selected: true, ..Look::default() };
    Card::new().state(on).draw(fb, Rect { x: b.x, y: b.y, w: b.w, h: b.h });
    headline(state, fb, &b);
    let ay = action_y(&b);
    for (i, slot) in slots(&b).iter().enumerate() {
        action(state, fb, i, slot, ay, b.labelled);
    }
}

fn headline(state: &State, fb: &mut PaintBuffer, b: &SelBand) {
    let (sy, r, s, pad, sum_px) = (summary_y(b), roles(), strip(), px(PAD), font(SUM_PX));
    draw(fb, Icon::Check, b.x + pad, sy, s.glyph, r.accent.argb());
    let x = b.x + pad + s.glyph + s.glyph_gap;
    let room = b.w.saturating_sub(pad * 2 + s.glyph + s.glyph_gap);
    let line = summary(state);
    let cut = text::truncate(line.as_str(), room, sum_px);
    text::left(fb, x, sy as i32, &cut, r.text, sum_px);
}
