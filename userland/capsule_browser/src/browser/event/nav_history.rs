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

use nonos_app_skeleton::EventOutcome;

use crate::browser::omnibox::{same_document, Change};
use crate::browser::state::{Origin, State, View};

/* Back and Forward, and a page's history.go(delta). They always answer,
 * even while images or scripts are still loading: the navigation they
 * queue cancels whatever is in flight. From the home page, Back returns to
 * the page that was left. A step to another fragment of the page on screen
 * only scrolls. */
pub fn nav_history(state: &mut State, delta: i32) -> EventOutcome {
    let home = state.view == View::Home;
    let h = &mut state.ui.history;
    let target = match (home, delta) {
        (true, -1) => h.current(),
        (_, delta) => h.go(delta),
    };
    let Some(url) = target.map(String::from) else {
        return EventOutcome::Idle;
    };
    let shown = state.box_doc.is_some() || state.document.is_some();
    if !home && shown && same_document(&state.ui.current_url, &url) {
        let frag = url.split_once('#').map_or("", |(_, f)| f);
        super::anchor::scroll_to_fragment(state, frag);
        state.ui.current_url = url.clone();
        state.show_url(&url);
        state.note_history();
        state.mark(Change::Toolbar);
        return EventOutcome::Repaint;
    }
    state.suppress_history_push = true;
    super::navigate::navigate(state, url, Origin::User)
}
