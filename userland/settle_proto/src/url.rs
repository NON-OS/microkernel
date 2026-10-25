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

//! FETCH RELEASE's url, read strictly: "https://", a DNS name (letters,
//! digits, '-' and '.'), then a path of visible ASCII. No port, no user, no
//! fragment, so the host the TLS checks is the host the url names.

/// The host and the path (at least "/") of `url`.
pub fn https_target(url: &[u8]) -> Option<(&str, &str)> {
    let rest = url.strip_prefix(b"https://")?;
    let cut = rest.iter().position(|b| *b == b'/').unwrap_or(rest.len());
    let (host, path) = rest.split_at(cut);
    let name = |b: &u8| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'.';
    let dotted = host.contains(&b'.') && !host.starts_with(b".") && !host.ends_with(b".");
    if host.is_empty() || host.len() > 253 || !dotted || !host.iter().all(name) {
        return None;
    }
    if !path.iter().all(|b| b.is_ascii_graphic() && *b != b'#') {
        return None;
    }
    let host = core::str::from_utf8(host).ok()?;
    let path = core::str::from_utf8(path).ok()?;
    Some((host, if path.is_empty() { "/" } else { path }))
}
