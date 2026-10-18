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

//! A mirror on the local network, dialled directly by address over a socket
//! of this capsule's own (net/raw.rs).

use alloc::vec::Vec;

use super::super::http_reply::complete;
use super::get::MAX_BODY;
use crate::linux::net::raw::{connect_host, open_stream_to};
use crate::linux::net::raw_io::{close, recv_until, send_all};

/// How long a mirror may go silent: one fetching upstream before it answers
/// sends nothing for a while, and a slow link for longer under emulation.
const IDLE_MS: u64 = 120_000;

/// The reply from a mirror on the local network, over a socket of this
/// capsule's own, dialled directly.
pub(super) fn direct(ip: &str, port: u16, req: &[u8]) -> Option<Vec<u8>> {
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
