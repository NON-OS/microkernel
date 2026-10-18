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

//! Where the gate's state is kept: in the driver, three atomics read and
//! written around each call; in the host proofs, nowhere.

#[cfg(not(test))]
use super::Gate;
#[cfg(not(test))]
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[cfg(not(test))]
static UNANSWERED: AtomicU32 = AtomicU32::new(0);
#[cfg(not(test))]
static PAUSES: AtomicU32 = AtomicU32::new(0);
#[cfg(not(test))]
static CLOSED_UNTIL_MS: AtomicU64 = AtomicU64::new(0);

#[cfg(not(test))]
fn now_ms() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}

#[cfg(not(test))]
fn load() -> Gate {
    Gate {
        unanswered: UNANSWERED.load(Ordering::Relaxed),
        pauses: PAUSES.load(Ordering::Relaxed),
        closed_until_ms: CLOSED_UNTIL_MS.load(Ordering::Relaxed),
    }
}

/// Whether a call may go out now.
#[cfg(not(test))]
pub fn may_call() -> bool {
    load().open(now_ms())
}

/// Count the result of a call that went out.
#[cfg(not(test))]
pub fn record(got: i64) {
    let mut g = load();
    g.record(got, now_ms());
    UNANSWERED.store(g.unanswered, Ordering::Relaxed);
    PAUSES.store(g.pauses, Ordering::Relaxed);
    CLOSED_UNTIL_MS.store(g.closed_until_ms, Ordering::Relaxed);
}

// The host proofs run many tests at once against one fake server; a gate
// shared between them would close on one test's timeouts in another's
// calls, so there it stays open and `Gate` is checked on its own.
#[cfg(test)]
pub fn may_call() -> bool {
    true
}

#[cfg(test)]
pub fn record(_got: i64) {}
