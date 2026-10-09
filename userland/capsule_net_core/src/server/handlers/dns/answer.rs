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

//! The replies a lookup ends with, the reading of its queries and the freeing
//! of those not read.

use smoltcp::iface::SocketSet;
use smoltcp::socket::dns::{GetQueryResultError, Socket as DnsSocket};
use smoltcp::wire::IpAddress;

use super::answered_by::say_answered;
use super::lookups::Lookup;
use super::verdict::{combine, Found};
use crate::protocol::dns::{MAGIC_NDNS, OP_RESOLVE_A};
use crate::server::respond::reply;
use crate::state;

// Read each server's query not yet read, and drop the ones read.
pub(super) fn read(sockets: &mut SocketSet<'static>, l: &mut Lookup) -> Found {
    let each = l.dns.iter().zip(l.queries.iter_mut()).enumerate();
    combine(each.map(|(server, (socket, query))| {
        let (Some(socket), Some(q)) = (socket, *query) else { return Found::Failed };
        let found = match sockets.get_mut::<DnsSocket>(*socket).get_query_result(q) {
            Err(GetQueryResultError::Pending) => return Found::Pending,
            Ok(addrs) => first_v4(&addrs).map_or(Found::Failed, Found::Address),
            Err(_) => Found::Failed,
        };
        *query = None;
        if let Found::Address(_) = found {
            say_answered(server);
        }
        found
    }))
}

// Free the slots of the queries not read, if their sockets are still the ones
// in use.
pub(super) fn cancel(l: &mut Lookup) {
    let (generation, dns) = (l.generation, l.dns);
    let _ = state::with_dns_at(generation, dns, |sockets| {
        for (socket, query) in dns.iter().zip(l.queries.iter_mut()) {
            if let (Some(socket), Some(q)) = (socket, query.take()) {
                sockets.get_mut::<DnsSocket>(*socket).cancel_query(q);
            }
        }
    });
}

pub(super) fn first_v4(addrs: &[IpAddress]) -> Option<[u8; 4]> {
    addrs.iter().find_map(|a| match a {
        IpAddress::Ipv4(v4) if v4.0 != [0, 0, 0, 0] => Some(v4.0),
        _ => None,
    })
}

pub(super) fn err(sender_pid: u32, request_id: u32, errno: u16, tx: &mut [u8]) {
    let _ = reply(sender_pid, MAGIC_NDNS, OP_RESOLVE_A, errno, request_id, &[], tx);
}
