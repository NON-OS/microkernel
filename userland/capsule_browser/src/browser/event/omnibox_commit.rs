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

use nonos_app_skeleton::EventOutcome;

use crate::browser::omnibox::{classify, Nav};
use crate::browser::state::{Origin, State};

/* Enter in the address bar: run a proxy command, or go to the address the
 * text names, or search for it. The bar then shows where it is going and
 * the keyboard moves to the page, as in any mainstream browser. */
pub(super) fn commit(state: &mut State) -> EventOutcome {
    let text = state.ui.omnibox.text.clone();
    if crate::browser::proxy::command(state, &text) {
        let url = state.ui.current_url.clone();
        state.ui.omnibox.set(&url);
        state.focus_page(None);
        return EventOutcome::Repaint;
    }
    match classify(&text, &state.ui.search) {
        Nav::Nothing => EventOutcome::Idle,
        Nav::Internal(u) | Nav::Url(u) | Nav::Search(u) => {
            state.ui.omnibox.set(&u);
            state.ui.text_off = 0;
            state.focus_page(None);
            state.mark_omnibox();
            super::navigate::navigate(state, u, Origin::Omnibox)
        }
    }
}
