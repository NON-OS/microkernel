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

use alloc::string::String;

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::omnibox::{edit_key, EditKey};
use crate::browser::state::{State, View};

/* A key while the address bar has the keyboard. Enter goes (or searches),
 * Esc puts back the address of the page on screen and hands the keyboard
 * to the page, and everything else edits the text. */
pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match edit_key(event.code, event.flags) {
        EditKey::Commit => super::omnibox_commit::commit(state),
        EditKey::Cancel => {
            let home = state.view == View::Home;
            let url = if home { String::new() } else { state.ui.current_url.clone() };
            state.ui.omnibox.set(&url);
            state.ui.omnibox.select_all();
            state.ui.text_off = 0;
            state.focus_page(None);
            state.mark_omnibox();
            EventOutcome::Repaint
        }
        EditKey::Ignore => EventOutcome::Idle,
        k => {
            if !super::omnibox_edit::apply(state, k) {
                return EventOutcome::Idle;
            }
            state.fit_text();
            state.mark_omnibox();
            EventOutcome::Repaint
        }
    }
}
