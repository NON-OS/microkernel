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

//! Letting go of the connections and ports of clients that ended without
//! closing them (`handles/table.rs`, `udp_ports/table.rs`), and of every
//! listener and every connection a listener took once the route is not
//! Direct: looked for at most every REAP_GAP_MS, and at once when a connect
//! or bind finds no room.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_pid_alive, mk_uptime_ms};
use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp;

use crate::listeners::{self, gate};
use crate::server::handlers::tcp::close::release;
use crate::server::handlers::tcp::listen::route;
use crate::server::handlers::tcp::reset::reset;
use crate::{handles, state, udp_ports};

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Close every connection and unbind every port whose client has ended, as
/// that client's own close and unbind would have.
pub fn reap_now() {
    let mut conns = handles::take_ended(alive);
    conns.extend(listeners::take_ended(alive));
    let mut cut = Vec::new();
    // A listener, and every connection one took, lives only while the
    // Direct route is chosen.
    let passive = handles::passive();
    if (listeners::any() || !passive.is_empty()) && !gate::passive_may_stay(route()) {
        conns.extend(listeners::take_all());
        cut = passive;
    }
    let ports = udp_ports::take_ended(alive);
    if conns.is_empty() && ports.is_empty() && cut.is_empty() {
        return;
    }
    state::with_iface(|_iface, sockets, _dev| {
        for sock_handle in conns {
            release(sockets, sock_handle);
        }
        for sock_handle in ports {
            sockets.remove(sock_handle);
        }
        for sock_handle in cut {
            cut_off(sockets, sock_handle);
        }
    });
}

/// Reset a connection a listener took, off the Direct route. Its handle
/// stays with its owner, whose reads now find it finished (POLL_FINISHED)
/// and whose CLOSE frees the slot, so the handle never names another
/// connection in between. What its peer sent and nobody read is dropped.
fn cut_off(sockets: &mut SocketSet<'static>, sock_handle: SocketHandle) {
    reset(sockets.get_mut::<tcp::Socket>(sock_handle));
}

/// Close every listener and its queue at once, for a route no longer Direct.
pub fn drop_listeners() {
    let conns = listeners::take_all();
    if conns.is_empty() {
        return;
    }
    state::with_iface(|_iface, sockets, _dev| {
        for sock_handle in conns {
            release(sockets, sock_handle);
        }
    });
}

pub fn reap_if_due() {
    let now = mk_uptime_ms();
    let last = LAST_REAP.load(Ordering::Relaxed);
    if last != i64::MIN && now.wrapping_sub(last) < REAP_GAP_MS {
        return;
    }
    LAST_REAP.store(now, Ordering::Relaxed);
    reap_now();
}
