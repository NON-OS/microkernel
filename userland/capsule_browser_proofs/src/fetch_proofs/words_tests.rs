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

//! Every code a fetch can stop with reaches the reader as one plain
//! sentence: never the bare code ("send failed", "timed out"), and never a
//! sentence that blames the wrong party for the way the request took.

use crate::browser::fetch::closed::EXIT_CLOSED;
use crate::browser::fetch::open::NO_STREAM;
use crate::browser::fetch::proxy_fault::{
    NOT_READY, PROXY_ENDED, PROXY_FULL, PROXY_GARBLED, PROXY_GONE, PROXY_LOST,
};
use crate::browser::fetch::words::{sentence, words};
use crate::browser::net::mixnet::{Network, Way};
use nonos_route_link::{Proxy, ANYONE_NEEDED};

const NYM: Way = Way::Proxy { net: Network::Nym, port: 41, proxy: Proxy::Nym };
const ANYONE: Way = Way::Proxy { net: Network::Anyone, port: 42, proxy: Proxy::Anyone };

/// Every code the fetch machine stops with, by name.
const CODES: [&str; 31] = [
    "connect failed",
    "dns failed",
    "timed out",
    "stalled",
    "kept connection dead",
    "send failed",
    "socks hello failed",
    "socks connect failed",
    "socks auth rejected",
    "socks target rejected",
    "bad socks response",
    "socks connect rejected",
    "socket failed",
    "net.sockets unavailable",
    "bad url",
    "response too large",
    "tls init failed",
    "tls handshake failed",
    "tls handshake refused",
    "tls record failed",
    "tls flight too large",
    EXIT_CLOSED,
    PROXY_GONE,
    PROXY_LOST,
    PROXY_GARBLED,
    PROXY_ENDED,
    NOT_READY,
    PROXY_FULL,
    NO_STREAM,
    "the page could not be fetched",
    ANYONE_NEEDED,
];

/// What every line the reader is shown must be.
fn plain(code: &str, said: &str) {
    assert_ne!(said, code, "{code}: the bare code");
    assert!(said.len() > code.len() + 10 || said.len() > 40, "{code}: {said}");
    assert!(said.ends_with('.') || said.ends_with(": proxy off"), "{code}: no full stop: {said}");
    let first = said.chars().next().unwrap_or(' ');
    assert!(first.is_uppercase() || said.starts_with("example.org") || said.starts_with("net."), "{code}: {said}");
    assert!(!said.contains('\u{2014}'), "{code}: no dashes");
}

#[test]
fn every_code_is_said_as_a_sentence_on_every_way() {
    for way in [Way::Direct, NYM, ANYONE] {
        for code in CODES {
            plain(code, &words(code, way, "example.org"));
        }
    }
}

#[test]
fn a_network_s_refusals_and_absences_are_sentences() {
    for net in [Network::Nym, Network::Anyone] {
        for rep in 1..=6 {
            let said = net.refused(rep);
            plain("socks connect rejected", &words(said, NYM, "example.org"));
        }
        plain("absent", net.absent());
    }
    plain("absent", Network::Direct.absent());
}

#[test]
fn a_close_names_whoever_closed() {
    let exit = words(EXIT_CLOSED, ANYONE, "example.org");
    assert!(exit.starts_with("The exit closed") && exit.contains("Anyone network"), "{exit}");
    let nym = words(EXIT_CLOSED, NYM, "example.org");
    assert!(nym.contains("could not resolve or reach"), "a Nym exit's failed lookup arrives as a close: {nym}");
    let restarted = words(PROXY_LOST, ANYONE, "example.org");
    assert!(restarted.starts_with("net.anon was restarted"), "{restarted}");
    let gone = words(PROXY_GONE, NYM, "example.org");
    assert!(gone.starts_with("net.socks5 is no longer running"), "{gone}");
    assert!(!words(PROXY_LOST, NYM, "example.org").contains("exit"), "the exit is not blamed");
}

#[test]
fn a_timeout_says_how_long_was_waited_on_the_way_taken() {
    let direct = words("timed out", Way::Direct, "example.org");
    assert!(direct.contains("12 seconds"), "{direct}");
    let nym = words("timed out", NYM, "example.org");
    assert!(nym.contains("Nym mixnet") && nym.contains("3 minutes"), "{nym}");
    let anyone = words("timed out", ANYONE, "example.org");
    assert!(anyone.contains("Anyone network") && anyone.contains("30 seconds"), "{anyone}");
    let waited = words(NOT_READY, ANYONE, "example.org");
    assert!(waited.contains("did not finish connecting in 3 minutes"), "{waited}");
}

#[test]
fn a_sentence_already_written_is_kept_with_a_capital_and_a_stop() {
    assert_eq!(
        sentence(ANYONE_NEEDED),
        "An .anyone address is reached only inside the Anyone network, which is not running."
    );
    assert_eq!(sentence("Done."), "Done.");
    assert_eq!(sentence(""), ".");
}
