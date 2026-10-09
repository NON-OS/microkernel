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

//! Sockets for this capsule's own use, rather than a guest's.

use alloc::vec::Vec;

use super::call::call;
use super::ops::{DOMAIN, KIND_MIXNET, KIND_STREAM, OP_CONNECT_HOST, OP_SOCKET};

/// A stream to `ip`: over the mixnet, like everything else, unless `ip` is a
/// mirror on the local network (`route::is_local`), which is dialled directly
/// and said so.
pub fn open_stream_to(ip: &str) -> Option<u32> {
    let kind = match super::route::is_local(ip) {
        true => {
            let line =
                alloc::format!("[LINUX] mirror {ip} is on the local network: reached directly\n");
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            KIND_STREAM
        }
        false => KIND_MIXNET,
    };
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&kind.to_le_bytes());
    match call(OP_SOCKET, &body, 8) {
        Some((0, out)) if out.len() >= 4 => {
            Some(u32::from_le_bytes([out[0], out[1], out[2], out[3]]))
        }
        got => failed("socket", ip, got.map(|g| g.0)),
    }
}

pub fn connect_host(handle: u32, host: &str, port: u16) -> Option<()> {
    let body = super::host_body::host_body(handle, port, host.as_bytes())?;
    match call(OP_CONNECT_HOST, &body, 0) {
        Some((0, _)) => Some(()),
        got => failed("connect", host, got.map(|g| g.0)),
    }
}

/// Which step a mirror fetch stopped at, and what net.sockets said: an
/// install that fails with only "no package index" cannot be told apart
/// from a mirror that is down.
fn failed<T>(step: &str, to: &str, status: Option<u16>) -> Option<T> {
    let line = match status {
        Some(code) => alloc::format!("[LINUX] mirror {to}: {step} refused, status {code}\n"),
        None => alloc::format!("[LINUX] mirror {to}: {step} got no reply\n"),
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    None
}
