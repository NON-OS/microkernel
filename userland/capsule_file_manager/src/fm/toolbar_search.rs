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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect, State as Look};

use super::header_layout::search_is_field;
use super::header_slots::{
    Slot, CROSS_S, GLYPH_GAP, GLYPH_S, ICON_PAD, SEARCH_HINT, TOOL_H, TOOL_PX,
};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::state::State;
use super::target::Target;
use super::target_at::look;
use super::toolbar_buttons::{glyph_btn, glyph_y, plain_icon};

/// The query control. Where the header is too narrow for a field it collapses to
/// the bare glyph, which opens the same Search surface; where it is a field, the
/// live query is drawn in it and a clear mark is offered at its right. The field
/// is a `Card`, outlined in the accent while it holds a query.
pub fn search(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, clear: Option<&Slot>) {
    if !search_is_field(state) {
        glyph_btn(state, fb, slot, y, IconId::Search, false);
        return;
    }
    let (empty, r) = (state.query.is_empty(), roles());
    let lit = Look { selected: !empty, ..look(state, Target::Head(slot.hit)) };
    Card::new().state(lit).draw(fb, Rect { x: slot.x, y, w: slot.w, h: TOOL_H });
    let gy = glyph_y(y, GLYPH_S);
    draw(fb, Icon::Magnifier, slot.x + ICON_PAD, gy, GLYPH_S, r.muted.argb());
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    let tail = clear.map(|c| c.w).unwrap_or(ICON_PAD);
    let room = (slot.x + slot.w).saturating_sub(pen + tail);
    let line = if empty { SEARCH_HINT } else { state.query.as_str() };
    let ink = if empty { r.quiet } else { r.text };
    let cut = text::truncate(line, room, TOOL_PX);
    text::left(fb, pen, text::centre_top(y, TOOL_H, TOOL_PX), &cut, ink, TOOL_PX);
    if let Some(slot) = clear {
        plain_icon(state, fb, slot, y, (IconId::Close, CROSS_S));
    }
}
