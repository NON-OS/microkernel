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

use crate::browser::fetch::constants;
use crate::browser::omnibox::Change;
use crate::browser::state::{Origin, State, View};
use crate::browser::url;

/* Follow a 3xx. The hop continues the same navigation: it keeps the
 * history mode of the load it came from (`suppress`: rewrite the current
 * entry), leaves keyboard focus and the address bar alone, and the final
 * address is what history records at commit. */
pub(super) fn redirect(state: &mut State, location: String, suppress: bool) {
    state.view = View::Page;
    if state.redirect_count >= constants::MAX_REDIRECTS {
        state.redirect_count = 0;
        state.status = String::from("too many redirects");
        state.document = None;
        state.box_doc = None;
        state.mark(Change::Full);
        return;
    }
    state.redirect_count += 1;
    let next = match &state.base {
        Some(b) => url::join(b, &location),
        None => location,
    };
    state.status = alloc::format!("redirecting to {}", next);
    state.suppress_history_push = suppress;
    state.pending_nav = Some(next);
    state.ui.origin = Origin::Auto;
}
