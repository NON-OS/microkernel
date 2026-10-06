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

//! Which hosts and paths a cookie belongs to, RFC 6265 sections 5.1.3 and 5.1.4.

/// Whether `host` is `domain` or a name under it. An IP address matches
/// only itself: "1.2.3.4" is not under "2.3.4".
pub fn domain_match(host: &str, domain: &str) -> bool {
    if host.eq_ignore_ascii_case(domain) {
        return true;
    }
    if is_ip(host) || domain.is_empty() || host.len() <= domain.len() {
        return false;
    }
    let cut = host.len() - domain.len();
    host.as_bytes()[cut - 1] == b'.' && host[cut..].eq_ignore_ascii_case(domain)
}

/// Whether a request for `path` carries a cookie scoped to `cookie_path`:
/// the same path, or one under it at a `/` boundary, so "/shop" is not
/// sent to "/shopping".
pub fn path_match(path: &str, cookie_path: &str) -> bool {
    if path == cookie_path {
        return true;
    }
    path.starts_with(cookie_path)
        && (cookie_path.ends_with('/') || path.as_bytes().get(cookie_path.len()) == Some(&b'/'))
}

/// The path a cookie set without one gets: the request path up to, not
/// including, its last `/`, or "/" when that leaves nothing.
pub fn default_path(request_path: &str) -> &str {
    let path = request_path.split(['?', '#']).next().unwrap_or("");
    if !path.starts_with('/') {
        return "/";
    }
    match path.rfind('/') {
        Some(0) | None => "/",
        Some(at) => &path[..at],
    }
}

fn is_ip(host: &str) -> bool {
    host.contains(':') || (!host.is_empty() && host.bytes().all(|b| b.is_ascii_digit() || b == b'.'))
}
