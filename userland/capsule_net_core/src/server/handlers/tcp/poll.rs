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

use smoltcp::socket::tcp;

use crate::handles;
use crate::listeners;
use crate::protocol::tcp::{E_BAD_LEN, E_NO_SOCKET, E_OK, MAGIC_NTCP, OP_POLL, POLL_READABLE};
use crate::server::handlers::tcp::poll_bits::poll_bits;
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

// Report the readiness of one connection without touching its buffers: bit0 is
// set when a receive would return data, bit1 when a send would accept data,
// bit2 when the peer has finished and nothing is left to read (poll_bits.rs).
// A reactor calls this across every socket it waits on to decide which tasks to
// wake, so the poll must never consume; `can_recv`/`can_send` only inspect.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 4 {
        let _ = reply(sender_pid, MAGIC_NTCP, OP_POLL, E_BAD_LEN, req.request_id, &[], tx);
        return;
    }
    let app_handle = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);

    // A listener is readable while a connection waits for its ACCEPT.
    if listeners::is_listener(app_handle) {
        let ready = state::with_iface(|_iface, sockets, _dev| {
            listeners::readable(sockets, app_handle, sender_pid)
        });
        let _ = match ready.flatten() {
            Some(r) => {
                let bits = if r { POLL_READABLE } else { 0 };
                reply(sender_pid, MAGIC_NTCP, OP_POLL, E_OK, req.request_id, &[bits], tx)
            }
            None => reply(sender_pid, MAGIC_NTCP, OP_POLL, E_NO_SOCKET, req.request_id, &[], tx),
        };
        return;
    }

    let sock_handle = match handles::get(app_handle, sender_pid) {
        Some(h) => h,
        None => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_POLL, E_NO_SOCKET, req.request_id, &[], tx);
            return;
        }
    };

    let bits = state::with_iface(|_iface, sockets, _dev| {
        let s = sockets.get::<tcp::Socket>(sock_handle);
        poll_bits(s.state(), s.can_recv(), s.can_send())
    })
    .unwrap_or(0);

    let _ = reply(sender_pid, MAGIC_NTCP, OP_POLL, E_OK, req.request_id, &[bits], tx);
}
