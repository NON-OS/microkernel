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

//! Starting a stream connect without waiting for it.

use nonos_libc::mk_time_millis;

use crate::clients::tcp;
use crate::protocol::E_NO_TRANSPORT;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::SocketKey;
use crate::state;

use super::table::{Pending, PENDING};

/// How long a handshake may take before the caller is told it failed. It
/// stays under the nine seconds the browser waits for the reply.
const DEADLINE_MS: i64 = 8000;

/// Send the SYN and record the caller. The reply is sent by `advance`, or
/// here at once when the connect cannot even start.
pub fn start(
    pid: u32,
    op: u16,
    req: &Request,
    key: SocketKey,
    ip: [u8; 4],
    port: u16,
    tx: &mut [u8],
) {
    let Ok(transport) = tcp::connect(state::tcp(), ip, port) else {
        respond(pid, op, E_NO_TRANSPORT, req.request_id, 0, tx);
        return;
    };
    let deadline_ms = mk_time_millis().wrapping_add(DEADLINE_MS);
    let entry =
        Pending { pid, op, request_id: req.request_id, key, transport, ip, port, deadline_ms };
    PENDING.lock().push(entry);
}
