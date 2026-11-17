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

//! Icon-grid view: one `grid_card` per entry on the shared `Card`. Cells
//! are placed by `grid_geom::cell_slots`, the same list `cell_at` hit-tests
//! against, so a click lands on the card that was drawn.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Rect, Scrolled, State as Look};

use super::grid_card::card;
use super::grid_geom::{cell_slots, grid_scroll};
use super::layout::{content_w, content_x, metrics, px};
use super::layout_measure::pane;
use super::listing_state::empty_listing;
use super::screen_row::empty_state;
use super::selection_is_selected::is_selected;
use super::state::State;
use super::target::Target;
use super::target_at::look;

// The plate is inset inside its cell so neighbouring cards never touch.
const INSET_X: u32 = 6;
const INSET_Y: u32 = 3;

pub fn paint_grid(state: &State, fb: &mut PaintBuffer) {
    // The grid is the default view, so an empty or unlisted folder must read
    // the same here as in the list instead of leaving the pane blank.
    let m = metrics();
    if state.entries.is_empty() {
        let filtered = !state.filter.is_empty() || !state.tag_filter.is_empty();
        let (head, note) = empty_listing(state.load_error, filtered);
        empty_state(fb, content_x(), m.grid_top + px(40), content_w(fb.width), head, note);
        return;
    }
    let (inset_x, inset_y) = (px(INSET_X), px(INSET_Y));
    let pw = m.grid_cell_w - inset_x * 2;
    let ph = m.grid_cell_h - inset_y * 2;
    let p = pane(state);
    let fb = &mut Scrolled::new(fb, grid_scroll(&p));
    for cell in cell_slots(&p) {
        let entry = &state.entries[cell.index];
        let (selected, focused) =
            (is_selected(state, &entry.full_path), cell.index == state.cursor);
        let lit = Look { selected, focused, ..look(state, Target::Cell(cell.index)) };
        card(fb, entry, Rect { x: cell.x + inset_x, y: cell.y + inset_y, w: pw, h: ph }, lit);
    }
}
