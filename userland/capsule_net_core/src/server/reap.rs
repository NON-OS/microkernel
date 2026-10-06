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
//! closing them (`handles/table.rs`, `udp_ports/table.rs`): looked for at
//! most every REAP_GAP_MS, and at once when a connect or bind finds no room.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_pid_alive, mk_uptime_ms};

use crate::server::handlers::tcp::close::release;
use crate::{handles, state, udp_ports};

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Close every connection and unbind every port whose client has ended, as
/// that client's own close and unbind would have.
pub fn reap_now() {
    let conns = handles::take_ended(alive);
    let ports = udp_ports::take_ended(alive);
    if conns.is_empty() && ports.is_empty() {
        return;
    }
    state::with_iface(|_iface, sockets, _dev| {
        for sock_handle in conns {
            release(sockets, sock_handle);
        }
        for sock_handle in ports {
            sockets.remove(sock_handle);
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
