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

use crate::browser::omnibox::Change;
use crate::browser::state::State;

/* Stop loading: the fetch in flight is closed, nothing queued starts, and
 * the document on screen stays. A result still on its way belongs to the
 * stopped load and is dropped, because its generation is no longer the
 * current one. A page held back for its stylesheets shows what it has. */
pub fn stop(state: &mut State) {
    if let Some(job) = state.fetch.take() {
        let _ = crate::browser::net::socket_close(state.sockets_port, job.handle);
    }
    if state.box_doc.is_none() && state.page_dom.is_some() {
        super::relayout::relayout(state);
    }
    state.pending_nav = None;
    state.pending_post = None;
    state.css_queue.clear();
    state.script_queue.clear();
    state.image_queue.clear();
    state.font_queue.clear();
    state.suppress_history_push = false;
    state.redirect_count = 0;
    state.retries = 0;
    state.ui.nav_gen = state.ui.nav_gen.wrapping_add(1);
    state.status = String::from("stopped");
    state.mark(Change::Full);
}
