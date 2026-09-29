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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::omnibox::geometry::CONTENT_TOP;
use crate::browser::omnibox::{hover_update, Change};
use crate::browser::state::{State, View};

/* Pointer motion: track the link under the pointer on the page, hit-tested
 * in viewport space. Only a change of link redraws, and then only the
 * status bubble at the bottom of the page. */
pub(super) fn on_pointer(state: &mut State, event: InputEvent) -> EventOutcome {
    let y = event.y - CONTENT_TOP as i32;
    let over_page = state.view == View::Page && !state.settings_open && y >= 0;
    let next = if over_page { super::link_under::link_under(state, event.x, y) } else { None };
    if !hover_update(&mut state.ui.hover_href, next.as_deref()) {
        return EventOutcome::Idle;
    }
    state.mark(Change::Bubble);
    EventOutcome::Repaint
}
