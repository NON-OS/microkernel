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

//! ACCEPT: the oldest connection a listener took whose handshake is done, as
//! a connection handle its owner reads and writes like any other. Nothing
//! ready answers E_RX_EMPTY at once; the serve loop never waits for a peer.
//! With a connection ready and the Direct route no longer chosen, every
//! listener goes and the caller is answered E_REFUSED (listeners/gate.rs).

use crate::handles;
use crate::listeners::{self, gate};
use crate::protocol::tcp::{
    E_NO_ROOM, E_NO_SOCKET, E_OK, E_REFUSED, E_RX_EMPTY, MAGIC_NTCP, OP_ACCEPT,
};
use crate::server::handlers::tcp::listen::route;
use crate::server::handlers::tcp::passive_body::accept_handle;
use crate::server::parse_req::Request;
use crate::server::reap::drop_listeners;
use crate::server::respond::reply;
use crate::state;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let answer = accept_handle(body).and_then(|listener| accept(sender_pid, listener));
    let _ = match answer {
        Ok(child) => {
            reply(sender_pid, MAGIC_NTCP, OP_ACCEPT, E_OK, req.request_id, &child.to_le_bytes(), tx)
        }
        Err(errno) => reply(sender_pid, MAGIC_NTCP, OP_ACCEPT, errno, req.request_id, &[], tx),
    };
}

fn accept(sender_pid: u32, listener: u32) -> Result<u32, u16> {
    let ready = state::with_iface(|_iface, sockets, _dev| {
        listeners::readable(sockets, listener, sender_pid)
    });
    match ready.flatten() {
        None => return Err(E_NO_SOCKET),
        Some(false) => return Err(E_RX_EMPTY),
        Some(true) => {}
    }
    if !gate::accept_allowed(route()) {
        drop_listeners();
        return Err(E_REFUSED);
    }
    let taken = state::with_iface(|_iface, sockets, _dev| {
        listeners::accept(sockets, listener, sender_pid, |h| {
            handles::alloc_passive(sender_pid, h).ok_or(())
        })
    });
    match taken.flatten() {
        None => Err(E_NO_SOCKET),
        Some(None) => Err(E_RX_EMPTY),
        Some(Some(Ok(child))) => Ok(child),
        // Every connection handle the caller may hold is taken; the
        // connection stays queued for an ACCEPT after a CLOSE. Not
        // E_NO_SOCKET, which says the listener is gone.
        Some(Some(Err(()))) => Err(E_NO_ROOM),
    }
}
