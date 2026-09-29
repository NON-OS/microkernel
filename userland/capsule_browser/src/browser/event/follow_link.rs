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

use alloc::string::ToString;

use nonos_app_skeleton::EventOutcome;

use crate::browser::omnibox::{link_action, Change, LinkAction};
use crate::browser::state::{Origin, State};
use crate::browser::url;

/* Follow a clicked link. A fragment of the page on screen scrolls to its
 * target and takes a history entry without a fetch; a scheme the browser
 * cannot open (javascript:, mailto:, tel:) leaves the page as it is. */
pub(super) fn follow_link(state: &mut State, href: &str) -> EventOutcome {
    let resolved = match state.base.as_ref() {
        Some(base) => url::join(base, href),
        None => href.to_string(),
    };
    match link_action(&state.ui.current_url, &resolved) {
        LinkAction::Navigate(u) => super::navigate::navigate(state, u, Origin::User),
        LinkAction::Anchor(frag) => {
            super::anchor::scroll_to_fragment(state, &frag);
            state.ui.history.push(&resolved);
            state.ui.current_url = resolved.clone();
            state.show_url(&resolved);
            state.mark(Change::Toolbar);
            EventOutcome::Repaint
        }
        LinkAction::Ignore => EventOutcome::Idle,
    }
}
