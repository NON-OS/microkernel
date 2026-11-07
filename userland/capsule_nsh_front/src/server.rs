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

//! The listener and the accept loop. nsh.front opens one stream socket on the
//! nsh/1 port, binds it to every address and listens, then serves one
//! connection to completion before taking the next, as a host that holds a
//! single session record at a time; each connection that comes meanwhile is
//! told BUSY by the session (newcomers.rs). A lost listener, whether the
//! accept loop or a session found it gone (net.sockets or net.core
//! restarted, or the route left Direct and net.core closed every listener)
//! is waited out and reopened rather than spun on: the reopen is refused
//! every REOPEN_WAIT_MS until Direct is chosen again.

use crate::marks::Marks;
use crate::session::serve;
use crate::setup;
use crate::sock::{Accept, Net};
use nonos_libc::mk_idle_ms;

/// The TCP port nsh/1 is served on; the client's default, as nshd uses.
pub const LISTEN_PORT: u16 = 2222;
const ACCEPT_IDLE_MS: u64 = 10;
const REOPEN_WAIT_MS: u64 = 500;

/// Serve nsh/1 forever.
pub fn run() -> ! {
    let mut peers = setup::wait();
    let mut listener = open(&mut peers.net);
    let mut marks = Marks::new();
    loop {
        let attest = peers.attest_port;
        let listening = match peers.net.accept(listener) {
            Accept::Child(child) => serve(&mut peers.net, child, attest, listener, &mut marks),
            Accept::Pending => {
                let _ = mk_idle_ms(ACCEPT_IDLE_MS);
                true
            }
            Accept::Gone => false,
        };
        if !listening {
            // Let the old handle go first, so a net.sockets that kept it
            // does not hold the port while the new listener is refused.
            peers.net.close(listener);
            peers = setup::wait();
            listener = open(&mut peers.net);
        }
    }
}

/// Open and listen, retrying until net.sockets grants the listener.
fn open(net: &mut Net) -> u32 {
    loop {
        if let Some(handle) = try_open(net) {
            return handle;
        }
        let _ = mk_idle_ms(REOPEN_WAIT_MS);
    }
}

fn try_open(net: &mut Net) -> Option<u32> {
    let handle = net.socket_stream()?;
    if net.bind(handle, [0, 0, 0, 0], LISTEN_PORT) && net.listen(handle) {
        return Some(handle);
    }
    net.close(handle);
    None
}
