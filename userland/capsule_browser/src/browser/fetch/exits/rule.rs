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

//! When a failed page is the Nym exits' doing, and when to offer Anyone.
//!
//! net.socks5 walks away from an exit that answers nothing for its silence
//! budget, and ends every stream bound to it. The browser saw only a close,
//! and said the exit had refused the site, though no exit had answered at
//! all. net.socks5 now counts the exits it walked away from (its status
//! answer's sixth byte); a navigation reads the count as it begins and as it
//! fails, and a larger count means the exits went silent under it. That is
//! tried again like any failure worth one more try, on the exit net.socks5
//! takes next; once the tries are spent the page says Nym exits are not
//! answering and offers Anyone with one click. A Nym navigation that waited
//! its whole bound for the mixnet to connect offers Anyone as well.
//!
//! Pure; the proofs hold it.

use crate::browser::fetch::closed::EXIT_CLOSED;
use crate::browser::fetch::proxy_fault::{NOT_READY, PROXY_ENDED};
use crate::browser::net::mixnet::Network;

/// The exits net.socks5 tried went silent while this navigation ran.
pub const EXIT_SILENT: &str = "exits silent";

/// Whether a navigation through `net` that stopped with `code` failed
/// because its exits went silent: net.socks5 had walked away from `before`
/// exits when it began and `after` when it stopped.
pub fn exits_silent(net: Network, code: &str, before: Option<u8>, after: Option<u8>) -> bool {
    let closed = matches!(code, EXIT_CLOSED | PROXY_ENDED | "timed out" | "stalled");
    let walked = matches!((before, after), (Some(b), Some(a)) if a > b);
    net == Network::Nym && closed && walked
}

/// Whether the page that says `code` for a navigation through `net`
/// offers to switch to the Anyone network.
pub fn offers_anyone(code: &str, net: Network) -> bool {
    net == Network::Nym && matches!(code, EXIT_SILENT | NOT_READY)
}

/// The address the offer's link goes to.
pub const SWITCH: &str = "about:anyone";

/// The page that says `said` and offers the switch, as HTML.
pub fn offer_html(said: &str) -> alloc::string::String {
    let mut text = alloc::string::String::with_capacity(said.len());
    for c in said.chars() {
        match c {
            '<' => text.push_str("&lt;"),
            '>' => text.push_str("&gt;"),
            '&' => text.push_str("&amp;"),
            c => text.push(c),
        }
    }
    alloc::format!(
        "<!doctype html><title>Nym exits are not answering</title>\
         <h2>Navigation failed</h2><p>{text}</p>\
         <p><a href=\"{SWITCH}\">Switch to the Anyone network and load this page again</a></p>\
         <p>The switch lasts until you choose another network in the browser's settings.</p>"
    )
}
