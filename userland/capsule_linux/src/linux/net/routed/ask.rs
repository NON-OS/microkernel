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

//! A whole CONNECT at the SOCKS endpoint, held to what a plain connect is:
//! the program's policy for the destination, then the chosen network, by
//! name. A name stays a name; an address this capsule invented for a name
//! goes back to being the name; any other address goes as its dotted quad.
//! No line says the destination, only how long it is.

use alloc::format;
use nonos_libc::mk_uptime_ms;
use nonos_linux_policy::dests::Target;

use super::super::anon_answer::dotted;
use super::super::dns::host_for;
use super::entry::{Dest, Phase};
use super::socks::reply::{reply, NOT_ALLOWED};
use super::socks::{Connect, To, Ver};
use super::{drive::drive, retry, table};
use crate::linux::abi::errno::EACCES;
use crate::linux::guest::Guest;

pub fn start(guest: &Guest, id: u32, h: u32, ver: Ver, c: Connect) {
    let host = match &c.to {
        To::Name(name) => name.clone(),
        To::V4(ip) => host_for(guest, *ip).unwrap_or_else(|| dotted(*ip)),
    };
    let target = match &c.to {
        To::V4(ip) if host_for(guest, *ip).is_none() => Target::V4(*ip),
        _ => Target::Name(&host),
    };
    /* This machine's own addresses are the family's, never an exit's to reach. */
    let own = matches!(c.to, To::V4([127, ..]));
    if own || !crate::linux::app::may_dial(guest, target, c.port) {
        let what =
            format!("a stream through the SOCKS endpoint, its destination {} bytes", host.len());
        let _ = crate::linux::app::refuse(&what, "this destination", EACCES);
        table::with(h, |e| {
            e.out.extend_from_slice(&reply(ver, NOT_ALLOWED));
            (e.socks, e.phase) = (Some(ver), Phase::Closed);
        });
        return;
    }
    let now = mk_uptime_ms();
    table::with(h, |e| {
        (e.socks, e.until) = (Some(ver), retry::until(now));
        e.dest = Dest { host, port: c.port };
        e.phase = Phase::Route { again: now };
    });
    drive(id, false);
}
