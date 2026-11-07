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

//! Waiting for the two peers nsh.front cannot serve without: net.sockets, for
//! the listener, and nsh.attest, for the per-handshake document. A timed
//! receive parks the capsule off the run queue between tries rather than
//! spinning a core for the life of the boot; no client is served until both
//! are resolved, so nothing is lost by waiting.

use crate::sock::Net;
use nonos_libc::{mk_ipc_recv, mk_service_lookup};

const OWN_INBOX: u64 = 0;
const RETRY_MS: u64 = 250;

/// The resolved peers: the socket service and nsh.attest's endpoint.
pub struct Peers {
    pub net: Net,
    pub attest_port: u32,
}

/// Block until net.sockets and nsh.attest are both up, then return them.
pub fn wait() -> Peers {
    let mut idle = [0u8; 1];
    loop {
        if let (Some(net), Some(attest_port)) = (Net::resolve(), attest_port()) {
            return Peers { net, attest_port };
        }
        let _ = mk_ipc_recv(OWN_INBOX, idle.as_mut_ptr(), idle.len(), RETRY_MS);
    }
}

fn attest_port() -> Option<u32> {
    let name = b"nsh.attest";
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 || port == 0 {
        return None;
    }
    Some(port)
}
