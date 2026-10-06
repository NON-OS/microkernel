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

//! What counts as a lost wake. Pure, so the host proofs can hold it.
//!
//! Every wait here has a partner on another thread that answers at once, so
//! a wait that lasts a second was not answered: its wake went missing and the
//! waiter ran only when its own timeout came. A sleep is late when it ends
//! well past its deadline, which is the tick sweep failing to wake it.

/// A futex hand-off or an IPC round trip this long had its wake lost.
pub const STALL_MS: u64 = 1000;
/// How long one wait may block before its timeout returns it.
pub const WAIT_TIMEOUT_MS: u64 = 2000;
/// A timed sleep that ends this far past its deadline was woken late. Two
/// ticks at 100 Hz and a scheduling round on a busy machine are well inside.
pub const LATE_MS: u64 = 50;

pub fn is_stall(waited_ms: u64) -> bool {
    waited_ms >= STALL_MS
}

pub fn is_late(asked_ms: u64, slept_ms: u64) -> bool {
    slept_ms.saturating_sub(asked_ms) >= LATE_MS
}

/// The run passed: work moved on every kind, and nothing was lost or late.
pub fn verdict(counts: &Counts) -> bool {
    counts.futex > 0
        && counts.ipc > 0
        && counts.sleeps > 0
        && counts.stalls == 0
        && counts.late == 0
        && counts.errors == 0
}

/// A snapshot of the counters, as the report prints them.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Counts {
    pub futex: u64,
    pub ipc: u64,
    pub sleeps: u64,
    pub stalls: u64,
    pub late: u64,
    pub errors: u64,
    pub max_futex_ms: u64,
    pub max_ipc_ms: u64,
    pub max_sleep_over_ms: u64,
}
