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

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::connect_dial::{dial, open};
use super::guest_route::{path, Path};
use super::ops::{NET_E_NAME_REFUSED, NET_E_NO_TRANSPORT};
use super::sock::{self, Addr, Backend};
use super::Route;

pub fn connect(guest: &Guest, id: u32, to: Addr) -> u64 {
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
        Path::Anyone(port) => return super::connect_anon::connect(guest, id, to, port),
        Path::Unreachable(why) => return super::policy::unreachable(to, why),
    }
    let handle = match open() {
        Ok(h) => h,
        Err(e) => return e,
    };
    let status = match dial(guest, handle, to) {
        Ok(s) => s,
        Err(e) => {
            super::stream::close(handle);
            return e;
        }
    };
    /*
     * No transport is a mixnet holding no gateway: there is no route out, which
     * a tool has to read as unreachable, not as a peer that answered and
     * refused.
     */
    /*
     * A name the mixnet cannot carry is no route out either: net.sockets
     * refused to resolve it in the clear, so nothing left the machine.
     */
    let answer = match status {
        Some((0, _)) => errno::ok(0),
        Some((NET_E_NO_TRANSPORT | NET_E_NAME_REFUSED, _)) => errno::fail(errno::ENETUNREACH),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    };
    if answer != 0 {
        super::stream::close(handle);
        return answer;
    }
    sock::with(|t| {
        if let Some(s) = t.get_mut(id) {
            s.svc = Some(Backend::Sockets(handle));
            s.remote = Some(to);
            s.connected = true;
        }
    });
    answer
}
