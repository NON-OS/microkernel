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
use crate::browser::state::{Origin, State};

/* Queue the navigation a form submission makes: a POST carries the fields
 * as its body, a GET appends them to the target's query. */
pub(super) fn form_send(state: &mut State, target: String, body: String, post: bool) {
    if target.is_empty() {
        return;
    }
    if post {
        state.pending_post = Some(body);
        state.pending_nav = Some(target);
    } else {
        let sep = if target.contains('?') { '&' } else { '?' };
        let mut t = target;
        if !body.is_empty() {
            t.push(sep);
            t.push_str(&body);
        }
        state.pending_nav = Some(t);
    }
    state.ui.origin = Origin::User;
    state.status = String::from("submitting form");
    state.mark(Change::Toolbar);
}
