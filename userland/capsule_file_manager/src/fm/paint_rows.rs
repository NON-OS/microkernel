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

//! The detail list: one shared `Row` per entry, placed by `row_geom::row_slots`
//! so a click hit-tests against exactly what was drawn.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Rect, Row, Scrolled, State as Look};

use super::layout::{content_w, content_x, metrics, px};
use super::layout_measure::pane;
use super::list_head::paint_head;
use super::listing_state::empty_listing;
use super::paint_row::row_body;
use super::row_geom::{list_scroll, list_top, row_slots};
use super::screen_row::empty_state;
use super::selection_is_selected::is_selected;
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// The head stays put and the rows scroll under it, drawn through `Scrolled`
/// at their content y. A checked row draws selected and the cursor's row
/// focused, so the shared focus ring marks it; hover comes from the pointer.
pub fn paint_rows(state: &State, fb: &mut PaintBuffer) {
    let left = content_x();
    let cw = content_w(fb.width);
    paint_head(state, fb, left, cw, state.row_top);
    let p = pane(state);
    if state.entries.is_empty() {
        let filtered = !state.filter.is_empty() || !state.tag_filter.is_empty();
        let (head, note) = empty_listing(state.load_error, filtered);
        empty_state(fb, left, list_top(&p) + px(40), cw, head, note);
        return;
    }
    let (fb, row_h) = (&mut Scrolled::new(fb, list_scroll(&p)), metrics().row_h);
    for slot in row_slots(&p) {
        let selected = is_selected(state, &state.entries[slot.index].full_path);
        let focused = slot.index == state.cursor;
        let lit = Look { selected, focused, ..look(state, Target::Row(slot.index)) };
        Row::new("").state(lit).draw(fb, Rect { x: left, y: slot.y, w: cw, h: row_h });
        row_body(state, fb, slot.index, slot.y, left, cw);
    }
}
