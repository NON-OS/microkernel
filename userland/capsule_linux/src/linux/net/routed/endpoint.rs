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

//! The SOCKS endpoint at 127.0.0.1:9050, for a family that holds NET_ANON
//! and has no socket of its own listening there. A connect to it completes
//! in the caller's own call, as one on loopback does; what the guest then
//! writes is its request, heard until it is whole (hear.rs).

use crate::linux::abi::errno::{self, EINPROGRESS};
use crate::linux::app::granted;
use crate::linux::guest::Guest;
use nonos_linux_policy::mask::NET_ANON;

use super::super::sock::{self, Addr, Backend};
use super::entry::{Entry, Phase};
use super::socks::Talk;
use super::table;

/// Where the endpoint listens: Tor's own SOCKS port, which programs that
/// speak SOCKS look for first.
pub const AT: Addr = Addr { ip: [127, 0, 0, 1], port: 9050 };

/// Whether a stream connect from socket `id` to `to` reaches the endpoint.
pub fn serves(guest: &Guest, id: u32, to: Addr) -> bool {
    to == AT
        && granted(guest, NET_ANON)
        && sock::with(|t| {
            let from = t.get(id).and_then(|s| s.local).map_or(0, |a| a.port);
            t.listener(to, from).is_none()
        })
}

/// Socket `id`, connected to the endpoint.
pub fn open(id: u32, nonblock: bool) -> u64 {
    let h = table::insert(Entry::new(Phase::Asking(Talk::default()), 0));
    let joined = sock::with(|t| {
        let s = t.get(id).ok_or(errno::EBADF)?;
        if s.connected || s.connecting || s.listening || s.svc.is_some() {
            return Err(errno::EISCONN);
        }
        t.autobind(id)?;
        let s = t.get_mut(id).ok_or(errno::EBADF)?;
        (s.svc, s.remote, s.connected) = (Some(Backend::Routed(h)), Some(AT), true);
        s.opened_late = nonblock;
        Ok(())
    });
    match joined {
        Err(e) => {
            drop(table::remove(h));
            errno::fail(e)
        }
        Ok(()) if nonblock => errno::fail(EINPROGRESS),
        Ok(()) => errno::ok(0),
    }
}
