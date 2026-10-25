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

//! A GET to a package mirror, named and reached through the network the
//! person chose (route.rs). Only a development build (`dev-mirror`) dials a
//! mirror on the local network directly by address, as raw.rs says.

use alloc::format;
use alloc::vec::Vec;

use super::super::http_reply::{body, complete};
use super::super::mirror::mirror;
#[cfg(feature = "dev-mirror")]
use super::direct::direct;
use super::route::exchange;
#[cfg(feature = "dev-mirror")]
use crate::linux::net::route::is_local;

/// Enough for the largest package index; a reply beyond it is refused
/// rather than truncated into a half-parsed index.
pub(super) const MAX_BODY: usize = 64 << 20;

/// A GET to Alpine's mirror.
pub fn get(path: &str) -> Option<Vec<u8>> {
    let (at, port, host) = mirror();
    get_as(at, port, host, path)
}

/// A GET to `at`, a mirror's name or a dotted IPv4 address, with `host` as
/// the Host line.
pub fn get_as(at: &str, port: u16, host: &str, path: &str) -> Option<Vec<u8>> {
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos\r\nConnection: close\r\n\r\n"
    );
    #[cfg(feature = "dev-mirror")]
    let raw = match is_local(at) {
        true => direct(at, port, req.as_bytes()),
        false => exchange(at, port, req.as_bytes(), MAX_BODY, &complete),
    };
    #[cfg(not(feature = "dev-mirror"))]
    let raw = exchange(at, port, req.as_bytes(), MAX_BODY, &complete);
    let got = raw.and_then(body);
    let line = match &got {
        Some(b) => format!("[LINUX] mirror {at}: {path}, {} bytes\n", b.len()),
        None => format!("[LINUX] mirror {at}: GET {path} gave no whole 200 reply\n"),
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    got
}
