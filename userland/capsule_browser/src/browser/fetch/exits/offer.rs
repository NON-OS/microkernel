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

//! The page that offers Anyone, and the switch its link makes.

use alloc::string::String;

use super::rule::{offer_html, SWITCH};
use crate::browser::http;
use crate::browser::net::mixnet::{choose, Network};
use crate::browser::omnibox::Change;
use crate::browser::state::State;

/*
 * The error page is drawn from text flows, which hold no link, so this one
 * is a small HTML page put on screen as a fetched page is, with the switch
 * as an ordinary link. It takes no history entry, as no failure page does,
 * and stays under the address that failed.
 */
/// Show `said` with a link that switches to Anyone and loads the page that
/// failed again.
pub fn offer_anyone(state: &mut State, said: &str) {
    let target = state.ui.last_target.clone();
    let html = offer_html(said);
    let mut raw = alloc::format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n",
        html.len()
    )
    .into_bytes();
    raw.extend_from_slice(html.as_bytes());
    let Some(resp) = http::response::parse(&raw) else { return };
    super::super::commit_doc::commit_doc(state, &resp, raw.len());
    state.anyone_retry = Some(target);
    state.status = String::from(said);
}

/// The offer's link, clicked: choose Anyone, as the settings panel does,
/// and load the page that failed again. False for any other link, and for
/// this one on any page but the offer: a page cannot switch the network.
pub fn switch_to_anyone(state: &mut State, href: &str) -> bool {
    if href.trim() != SWITCH {
        return false;
    }
    let Some(failed) = state.anyone_retry.take() else { return false };
    choose(Network::Anyone);
    state.pending_nav = Some(failed);
    state.tell(String::from("Anyone network from now on; loading the page again."));
    state.mark(Change::Full);
    true
}
