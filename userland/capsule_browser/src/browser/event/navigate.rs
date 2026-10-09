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

use crate::browser::omnibox::Change;
use crate::browser::state::{Origin, State, View};

/* Queue a navigation; the tick starts it. */
pub(super) fn navigate(state: &mut State, url: String, origin: Origin) -> EventOutcome {
    state.status = alloc::format!("loading {}", url);
    state.pending_nav = Some(url);
    state.ui.origin = origin;
    state.mark(Change::Toolbar);
    EventOutcome::Repaint
}

/* Load the page on screen again, in place in history. A page that failed
 * to load has no current address yet; its last target is tried again. The
 * home page has nothing to reload. */
pub(super) fn reload(state: &mut State) -> EventOutcome {
    if state.view == View::Home {
        return EventOutcome::Idle;
    }
    let url = match state.ui.current_url.is_empty() {
        false => state.ui.current_url.clone(),
        true => state.ui.last_target.clone(),
    };
    if url.is_empty() {
        return EventOutcome::Idle;
    }
    state.suppress_history_push = true;
    navigate(state, url, Origin::User)
}

/* The home page. The load in flight stops and the page's script engine is
 * dropped, so nothing of the page keeps running or pulls the view back;
 * Back returns to the page that was left. */
pub(super) fn go_home(state: &mut State) -> EventOutcome {
    super::stop::stop(state);
    state.engine = None;
    state.world = None;
    state.view = View::Home;
    state.ui.hover_href = None;
    state.ui.omnibox.set("");
    state.ui.text_off = 0;
    state.focus_omnibox();
    state.mark(Change::Full);
    EventOutcome::Repaint
}
