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

//! A stream to an address outside the family. It goes over the anonymity
//! network the person chose, never the open network, and the guest holds no
//! capability that could name a socket: there is no second route to disable
//! and no firewall rule to remove. With the Anyone network chosen net.anon
//! holds the stream (connect_anon.rs); otherwise net.sockets holds it on the
//! Nym mixnet, as below. The family's entry names it either way.
//!
//! Direct, when it is the system's default, still leaves a guest on the
//! mixnet. That is by design and not an omission: a guest runs a program
//! nobody here wrote, and a direct socket would name this machine to
//! whatever it connects to. The rule is guest_route.rs; the one exception on
//! the installer's own path is design/install-network.md.

use nonos_linux_policy::dests::Target;
use nonos_linux_policy::mask::NET_ANON;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::guest_route::{path, Path};
use super::super::sock::{self, Addr};
use super::super::Route;

pub fn connect(guest: &Guest, id: u32, to: Addr) -> u64 {
    let host = super::super::dns::host_for(guest, to.ip);
    let target = host.as_deref().map_or(Target::V4(to.ip), Target::Name);
    if !crate::linux::app::may_dial(guest, target, to.port) {
        let needs = match crate::linux::app::granted(guest, NET_ANON) {
            true => "this destination",
            false => "NET_ANON",
        };
        let [a, b, c, d] = to.ip;
        let what = match host.as_deref() {
            Some(name) => {
                alloc::format!(
                    "a stream to {}:{}",
                    alloc::string::String::from_utf8_lossy(name),
                    to.port
                )
            }
            None => alloc::format!("a stream to {a}.{b}.{c}.{d}:{}", to.port),
        };
        return crate::linux::app::refuse(&what, needs, errno::ENETUNREACH);
    }
    if sock::with(|t| t.get(id).is_some_and(|s| s.connected || s.listening || s.svc.is_some())) {
        return errno::fail(errno::EISCONN);
    }
    /*
     * The chosen network, read for this connection: Anyone goes to net.anon,
     * or nowhere when net.anon is not running; Nym, an unreadable default and
     * Direct go to the mixnet below. One network or none, never a second.
     */
    match path(Route::chosen()) {
        Path::Mixnet => {}
        Path::Anyone(port) => return super::anon::connect(guest, id, to, port),
        Path::Unreachable(why) => return super::super::policy::unreachable(to, why),
    }
    super::mixnet::mixnet(guest, id, to)
}
