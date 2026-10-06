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

use crate::browser::state::State;

/* Record the committed document in session history: a new visit adds an
 * entry, while a reload, a back/forward step or a redirect after one
 * (`replace`) rewrites the current entry in place. */
pub(super) fn record_history(state: &mut State, replace: bool) {
    let url = state.ui.current_url.clone();
    if url.is_empty() {
        return;
    }
    if replace {
        state.ui.history.replace(&url);
    } else {
        state.ui.history.push(&url);
    }
    state.note_history();
}
