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

use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_DOWN, KEY_END, KEY_ESC, KEY_HOME, KEY_PAGE_DOWN, KEY_PAGE_UP,
    KEY_TAB, KEY_UP, MOD_SHIFT,
};

use crate::browser::omnibox::{Route, ScrollAct};
use crate::browser::state::State;

/* A key while the page has the keyboard: a focused field types, scroll
 * keys scroll, Tab walks the fields, and Esc leaves a field or stops a
 * load. */
pub fn on_page_key(state: &mut State, event: InputEvent, route: Route) -> EventOutcome {
    let shift = event.flags & MOD_SHIFT != 0;
    match (route, event.code) {
        (_, KEY_TAB) => super::tab_focus::tab_focus(state, shift),
        (Route::Field(_), KEY_ESC) => {
            state.focus_page(None);
            EventOutcome::Idle
        }
        (Route::Field(id), _) => super::field_key::field_key(state, id, event),
        (Route::Scroll, code) => {
            let act = match code {
                KEY_UP => ScrollAct::Line(-1),
                KEY_DOWN => ScrollAct::Line(1),
                KEY_PAGE_UP => ScrollAct::Page(-1),
                KEY_PAGE_DOWN => ScrollAct::Page(1),
                0x20 if shift => ScrollAct::Page(-1),
                0x20 => ScrollAct::Page(1),
                KEY_HOME => ScrollAct::Home,
                KEY_END => ScrollAct::End,
                _ => return EventOutcome::Idle,
            };
            match super::scroll_by::apply_scroll(state, act) {
                true => EventOutcome::Repaint,
                false => EventOutcome::Idle,
            }
        }
        (_, KEY_ESC) if state.loading() => {
            super::stop::stop(state);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
