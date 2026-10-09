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

//! The browser's jars, and which network each request and page is on.

use alloc::string::String;
use core::sync::atomic::{AtomicU8, Ordering};

use spin::Mutex;

use super::jars::Jars;
use super::parse::{parse, Origin};
use crate::browser::net::mixnet::Network;
use crate::browser::url::{Scheme, Url};

/*
 * Held in memory and nowhere else: the jars are dropped with the process,
 * which is the whole of "cleared at exit". The route the requests take is
 * already process state (net::mixnet), and the jars sit beside it for the
 * same reason: every fetch and every page script reaches them, from paths
 * that share no owner.
 */
static JARS: Mutex<Jars> = Mutex::new(Jars::new());

/* The network the page on screen was loaded over, which its scripts'
 * document.cookie reads. A request reads the jar of the way its own bytes
 * take (`Fetch::net`), which is not always the page's: an .anyone image on
 * a page over Nym rides Anyone. Nym until told. */
static PAGE: AtomicU8 = AtomicU8::new(1);

fn to_u8(n: Network) -> u8 {
    match n {
        Network::Direct => 0,
        Network::Nym => 1,
        Network::Anyone => 2,
    }
}

fn from_u8(v: u8) -> Network {
    match v {
        0 => Network::Direct,
        2 => Network::Anyone,
        _ => Network::Nym,
    }
}

/// The page now on screen was loaded over `net`.
pub fn set_page(net: Network) {
    PAGE.store(to_u8(net), Ordering::Relaxed);
}

pub fn page() -> Network {
    from_u8(PAGE.load(Ordering::Relaxed))
}

fn origin(url: &Url) -> Origin<'_> {
    Origin { host: &url.host, path: &url.path, secure: url.scheme == Scheme::Https }
}

fn path_of(url: &Url) -> &str {
    let path = url.path.split(['?', '#']).next().unwrap_or("");
    if path.is_empty() {
        "/"
    } else {
        path
    }
}

/// The Cookie header value a request to `url` over `net` carries, if any.
pub fn request_header(net: Network, url: &Url, now: i64) -> Option<String> {
    let jars = JARS.lock();
    let secure = url.scheme == Scheme::Https;
    let value = jars.get(net).header(&url.host, path_of(url), secure, false, now);
    (!value.is_empty()).then_some(value)
}

/// Keep the cookies a response from `url` over `net` set.
pub fn absorb(net: Network, url: &Url, raw: &[u8], now: i64) {
    let lines = super::absorb::set_cookie_lines(raw);
    if lines.is_empty() {
        return;
    }
    let at = origin(url);
    let mut jars = JARS.lock();
    let jar = jars.of(net);
    for line in &lines {
        if let Some(c) = parse(line, &at, now, false) {
            jar.set(c, at.secure, false, now);
        }
    }
}

/// What `document.cookie` reads on the page at `url`.
pub fn script_get(url: &Url, now: i64) -> String {
    let jars = JARS.lock();
    let secure = url.scheme == Scheme::Https;
    jars.get(page()).header(&url.host, path_of(url), secure, true, now)
}

/// A `document.cookie = line` on the page at `url`.
pub fn script_set(url: &Url, line: &str, now: i64) {
    let at = origin(url);
    if let Some(c) = parse(line, &at, now, true) {
        JARS.lock().of(page()).set(c, at.secure, true, now);
    }
}
