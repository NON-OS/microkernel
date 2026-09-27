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

//! A GET, over the socket service this capsule already uses.

use alloc::format;
use alloc::vec::Vec;

use super::http_reply::{body, complete};
use super::mirror::HOST_LINE;
use crate::linux::net::raw::{connect_host, open_stream_to};
use crate::linux::net::raw_io::{close, recv_until, send_all};

/// Enough for the largest package index; a reply beyond it is refused
/// rather than truncated into a half-parsed index.
const MAX_BODY: usize = 64 << 20;
/// How long a mirror may go silent: one fetching upstream before it answers
/// sends nothing for a while, and a slow link for longer under emulation.
const IDLE_MS: u64 = 120_000;

/// A GET to Alpine's mirror.
pub fn get(ip: &str, port: u16, path: &str) -> Option<Vec<u8>> {
    get_as(ip, port, HOST_LINE, path)
}

/// A GET to `ip`, which must be a dotted IPv4 address: a name here would be
/// resolved by the socket service, in the clear. `host` is only the Host line.
pub fn get_as(ip: &str, port: u16, host: &str, path: &str) -> Option<Vec<u8>> {
    if ip.split('.').filter(|o| o.parse::<u8>().is_ok()).count() != 4 {
        return None;
    }
    let handle = open_stream_to(ip)?;
    if connect_host(handle, ip, port).is_none() {
        close(handle);
        return None;
    }
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos\r\nConnection: close\r\n\r\n"
    );
    let sent = send_all(handle, req.as_bytes());
    let raw = sent.and_then(|()| recv_until(handle, MAX_BODY, &complete, IDLE_MS));
    close(handle);
    let got = raw.and_then(body);
    let line = match &got {
        Some(b) => format!("[LINUX] mirror {ip}: {path}, {} bytes\n", b.len()),
        None => format!("[LINUX] mirror {ip}: GET {path} gave no whole 200 reply\n"),
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    got
}
