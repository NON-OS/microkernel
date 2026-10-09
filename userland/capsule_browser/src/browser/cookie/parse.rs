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

//! One Set-Cookie line, or one `document.cookie = ...`, read into a cookie.

use alloc::string::{String, ToString};

use super::matching::{default_path, domain_match};
use super::types::Cookie;

/// The longest name and value together a cookie may have, RFC 6265bis 5.6.
pub const MAX_NAME_VALUE: usize = 4096;
/// Attribute values longer than this are ignored, RFC 6265bis 5.6.
const MAX_ATTRIBUTE: usize = 1024;

/// Where the cookie came from: the request's host and path, and whether it
/// arrived over https.
pub struct Origin<'a> {
    pub host: &'a str,
    pub path: &'a str,
    pub secure: bool,
}

/*
 * The steps of RFC 6265 section 5.2 and the storage checks of 5.3, with the
 * later draft's tightening where it closes a hole: a cookie a page script
 * sets can never be HttpOnly, a Secure cookie arrives only over https, and
 * the __Secure- and __Host- name prefixes mean what they promise.
 *
 * There is no public suffix list here, so a Domain attribute naming a
 * single label ("com") is refused rather than letting one site set a cookie
 * for every site under that label. A Domain the request's host does not
 * sit under is refused outright.
 */
/// The cookie `line` sets for a response from `at` at `now` (Unix seconds),
/// or `None` if it is to be ignored. `from_script` is a `document.cookie`
/// write rather than a header.
pub fn parse(line: &str, at: &Origin, now: i64, from_script: bool) -> Option<Cookie> {
    let mut parts = line.split(';');
    let pair = parts.next()?;
    let eq = pair.find('=')?;
    let name = pair[..eq].trim_matches(is_space);
    let value = pair[eq + 1..].trim_matches(is_space);
    if name.is_empty() || name.len() + value.len() > MAX_NAME_VALUE {
        return None;
    }
    if !name.chars().chain(value.chars()).all(allowed) {
        return None;
    }
    let mut attrs = Attrs::default();
    for attr in parts {
        attrs.feed(attr, now);
    }
    let host = at.host.to_ascii_lowercase();
    let (domain, host_only) = match attrs.domain {
        Some(d) if !domain_match(&host, &d) => return None,
        Some(d) if !d.contains('.') && d != host => return None,
        Some(d) => (d, false),
        None => (host, true),
    };
    let path = attrs.path.unwrap_or_else(|| default_path(at.path).to_string());
    if (attrs.secure && !at.secure) || (from_script && attrs.http_only) {
        return None;
    }
    let lower = name.to_ascii_lowercase();
    if lower.starts_with("__secure-") && !(attrs.secure && at.secure) {
        return None;
    }
    if lower.starts_with("__host-") && !(attrs.secure && at.secure && host_only && path == "/") {
        return None;
    }
    Some(Cookie {
        name: name.to_string(),
        value: value.to_string(),
        domain,
        host_only,
        path,
        secure: attrs.secure,
        http_only: attrs.http_only,
        expires: attrs.max_age.or(attrs.expires),
        seq: 0,
    })
}

#[derive(Default)]
struct Attrs {
    expires: Option<i64>,
    max_age: Option<i64>,
    domain: Option<String>,
    path: Option<String>,
    secure: bool,
    http_only: bool,
}

impl Attrs {
    fn feed(&mut self, attr: &str, now: i64) {
        let (key, value) = match attr.find('=') {
            Some(eq) => (&attr[..eq], &attr[eq + 1..]),
            None => (attr, ""),
        };
        let key = key.trim_matches(is_space);
        let value = value.trim_matches(is_space);
        if value.len() > MAX_ATTRIBUTE {
            return;
        }
        if key.eq_ignore_ascii_case("expires") {
            if let Some(at) = super::date::cookie_date(value) {
                self.expires = Some(at);
            }
        } else if key.eq_ignore_ascii_case("max-age") {
            if let Some(secs) = max_age(value) {
                /* Zero or less means now: the cookie is deleted. */
                self.max_age = Some(if secs <= 0 { i64::MIN } else { now.saturating_add(secs) });
            }
        } else if key.eq_ignore_ascii_case("domain") {
            let d = value.strip_prefix('.').unwrap_or(value);
            if !d.is_empty() {
                self.domain = Some(d.to_ascii_lowercase());
            }
        } else if key.eq_ignore_ascii_case("path") {
            self.path = value.starts_with('/').then(|| value.to_string());
        } else if key.eq_ignore_ascii_case("secure") {
            self.secure = true;
        } else if key.eq_ignore_ascii_case("httponly") {
            self.http_only = true;
        }
    }
}

/// A Max-Age value: an optional minus and digits, nothing else.
fn max_age(value: &str) -> Option<i64> {
    let digits = value.strip_prefix('-').unwrap_or(value);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    /* More digits than an i64 holds is a very long time, not an error. */
    let n = digits.parse::<i64>().unwrap_or(i64::MAX);
    Some(if value.starts_with('-') { -n } else { n })
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

/* Control characters would let a value split the Cookie header it is
 * later written into. */
fn allowed(c: char) -> bool {
    c == '\t' || !(c.is_ascii_control())
}
