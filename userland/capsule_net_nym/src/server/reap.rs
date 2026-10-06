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

//! Closing the sessions of clients that ended without closing them
//! (`state/table/ended.rs`): looked for at most every REAP_GAP_MS, and at
//! once when a session cannot be opened.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_pid_alive, mk_uptime_ms};

use crate::state::TABLE;

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Close every session whose owner has ended.
pub fn reap_now() {
    let _ = TABLE.lock().close_ended(alive);
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
