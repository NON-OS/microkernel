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
//! HTTP/1.1 for a client, with no I/O of its own.

//! The address a download is asked for, and where a redirect points.

use alloc::string::String;

/// Longest address looked at.
const MAX_URL: usize = 2048;

/// An `https://host[:port]/path?query` address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DlUrl {
    pub host: String,
    pub port: u16,
    /// The path and query as sent, always starting with '/'.
    pub target: String,
}

/// Read an address. Only https: over plain http the exit relay at the end of
/// an Anyone circuit reads the file and can change it, and nothing here could
/// tell. A bare `host/path` is taken as https.
pub fn parse(raw: &str) -> Result<DlUrl, &'static str> {
    let raw = raw.trim();
    if raw.len() > MAX_URL {
        return Err("That address is too long.");
    }
    if raw.starts_with("http://") {
        return Err("Only https addresses: over plain http the exit relay could read and change the file.");
    }
    let rest = raw.strip_prefix("https://").unwrap_or(raw);
    if rest.contains("://") {
        return Err("Only https addresses can be downloaded.");
    }
    let rest = rest.split('#').next().unwrap_or("");
    let cut = rest.find(['/', '?']).unwrap_or(rest.len());
    let (authority, target) = rest.split_at(cut);
    if authority.contains('@') {
        return Err("An address with a user name in it is not downloaded.");
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, p.parse::<u16>().ok().filter(|p| *p != 0).ok_or("That address has a bad port.")?),
        None => (authority, 443),
    };
    if host.is_empty() || host.len() > 253 || !host.bytes().all(host_byte) {
        return Err("That address has no host this can reach.");
    }
    let target = if target.is_empty() || target.starts_with('?') {
        let mut t = String::from("/");
        t.push_str(target);
        t
    } else {
        String::from(target)
    };
    if !target.bytes().all(target_byte) {
        return Err("That address has characters a request cannot carry.");
    }
    Ok(DlUrl { host: host.to_ascii_lowercase(), port, target })
}

/// Where a redirect's `Location` points from `base`: a whole address, one
/// without its scheme (`//host/path`), a path from the root, or one relative
/// to the current path.
pub fn resolve(base: &DlUrl, location: &str) -> Result<DlUrl, &'static str> {
    let location = location.trim();
    if location.starts_with("http://") {
        return Err("The server sent the download to a plain http address, which is not followed.");
    }
    if location.starts_with("https://") {
        return parse(location);
    }
    if let Some(rest) = location.strip_prefix("//") {
        return parse(rest);
    }
    let mut target = String::new();
    if location.starts_with('/') {
        target.push_str(location);
    } else {
        let path = base.target.split('?').next().unwrap_or("/");
        let dir = &path[..path.rfind('/').map_or(1, |i| i + 1)];
        target.push_str(dir);
        target.push_str(location);
    }
    if !target.bytes().all(target_byte) {
        return Err("The server's redirect has characters a request cannot carry.");
    }
    Ok(DlUrl { host: base.host.clone(), port: base.port, target })
}

fn host_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'.' || b == b'-'
}

/// Printable, no space, nothing that would end the request line or start a
/// header.
fn target_byte(b: u8) -> bool {
    b.is_ascii_graphic()
}
