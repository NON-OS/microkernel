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

//! A stream to an address outside the family. It goes over the mixnet,
//! never the open network, and the guest holds no capability that could
//! name a socket: there is no second route to disable and no firewall rule
//! to remove. net.sockets holds the stream; the family's entry names it.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::call::call;
use super::dns::host_for;
use super::ops::{DOMAIN, KIND_MIXNET, OP_CONNECT, OP_CONNECT_HOST, OP_SOCKET};
use super::sock::{self, Addr};

pub fn connect(guest: &Guest, id: u32, to: Addr) -> u64 {
    if sock::with(|t| t.get(id).is_some_and(|s| s.connected || s.listening || s.svc.is_some())) {
        return errno::fail(errno::EISCONN);
    }
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&KIND_MIXNET.to_le_bytes());
    let handle = match call(OP_SOCKET, &body, 8) {
        Some((0, out)) if out.len() >= 4 => u32::from_le_bytes([out[0], out[1], out[2], out[3]]),
        Some(_) => return errno::fail(errno::ENOMEM),
        None => return errno::fail(errno::EIO),
    };
    // An address this capsule invented for a name goes back to being the name.
    let status = match host_for(guest, to.ip) {
        Some(host) => match super::host_body::host_body(handle, to.port, &host) {
            Some(body) => call(OP_CONNECT_HOST, &body, 0),
            None => {
                super::stream::close(handle);
                return errno::fail(errno::EINVAL);
            }
        },
        None => {
            let mut body = Vec::with_capacity(10);
            body.extend_from_slice(&handle.to_le_bytes());
            body.extend_from_slice(&to.ip);
            body.extend_from_slice(&to.port.to_le_bytes());
            call(OP_CONNECT, &body, 0)
        }
    };
    let answer = match status {
        Some((0, _)) => errno::ok(0),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    };
    if answer != 0 {
        super::stream::close(handle);
        return answer;
    }
    sock::with(|t| {
        if let Some(s) = t.get_mut(id) {
            s.svc = Some(handle);
            s.remote = Some(to);
            s.connected = true;
        }
    });
    answer
}
