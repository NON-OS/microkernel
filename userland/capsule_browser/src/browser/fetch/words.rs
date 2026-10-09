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

//! What the reader is told when a page could not be fetched.
//!
//! A fetch stops with a short code ("connect failed", "timed out", "socks
//! auth rejected"), and the error page and the status line showed that code
//! as it was: a reader was told "send failed" with no word of what had sent
//! what, or whether to wait, retry or look elsewhere. The codes stay, since
//! the retry rule reads them (`retryable_error`), and each is said here as
//! one plain sentence that names what happened and what to do, on the way
//! the request took. A code that is already a sentence (a network's
//! refusal, route_link's words) is said as it is, with a capital and a stop.
//!
//! Pure; the proofs hold every code to a sentence.

use alloc::format;
use alloc::string::String;

use super::budget::budget;
use super::closed::EXIT_CLOSED;
use super::exits::EXIT_SILENT;
use super::open::NO_STREAM;
use super::proxy_fault::{NOT_READY, PROXY_ENDED, PROXY_FULL, PROXY_GARBLED, PROXY_GONE, PROXY_LOST};
use super::socks::hold::HOLD_MS;
use crate::browser::net::mixnet::{Network, Way};

/// The sentence for a fetch of `host` that stopped with `code`, having left
/// (or been about to leave) by `way`.
pub fn words(code: &str, way: Way, host: &str) -> String {
    let host = if host.is_empty() { "the site" } else { host };
    let proxy = service(way);
    /* A sentence begins with the service's own name, or with a capital. */
    let lead = if way.proxied() { proxy } else { "The SOCKS proxy you set" };
    let net = way.network();
    match code {
        "connect failed" => format!(
            "{host} did not accept a connection: it may be down, or the address may be wrong. \
             Try again later."
        ),
        "dns failed" => format!("No address was found for {host}. Check the spelling of the address."),
        "dns unreachable" => format!(
            "Name lookup is not answering: no DNS server is reachable, so {host} could not be \
             looked up. Check the network connection, or choose Nym or Anyone, which look names \
             up at their exit."
        ),
        "timed out" if way.proxied() => format!(
            "No answer came back through the {} in {}. The site, its exit or the network may be \
             down; try again later.",
            net.label(),
            span(budget(net).silent_ms)
        ),
        "timed out" => format!(
            "{host} did not answer in {}. It may be down or overloaded; try again later.",
            span(budget(net).silent_ms)
        ),
        "stalled" => format!("{host} stopped sending part way through. Reload to try again."),
        "kept connection dead" => format!(
            "The connection kept open to {host} had been closed, and a new one did not start. \
             Reload to try again."
        ),
        "send failed" | "socks hello failed" | "socks connect failed" if way.proxied() => format!(
            "{lead} stopped taking the request. Try again; if it keeps happening, it may have \
             stopped running."
        ),
        "send failed" => format!(
            "The connection to {host} broke while the request was being sent. Try again."
        ),
        "socks hello failed" | "socks connect failed" => String::from(
            "The SOCKS proxy you set did not take the request. Check that it is running, or \
             turn it off by typing: proxy off",
        ),
        "socks auth rejected" => String::from(
            "The SOCKS proxy you set asks for a password, which this browser cannot give. Turn \
             it off by typing: proxy off",
        ),
        "socks target rejected" => {
            String::from("That host name is too long to send through a SOCKS proxy.")
        }
        "bad socks response" => String::from(
            "The proxy answered with something that is not SOCKS5, so nothing was sent.",
        ),
        "socks connect rejected" => format!("{lead} refused to connect to {host}."),
        "socket failed" => String::from(
            "net.sockets would not open a connection; it may have run out. Try again in a moment.",
        ),
        "net.sockets unavailable" => String::from(
            "The network service net.sockets is not running, so no page can be fetched. It may \
             still be starting; try again in a moment.",
        ),
        "bad url" => String::from("That address could not be read. Check it and try again."),
        "response too large" => {
            String::from("The page is larger than 4 MB, the most this browser reads.")
        }
        "tls init failed" => format!(
            "A secure connection to {host} could not be started: the browser could not build \
             its first message (no random numbers, or a name too long to send)."
        ),
        "tls handshake failed" => format!(
            "A secure connection to {host} could not be set up: its handshake did not complete."
        ),
        "tls handshake refused" => format!(
            "{host} refused the secure connection. It may need something this browser does not \
             offer."
        ),
        "tls record failed" => format!(
            "A secure record from {host} could not be decrypted, so what came was dropped. \
             Reload to try again."
        ),
        "tls flight too large" => format!(
            "{host} sent a handshake larger than the 512 KB this browser accepts."
        ),
        /* net.socks5 answers the CONNECT before the exit has tried the
         * host, so a name the Nym exit could not resolve, or a host it
         * could not reach, arrives as this close. */
        EXIT_CLOSED if net == Network::Nym => format!(
            "The Nym exit closed the connection to {host} before any answer came: it could not \
             resolve or reach the site, or the site refuses the mixnet. Check the address, or try \
             again later."
        ),
        EXIT_CLOSED if way.proxied() => format!(
            "The exit closed the connection to {host} before any answer came. The site may \
             refuse visitors from the {}; try again later.",
            net.label()
        ),
        EXIT_CLOSED => format!("{host} closed the connection before answering."),
        PROXY_GONE => format!(
            "{lead} is no longer running where this page reached it: it was stopped or \
             restarted. Reload to try again."
        ),
        PROXY_LOST => format!(
            "{lead} was restarted and no longer holds this connection. Reload to try again."
        ),
        PROXY_GARBLED => format!(
            "{lead} sent an answer the browser cannot read, so the page was dropped. Reload to \
             try again."
        ),
        PROXY_ENDED => format!(
            "{lead} ended the connection before it was set up. It may be starting or \
             restarting; try again in a moment."
        ),
        NOT_READY => format!(
            "The {} did not finish connecting in {}, so nothing was sent. Try again later.",
            net.label(),
            span(HOLD_MS)
        ),
        PROXY_FULL => format!(
            "{lead} was serving as many connections as it can for {}. Close other programs \
             using the {}, or try again later.",
            span(HOLD_MS),
            net.label()
        ),
        EXIT_SILENT => String::from(
            "Nym exits are not answering right now; try Anyone. net.socks5 walked away from \
             exits that did not answer, and none of those tried after them answered either.",
        ),
        NO_STREAM => format!(
            "Every connection this browser may hold through {proxy} is in use. Try again in a \
             moment."
        ),
        "the page could not be fetched" => String::from(
            "The page could not be fetched, and the browser has no more detail. Reload to try \
             again.",
        ),
        other => sentence(other),
    }
}

/// `line` as a sentence: a capital first, and a full stop unless it ends
/// with one already.
pub fn sentence(line: &str) -> String {
    let line = line.trim();
    let mut out = String::with_capacity(line.len() + 1);
    let mut chars = line.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.push_str(chars.as_str());
    }
    if !out.ends_with(['.', '!', '?', ':']) {
        out.push('.');
    }
    out
}

/// The service a request through `way` goes to.
fn service(way: Way) -> &'static str {
    match way {
        Way::Proxy { net: Network::Anyone, .. } => "net.anon",
        Way::Proxy { .. } => "net.socks5",
        Way::Direct | Way::Refused(_) => "the SOCKS proxy you set",
    }
}

/// A span of milliseconds as a person says it.
fn span(ms: i64) -> String {
    match ms / 1_000 {
        s if s >= 120 && s % 60 == 0 => format!("{} minutes", s / 60),
        s => format!("{s} seconds"),
    }
}
