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

//! When a failed page is said to be the Nym exits' silence, and when it
//! offers the Anyone network. Exits that answered nothing ended the stream
//! from net.socks5's side, and the page blamed the site.

use crate::browser::fetch::closed::EXIT_CLOSED;
use crate::browser::fetch::exits::{exits_silent, offer_html, offers_anyone, EXIT_SILENT, SWITCH};
use crate::browser::fetch::proxy_fault::{NOT_READY, PROXY_GONE, PROXY_LOST};
use crate::browser::fetch::retryable_error::{retry_nav, retryable_error};
use crate::browser::fetch::words::words;
use crate::browser::net::mixnet::status::{read, Progress};
use crate::browser::net::mixnet::{still, Heard, Network, Way};
use nonos_route_link::Proxy;

const NYM: Way = Way::Proxy { net: Network::Nym, port: 41, proxy: Proxy::Nym };

#[test]
fn a_close_after_net_socks5_walked_away_from_an_exit_is_the_exits() {
    assert!(exits_silent(Network::Nym, EXIT_CLOSED, Some(0), Some(1)));
    assert!(exits_silent(Network::Nym, "timed out", Some(2), Some(5)));
    assert!(exits_silent(Network::Nym, "proxy ended", Some(1), Some(2)));
}

#[test]
fn a_close_with_no_exit_walked_away_from_is_the_site_s() {
    assert!(!exits_silent(Network::Nym, EXIT_CLOSED, Some(3), Some(3)), "the same exit answered");
    assert!(!exits_silent(Network::Nym, EXIT_CLOSED, None, Some(3)), "no count when it began");
    assert!(!exits_silent(Network::Nym, EXIT_CLOSED, Some(3), None), "no answer when it ended");
}

#[test]
fn only_a_close_or_silence_on_nym_counts() {
    assert!(!exits_silent(Network::Anyone, EXIT_CLOSED, Some(0), Some(1)));
    assert!(!exits_silent(Network::Nym, "dns failed", Some(0), Some(1)));
    assert!(!exits_silent(Network::Nym, PROXY_LOST, Some(0), Some(1)), "a restart is said as one");
    assert!(!exits_silent(Network::Nym, PROXY_GONE, Some(0), Some(1)));
}

#[test]
fn silent_exits_are_tried_again_then_anyone_is_offered() {
    assert!(retryable_error(EXIT_SILENT), "net.socks5 takes another exit");
    assert!(!retry_nav(EXIT_SILENT, true, true), "never a POST the server may have acted on");
    assert!(offers_anyone(EXIT_SILENT, Network::Nym));
    assert!(offers_anyone(NOT_READY, Network::Nym), "the open never answered in the hold budget");
    assert!(!offers_anyone(NOT_READY, Network::Anyone));
    assert!(!offers_anyone(EXIT_CLOSED, Network::Nym), "a site that hung up is not the exits");
    let said = words(EXIT_SILENT, NYM, "example.org");
    assert!(said.starts_with("Nym exits are not answering right now; try Anyone."), "{said}");
}

#[test]
fn the_offer_is_one_link_and_carries_the_sentence_safely() {
    let page = offer_html("a <b> & c");
    assert!(page.contains("a &lt;b&gt; &amp; c"));
    assert_eq!(page.matches("<a href=").count(), 1);
    assert!(page.contains(&format!("href=\"{SWITCH}\"")));
}

#[test]
fn net_socks5_says_how_many_exits_it_walked_away_from() {
    let p = read(&[3, 1, 0, 3, 3, 4]).expect("six bytes");
    assert_eq!((p.step, p.silent), (3, Some(4)));
    assert_eq!(read(&[3, 1, 0, 5, 5]).map(|p| p.silent), Some(None), "net.anon gives five");
    assert_eq!(read(&[3, 1, 0, 3, 3, 4, 0]), None);
    let trying = Heard::Known(Progress { ready: false, step: 3, steps: 3, silent: Some(3) });
    assert_eq!(
        still(Network::Nym, Some(trying)).as_deref(),
        Some("Nym: exit did not answer, trying another (3)")
    );
}
