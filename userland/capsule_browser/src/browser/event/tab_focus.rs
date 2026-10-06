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

use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;

use crate::browser::omnibox::ScrollAct;
use crate::browser::state::State;

use super::field_at::{field_at, Field};

/* Tab and Shift+Tab walk the page's text fields in document order,
 * wrapping around, and bring the focused one into view. */
pub(super) fn tab_focus(state: &mut State, back: bool) -> EventOutcome {
    let Some(dom) = state.page_dom.as_ref() else {
        return EventOutcome::Idle;
    };
    let fields: Vec<usize> = (1..dom.nodes.len())
        .filter(|&i| matches!(field_at(dom, i), Field::Edit(id) if id == i))
        .collect();
    if fields.is_empty() {
        return EventOutcome::Idle;
    }
    let n = fields.len();
    let at = state.focus.and_then(|f| fields.iter().position(|&x| x == f));
    let next = match (at, back) {
        (None, false) => 0,
        (None, true) => n - 1,
        (Some(i), false) => (i + 1) % n,
        (Some(i), true) => (i + n - 1) % n,
    };
    let id = fields[next];
    let (y, h) = (dom.box_of(id, 1), dom.box_of(id, 3));
    state.focus_page(Some(id));
    let top = state.scroll as i32;
    if y < top || y + h > top + state.viewport_h as i32 {
        let to = y as i64 - state.viewport_h as i64 / 3;
        super::scroll_by::apply_scroll(state, ScrollAct::To(to));
    }
    EventOutcome::Idle
}
