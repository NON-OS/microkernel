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

use alloc::string::String;
use nonos_linux_policy::dests::Target;
use nonos_linux_policy::mask::NET_ANON;

use crate::linux::abi::errno;
use crate::linux::app::granted;
use crate::linux::guest::Guest;

use super::super::guest_route::{path, Path};
use super::super::sock::{self, Addr};
use super::super::Route;

pub fn connect(guest: &Guest, id: u32, to: Addr) -> u64 {
    let host = super::super::dns::host_for(guest, to.ip);
    let target = host.as_deref().map_or(Target::V4(to.ip), Target::Name);
    let ([a, b, c, d], port) = (to.ip, to.port);
    let dest = match host.as_deref() {
        Some(name) => alloc::format!("{}:{port}", String::from_utf8_lossy(name)),
        None => alloc::format!("{a}.{b}.{c}.{d}:{port}"),
    };
    if !crate::linux::app::may_dial(guest, target, to.port) {
        let needs = if granted(guest, NET_ANON) { "this destination" } else { "NET_ANON" };
        crate::linux::boxed::tried(dest.as_bytes());
        let what = alloc::format!("a stream to {dest}");
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
    let answer = match path(Route::chosen()) {
        Path::Mixnet => super::mixnet::mixnet(guest, id, to),
        Path::Anyone(port) => super::anon::connect(guest, id, to, port),
        Path::Unreachable(why) => return super::super::policy::unreachable(to, why),
    };
    /* A boxed run's report names each destination the network took a stream to. */
    if answer == errno::ok(0) || answer == errno::fail(errno::EINPROGRESS) {
        crate::linux::boxed::connected(dest.as_bytes());
    }
    answer
}
