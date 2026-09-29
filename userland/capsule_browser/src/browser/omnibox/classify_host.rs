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
}

/* `auth` is the text before any '/', '?' or '#'. A numeric port may follow
 * the host. A public name needs two or more labels of letters, digits and
 * inner hyphens, the last alphabetic and two or more long, or punycode. */
pub fn host_kind(auth: &str) -> Option<HostKind> {
    let host = match auth.rsplit_once(':') {
        Some((h, p)) if !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()) => {
            p.parse::<u16>().ok().filter(|&n| n != 0)?;
            h
        }
        Some(_) => return None,
        None => auth,
    };
    let lower = host.to_ascii_lowercase();
    if lower == "localhost" || lower.ends_with(".localhost") || ipv4(host) {
        return Some(HostKind::Local);
    }
    let labels: alloc::vec::Vec<&str> = host.split('.').collect();
    if labels.len() < 2 || !labels.iter().all(|l| label_ok(l)) {
        return None;
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
