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

use nonos_libc::mk_service_lookup;

use crate::clients::tcp;
use crate::protocol::{E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_LISTEN};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::sockets::{SocketKey, SOCKETS};
use crate::state;

use super::listen_route::listen_route;
use super::listen_who::listener_allowed;
use super::passive::listen_port;
use super::route_chosen::direct_chosen;

pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(h) => h,
        Err(e) => return status(pid, req, e, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status(pid, req, E_NO_HANDLE, tx);
    };
    let port = match listen_port(&sock) {
        Ok(port) => port,
        Err(errno) => return status(pid, req, errno, tx),
    };
    if !listener_allowed(pid, nsh_front_pid()) {
        return status(pid, req, E_NO_TRANSPORT, tx);
    }
    if let Err(errno) = listen_route(direct_chosen) {
        return status(pid, req, errno, tx);
    }
    let transport = match tcp::listen(state::tcp(), port) {
        Ok(h) => h,
        Err(_) => return status(pid, req, E_NO_TRANSPORT, tx),
    };
    let updated = SOCKETS.with(key, |s| {
        s.transport_handle = transport;
        s.listening = true;
    });
    if updated.is_none() {
        let _ = tcp::close(state::tcp(), transport);
    }
    status(pid, req, updated.map_or(E_NO_HANDLE, |_| E_OK), tx);
}

fn nsh_front_pid() -> Option<u32> {
    let name = b"nsh.front";
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 {
        return None;
    }
    Some(pid)
}

fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_LISTEN, errno, req.request_id, 0, tx);
}
