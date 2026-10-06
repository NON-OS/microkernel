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

//! A GET to a package mirror. A mirror on the local network is dialled
//! directly by address, as raw.rs says; any other is named and reached through
//! the network the person chose (http_route.rs).

use alloc::format;
use alloc::vec::Vec;

use super::http_reply::{body, complete};
use super::http_route::{down, exchange};
use super::mirror::mirror;
use super::route_read::offline;
use super::Why;
use crate::linux::net::raw::{connect_host, open_stream_to};
use crate::linux::net::raw_io::{close, recv_until, send_all};
use crate::linux::net::route::is_local;

/// Enough for the largest package index; a reply beyond it is refused
/// rather than truncated into a half-parsed index.
const MAX_BODY: usize = 64 << 20;
/// How long a mirror may go silent: one fetching upstream before it answers
/// sends nothing for a while, and a slow link for longer under emulation.
const IDLE_MS: u64 = 120_000;

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
    let raw = match is_local(at) {
        true => direct(at, port, req.as_bytes()),
        false => exchange(at, port, req.as_bytes(), MAX_BODY, &complete),
    };
    let got = raw.and_then(body);
    let line = match &got {
        Some(b) => format!("[LINUX] mirror {at}: {path}, {} bytes\n", b.len()),
        None => format!("[LINUX] mirror {at}: GET {path} gave no whole 200 reply\n"),
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    got
}

/// Whether an install from `at` can start now: a mirror on the local
/// network at once; any other once Anyone, which installs download over,
/// is up, waited for up to three minutes (`http_route::down`). When
/// it does not come up, that is said before anything is fetched, as
/// NoNetwork: an outage, which a retry once it runs gets past, not a mirror
/// that refused.
pub fn reachable(at: &str) -> Result<(), Why> {
    match offline(is_local(at), down) {
        None => Ok(()),
        Some(why) => {
            let line = format!("[LINUX] mirror {at}: not reachable now: {why}\n");
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            Err(Why::NoNetwork)
        }
    }
}

/// The reply from a mirror on the local network, over a socket of this
/// capsule's own, dialled directly.
fn direct(ip: &str, port: u16, req: &[u8]) -> Option<Vec<u8>> {
    let handle = open_stream_to(ip)?;
    if connect_host(handle, ip, port).is_none() {
        close(handle);
        return None;
    }
    let sent = send_all(handle, req);
    let raw = sent.and_then(|()| recv_until(handle, MAX_BODY, &complete, IDLE_MS));
    close(handle);
    raw
}
