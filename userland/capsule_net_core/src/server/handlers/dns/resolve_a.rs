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

//! RESOLVE_A: ask every DNS server the lease named at once and keep the
//! caller waiting in `pending`; the serve loop answers it after a poll
//! (`settle`) with the first address any server gives, so a lookup never
//! holds the loop that serves everyone else, and one server down does not
//! fail it. smoltcp moves to a second server only after ten seconds, well
//! past TIMEOUT_MS and the two seconds the browser waits.

use nonos_libc::mk_uptime_ms;
use smoltcp::socket::dns::Socket as DnsSocket;
use smoltcp::wire::DnsQueryType;

use super::answer::{cancel, err};
use super::lookups::{Lookup, PENDING, TIMEOUT_MS};
use super::pending::Waiting;
use crate::protocol::dns::{E_NAME_INVALID, E_NO_LEASE, E_SERVFAIL};
use crate::server::parse_req::Request;
use crate::state::{self, DNS_SERVERS};

pub use super::lookups::waiting;
pub use super::settle::{settle, settle_for};

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let name = match core::str::from_utf8(body) {
        Ok(s) if !s.is_empty() => s,
        _ => return err(sender_pid, req.request_id, E_NAME_INVALID, tx),
    };
    let started = state::with_dns(|iface, sockets, dns| {
        let mut queries = [None; DNS_SERVERS];
        for (query, socket) in queries.iter_mut().zip(dns) {
            if let Some(socket) = socket {
                let s = sockets.get_mut::<DnsSocket>(socket);
                *query = s.start_query(iface.context(), name, DnsQueryType::A).ok();
            }
        }
        Lookup { generation: state::generation(), dns, queries }
    });
    let lookup = match started {
        Some(l) if l.queries.iter().any(Option::is_some) => l,
        Some(_) => return err(sender_pid, req.request_id, E_NAME_INVALID, tx),
        None => return err(sender_pid, req.request_id, E_NO_LEASE, tx),
    };
    let w = Waiting {
        pid: sender_pid,
        request_id: req.request_id,
        query: lookup,
        deadline_ms: mk_uptime_ms() + TIMEOUT_MS,
    };
    if let Err(mut w) = PENDING.lock().add(w) {
        cancel(&mut w.query);
        err(sender_pid, req.request_id, E_SERVFAIL, tx);
    }
}
