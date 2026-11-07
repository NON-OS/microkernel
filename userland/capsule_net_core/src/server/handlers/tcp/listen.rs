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

//! LISTEN: a passive open on a port, for net.sockets alone and only on the
//! Direct route (listeners/gate.rs). The body is the port (passive_body.rs).

use nonos_libc::mk_service_lookup;
use nonos_policy_proto::Field;

use crate::listeners::{self, gate, Refusal};
use crate::protocol::tcp::{
    E_BAD_ADDR, E_NO_SOCKET, E_OK, E_PORT_IN_USE, E_REFUSED, MAGIC_NTCP, OP_LISTEN,
};
use crate::server::handlers::tcp::connect::new_socket;
use crate::server::handlers::tcp::passive_body::listen_port;
use crate::server::parse_req::Request;
use crate::server::respond::reply;
use crate::state;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let answer = match listen_port(body) {
        Ok(port) if allowed(sender_pid) => open(sender_pid, port),
        Ok(_) => Err(E_REFUSED),
        Err(errno) => Err(errno),
    };
    let _ = match answer {
        Ok(handle) => reply(
            sender_pid,
            MAGIC_NTCP,
            OP_LISTEN,
            E_OK,
            req.request_id,
            &handle.to_le_bytes(),
            tx,
        ),
        Err(errno) => reply(sender_pid, MAGIC_NTCP, OP_LISTEN, errno, req.request_id, &[], tx),
    };
}

pub fn route() -> Option<u8> {
    nonos_policy_client::lookup()
        .and_then(|port| nonos_policy_client::get_u8(port, Field::NetworkRoute))
}

fn sockets_pid() -> Option<u32> {
    let name = b"net.sockets";
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) < 0 {
        return None;
    }
    Some(pid)
}

fn allowed(sender_pid: u32) -> bool {
    gate::listen_allowed(sender_pid, sockets_pid(), route())
}

fn open(sender_pid: u32, port: u16) -> Result<u32, u16> {
    let opened = state::with_iface(|_iface, sockets, _dev| {
        let mut sock = new_socket();
        if sock.listen(port).is_err() {
            return Err(E_BAD_ADDR);
        }
        let socket = sockets.add(sock);
        listeners::open(sender_pid, port, socket).map_err(|refusal| {
            sockets.remove(socket);
            match refusal {
                Refusal::PortInUse => E_PORT_IN_USE,
                Refusal::Full => E_NO_SOCKET,
            }
        })
    });
    opened.unwrap_or(Err(E_NO_SOCKET))
}
