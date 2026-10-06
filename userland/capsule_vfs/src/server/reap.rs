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

//! Closing the handles of clients that ended without closing them, and
//! removing the private files of Linux runs that ended without removing
//! them (`store/fdtable/reap.rs`): looked for at most every REAP_GAP_MS while
//! requests arrive, and at once when an open finds no room.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_pid_alive, mk_uptime_ms};

use crate::store::Store;

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Close every handle whose owner has ended.
pub(super) fn reap_now(store: &mut Store) {
    let _ = store.close_ended(alive);
    let _ = store.drop_private_of_ended(alive);
}

pub(super) fn reap_if_due(store: &mut Store) {
    let now = mk_uptime_ms();
    let last = LAST_REAP.load(Ordering::Relaxed);
    if last != i64::MIN && now.wrapping_sub(last) < REAP_GAP_MS {
        return;
    }
    LAST_REAP.store(now, Ordering::Relaxed);
    reap_now(store);
}
