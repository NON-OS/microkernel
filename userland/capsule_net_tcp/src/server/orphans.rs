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

//! Resetting the connections of owners that ended (`state/table/orphans.rs`),
//! looked for at most every REAP_GAP_MS while the stack is busy.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::server::tcp_tx;
use crate::state::TABLE;

const REAP_GAP_MS: u64 = 2_000;

static LAST_REAP: AtomicU64 = AtomicU64::new(0);

fn alive(pid: u32) -> bool {
    nonos_libc::mk_pid_alive(pid)
}

pub fn reap_if_due(now: u64) {
    let last = LAST_REAP.load(Ordering::Relaxed);
    if last != 0 && now >= last && now - last < REAP_GAP_MS {
        return;
    }
    LAST_REAP.store(now, Ordering::Relaxed);
    reap_now();
}

/// Take out every ended owner's connections and reset the open ones, so
/// their peers stop waiting on an answer that will not come.
pub fn reap_now() {
    let open = TABLE.lock().take_orphans(alive);
    for t in open {
        let _ = tcp_tx::send_rst(t.local, t.remote, t.send.nxt, t.recv.nxt);
    }
}
