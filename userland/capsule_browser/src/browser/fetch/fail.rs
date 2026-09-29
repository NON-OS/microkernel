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

use crate::browser::fetch::{constants, render_error, retryable_error, security_error};
use crate::browser::omnibox::{should_commit, Change};
use crate::browser::state::{State, View};

/* The document load failed. A transient network error retries the address
 * that was being loaded (not whatever is typed in the address bar); any
 * other error shows its page, under the address that failed. A failure of
 * a load that was stopped or overtaken is dropped. */
pub(super) fn fail(state: &mut State, msg: &str) {
    if !should_commit(state.ui.nav_gen, state.ui.loading_gen) {
        return;
    }
    let target = state.ui.last_target.clone();
    if retryable_error::retryable_error(msg)
        && !security_error::security_error(msg)
        && state.retries < constants::MAX_RETRIES
        && !target.is_empty()
    {
        state.retries += 1;
        state.status = alloc::format!("retry {} - {}", state.retries, msg);
        state.pending_nav = Some(target);
    } else {
        state.retries = 0;
        state.status = String::from(msg);
        state.document = Some(render_error::render_error(msg));
        state.box_doc = None;
        state.engine = None;
        state.page_dom = None;
        state.world = None;
        state.view = View::Page;
        state.ui.current_url = target.clone();
        state.show_url(&target);
        state.mark(Change::Full);
    }
}
