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

//! The cookies of one network, bounded.

use alloc::string::String;
use alloc::vec::Vec;

use super::matching::{domain_match, path_match};
use super::types::Cookie;

/// Cookies kept at once. A page that sets more loses its oldest first.
pub const MAX_COOKIES: usize = 300;
/// Cookies kept for one domain, so one site cannot push every other out.
pub const MAX_PER_DOMAIN: usize = 50;

/*
 * Worst case held: 300 cookies of at most 4 KiB of name and value each,
 * about 1.2 MiB, against the capsule's 96 MiB heap. RFC 6265 section 6.1
 * asks for at least 3000 cookies and 50 per domain; the total is lower
 * here because nothing is kept past the session.
 */
pub struct Jar {
    cookies: Vec<Cookie>,
    next_seq: u64,
}

impl Default for Jar {
    fn default() -> Self {
        Self::new()
    }
}

impl Jar {
    pub const fn new() -> Jar {
        Jar { cookies: Vec::new(), next_seq: 1 }
    }

    /*
     * RFC 6265 section 5.3 steps 11 and 12: a cookie replaces the one with
     * the same name, domain and path and keeps its creation time; one that
     * arrives already expired only deletes. A script may not replace an
     * HttpOnly cookie, and a cookie from plain http may not replace a
     * Secure one (RFC 6265bis 5.7 step 16), or either could be overwritten
     * by whoever can run script or sit on the wire.
     */
    /// Store `c`, set from `origin_secure` (https) or not, at `now`.
    /// Returns whether the jar took it.
    pub fn set(&mut self, mut c: Cookie, origin_secure: bool, from_script: bool, now: i64) -> bool {
        let same = self
            .cookies
            .iter()
            .position(|o| o.name == c.name && o.domain == c.domain && o.path == c.path);
        if let Some(at) = same {
            let old = &self.cookies[at];
            if (old.http_only && from_script) || (old.secure && !origin_secure && !c.secure) {
                return false;
            }
            c.seq = old.seq;
            self.cookies.remove(at);
        } else {
            c.seq = self.next_seq;
            self.next_seq += 1;
        }
        if c.expired(now) {
            return true;
        }
        let domain = c.domain.clone();
        self.cookies.push(c);
        self.evict(&domain, now);
        true
    }

    /// The `Cookie` header value for a request to `host` at `path`, or what
    /// `document.cookie` reads when `for_script`.
    pub fn header(&self, host: &str, path: &str, secure: bool, for_script: bool, now: i64) -> String {
        let host = host.to_ascii_lowercase();
        let mut sent: Vec<&Cookie> = self
            .cookies
            .iter()
            .filter(|c| !c.expired(now))
            .filter(|c| if c.host_only { c.domain == host } else { domain_match(&host, &c.domain) })
            .filter(|c| path_match(path, &c.path))
            .filter(|c| secure || !c.secure)
            .filter(|c| !(for_script && c.http_only))
            .collect();
        /* Longer paths first, then older first: RFC 6265 section 5.4 step 2. */
        sent.sort_by(|a, b| b.path.len().cmp(&a.path.len()).then(a.seq.cmp(&b.seq)));
        let mut out = String::new();
        for c in sent {
            if !out.is_empty() {
                out.push_str("; ");
            }
            out.push_str(&c.name);
            out.push('=');
            out.push_str(&c.value);
        }
        out
    }

    fn evict(&mut self, domain: &str, now: i64) {
        self.cookies.retain(|c| !c.expired(now));
        while self.cookies.iter().filter(|c| c.domain == domain).count() > MAX_PER_DOMAIN {
            self.drop_oldest(|c| c.domain == domain);
        }
        while self.cookies.len() > MAX_COOKIES {
            self.drop_oldest(|_| true);
        }
    }

    fn drop_oldest(&mut self, pick: impl Fn(&Cookie) -> bool) {
        let oldest = self
            .cookies
            .iter()
            .enumerate()
            .filter(|(_, c)| pick(c))
            .min_by_key(|(_, c)| c.seq)
            .map(|(i, _)| i);
        if let Some(i) = oldest {
            self.cookies.remove(i);
        }
    }
}
