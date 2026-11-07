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

use smoltcp::socket::icmp;
use smoltcp::wire::{IpAddress, Ipv4Address};

use super::last_ident::remember_ident;
use super::send_body::read;
use super::socket::with_icmp;
use crate::protocol::ip::*;
use crate::server::handlers::tcp::listen::route;
use crate::server::parse_req::Request;
use crate::server::respond::reply;

/// Send one ICMP message to `dst`.
///
/// The caller builds the ICMP header, so the identifier it chose is what the
/// socket must be bound to; it is read back out of the message rather than
/// assumed, which keeps the service usable by anything that speaks ICMP.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let send = match read(body) {
        Ok(send) => send,
        Err(code) => return fail(sender_pid, req, code, tx),
    };
    let (dst, ident, payload) = (Ipv4Address(send.dst), send.ident, send.message);

    if !super::route_gate::clear_send_allowed(route()) {
        return fail(sender_pid, req, E_NOT_DIRECT, tx);
    }

    let code = with_icmp(ident, |sock| {
        if !sock.can_send() {
            return E_NO_NEIGHBOUR;
        }
        match sock.send_slice(payload, IpAddress::Ipv4(dst)) {
            Ok(()) => E_OK,
            // A full buffer is the queue backing up, not a dead peer, so the
            // caller is told to retry rather than that the host is gone.
            Err(icmp::SendError::BufferFull) => E_NO_NEIGHBOUR,
            Err(icmp::SendError::Unaddressable) => E_NO_ROUTE,
        }
    });

    let code = code.unwrap_or(E_NO_CONFIG);
    if code == E_OK {
        remember_ident(ident);
    }
    let _ = reply(sender_pid, MAGIC_NIP4, req.op, code, req.request_id, &[], tx);
}

fn fail(sender_pid: u32, req: &Request, code: u16, tx: &mut [u8]) {
    let _ = reply(sender_pid, MAGIC_NIP4, req.op, code, req.request_id, &[], tx);
}
