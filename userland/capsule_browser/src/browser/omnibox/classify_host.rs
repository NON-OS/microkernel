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

/* Whether typed text names a host, and which kind. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostKind {
    /* localhost or an IPv4 literal: reached over plain http by default. */
    Local,
    Public,
    /* An .anyone onion service: reached over plain http by default. The
     * onion circuit already encrypts it end to end and proves the service's
     * key, and no certificate authority signs an .anyone name, so https by
     * default would refuse every service that serves http, which is most. */
    Onion,
}

/* `auth` is the text before any '/', '?' or '#'; a port may follow the host,
 * and makes a one-label host (nas:5000) local. A public name has two or more
 * labels of letters, digits and inner hyphens, the last 2+ letters, or xn--. */
pub fn host_kind(auth: &str) -> Option<HostKind> {
    let (host, port) = match auth.rsplit_once(':') {
        Some((h, p)) if p.bytes().all(|b| b.is_ascii_digit()) && p.parse::<u16>().ok()? != 0 => {
            (h, true)
        }
        Some(_) => return None,
        None => (auth, false),
    };
    let lower = host.to_ascii_lowercase();
    let lan = port && !host.contains('.') && label_ok(host);
    if lower == "localhost" || lower.ends_with(".localhost") || ipv4(host) || lan {
        return Some(HostKind::Local);
    }
    let labels: alloc::vec::Vec<&str> = host.split('.').collect();
    if labels.len() < 2 || !labels.iter().all(|l| label_ok(l)) {
        return None;
    }
    if nonos_route_link::is_anyone(host) {
        return Some(HostKind::Onion);
    }
    let tld = labels[labels.len() - 1];
    let alpha = tld.len() >= 2 && tld.bytes().all(|b| b.is_ascii_alphabetic());
    (alpha || lower.rsplit('.').next().is_some_and(|t| t.starts_with("xn--")))
        .then_some(HostKind::Public)
}

fn label_ok(l: &str) -> bool {
    !l.is_empty()
        && l.len() <= 63
        && !l.starts_with('-')
        && !l.ends_with('-')
        && l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

fn ipv4(host: &str) -> bool {
    let parts: alloc::vec::Vec<&str> = host.split('.').collect();
    parts.len() == 4
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.len() <= 3
                && p.bytes().all(|b| b.is_ascii_digit())
                && p.parse::<u16>().is_ok_and(|n| n <= 255)
        })
}
