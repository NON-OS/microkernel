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

use crate::browser::omnibox::{scroll_to, Change, ScrollAct};
use crate::browser::state::{State, View};

/* Move the page by `act` within what it can scroll, measured against the
 * real viewport height. Returns whether the offset changed; a key or wheel
 * notch at the end of the page changes nothing and draws nothing. */
pub(super) fn apply_scroll(state: &mut State, act: ScrollAct) -> bool {
    let content_h = match (state.box_doc.as_ref(), state.document.as_ref()) {
        (Some(b), _) => b.content_h,
        (None, Some(d)) => d.content_h,
        (None, None) => 0,
    };
    let next = scroll_to(state.scroll, content_h, state.viewport_h, act);
    if next == state.scroll {
        return false;
    }
    state.scroll = next;
    /* Images evicted for the byte budget come back as they scroll into view. */
    crate::browser::image::requeue_visible(state);
    state.mark(Change::Scroll);
    true
}

/* The wheel scrolls the page, and only while a page is shown with nothing
 * over it. */
pub(super) fn on_wheel(state: &mut State, event: InputEvent) -> EventOutcome {
    if state.view != View::Page || state.settings_open {
        return EventOutcome::Idle;
    }
    match apply_scroll(state, ScrollAct::Wheel(event.delta_y)) {
        true => EventOutcome::Repaint,
        false => EventOutcome::Idle,
    }
}
