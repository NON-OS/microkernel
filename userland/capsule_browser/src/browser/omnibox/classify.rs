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

use alloc::string::{String, ToString};

use super::classify_host::{host_kind, HostKind};
use super::classify_scheme::scheme;
use super::query_encode::encode_query;
use crate::browser::url;

/* What pressing Enter on the address bar text should do. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Nav {
    Nothing,
    /* An about: page served without the network. */
    Internal(String),
    Url(String),
    /* A search for the text, already built into a URL from the template. */
    Search(String),
}

/* Decide between an address and a search, the way mainstream browsers do.
 * An http(s) URL is used as typed; text that names a host (a dotted name
 * with a real top-level label, localhost, or an IPv4 address, with an
 * optional port and path) gets a scheme, plain http for a local host and
 * https otherwise; anything else, words with spaces included, is searched
 * through `search`, whose "%s" takes the encoded text. */
pub fn classify(input: &str, search: &str) -> Nav {
    let s = input.trim();
    if s.is_empty() {
        return Nav::Nothing;
    }
    if s.get(..6).is_some_and(|p| p.eq_ignore_ascii_case("about:")) {
        return Nav::Internal(alloc::format!("about:{}", &s[6..]));
    }
    let stop = s.find(['/', '?', '#']).unwrap_or(s.len());
    match scheme(s, stop) {
        Some(name) if name.eq_ignore_ascii_case("http") || name.eq_ignore_ascii_case("https") => {
            if url::parse(s).is_some() {
                return Nav::Url(s.to_string());
            }
        }
        Some(_) => {}
        None if !s.contains(char::is_whitespace) => {
            let plain = match host_kind(&s[..stop]) {
                Some(HostKind::Local) => Some("http"),
                Some(HostKind::Public) => Some("https"),
                None => None,
            };
            if let Some(scheme) = plain {
                let u = alloc::format!("{}://{}", scheme, s);
                if url::parse(&u).is_some() {
                    return Nav::Url(u);
                }
            }
        }
        None => {}
    }
    Nav::Search(search.replacen("%s", &encode_query(s), 1))
}
