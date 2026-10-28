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
//! and no firewall rule to remove. routed/ opens and carries it: through
//! net.anon on the Anyone network, through net.socks5 on the Nym mixnet.
//!
//! An address this capsule invented for a name goes as the name, and any
//! other as its dotted quad: the far end's exit resolves a name, so nothing
//! on this machine looks one up. Direct, when it is the system's default,
//! still leaves a guest on the mixnet; that is by design (guest_route.rs).

use alloc::string::String;
use nonos_linux_policy::dests::Target;
use nonos_linux_policy::mask::NET_ANON;

use crate::linux::abi::errno;
use crate::linux::app::granted;
use crate::linux::guest::Guest;

use super::super::anon_answer::dotted;
use super::super::routed::{self, Dest};
use super::super::sock::{self, Addr};

pub fn connect(guest: &Guest, id: u32, to: Addr, nonblock: bool) -> u64 {
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
    let host = host.unwrap_or_else(|| dotted(to.ip));
    let answer = routed::dial(id, Dest { host, port }, to, nonblock);
    /*
     * A boxed run's report names each destination the network took a
     * stream to, or is taking one to: a blocking connect still opening is
     * parked on EAGAIN and finishes in a later try.
     */
    let opening = [errno::ok(0), errno::fail(errno::EINPROGRESS), errno::fail(errno::EAGAIN)];
    if opening.contains(&answer) {
        crate::linux::boxed::connected(dest.as_bytes());
    }
    answer
}
