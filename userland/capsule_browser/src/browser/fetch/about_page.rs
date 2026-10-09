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

use crate::browser::http;
use crate::browser::state::{State, View};

use super::about_html::ENGINE_HTML;

/* Serve about: addresses locally. Returns true when the target was
 * handled. about:loads lists how this window's page loads ended;
 * about:engine shows the layout and script engine working, and any other
 * about: name gets the same page. The page is served as a raw HTTP
 * response through the same parser a fetched page goes through, so it
 * carries whatever a parsed response carries. */
pub fn about_page(state: &mut State, target: &str) -> bool {
    if !target.starts_with("about:") {
        return false;
    }
    /* about:loads lists how the window's page loads ended (`load_log`). */
    let loads;
    let body: &[u8] = if target.eq_ignore_ascii_case("about:loads") {
        loads = super::load_log::page();
        loads.as_bytes()
    } else {
        ENGINE_HTML
    };
    let mut raw = alloc::format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n",
        body.len()
    )
    .into_bytes();
    raw.extend_from_slice(body);
    state.focus = None;
    /* Its history entry first, as for any page (finish). */
    let suppress = core::mem::take(&mut state.suppress_history_push);
    super::committed::committed(state, target, suppress);
    match http::response::parse(&raw) {
        Some(resp) => super::commit_doc::commit_doc(state, &resp, raw.len()),
        None => {
            state.status = String::from("about page did not parse");
            state.document = Some(super::render_error::render_error("about page did not parse"));
            state.box_doc = None;
        }
    }
    state.status = alloc::format!("{} {}", target, state.status);
    state.view = View::Page;
    true
}
