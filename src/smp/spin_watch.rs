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

//! When a lock wait is worth a word, pure: kernel_proofs::spin_watch.
//!
//! A spin with interrupts masked that never ends stops its CPU without a
//! sound, and on one CPU the whole machine. The wait says so once it has
//! lasted `REPORT_MS`, naming where it waits and where the lock was taken.

/// How long a wait goes on before it is reported, in milliseconds. A healthy
/// hold is microseconds; under emulation a long one is tens of milliseconds.
pub(crate) const REPORT_MS: u64 = 1000;

/// Slots in the table of who took each lock last.
pub(crate) const SLOTS: usize = 64;

/// The deepest a call chain reaches below its first frame on one kernel
/// stack: a process's kernel stack is 32 KiB.
pub(crate) const CHAIN_SPAN: usize = 32 * 1024;

/// The table slot for the lock at `addr`. Locks are at least 8-byte aligned,
/// so the low three bits say nothing.
pub(crate) const fn slot_of(addr: usize) -> usize {
    (addr >> 3) % SLOTS
}

/// Whether a wait that began at `start` ms is due its one report at `now`.
pub(crate) const fn due(start: u64, now: u64, reported: bool) -> bool {
    !reported && now.saturating_sub(start) >= REPORT_MS
}

/// Whether the holder's frame at `holder_sp` sits above the waiter's frame at
/// `waiter_sp` on the same stack: the waiter's own caller holds the lock, and
/// the wait can never end. Two stacks laid side by side can look the same, so
/// the report says "likely".
pub(crate) const fn same_chain(holder_sp: usize, waiter_sp: usize) -> bool {
    holder_sp > waiter_sp && holder_sp - waiter_sp < CHAIN_SPAN
}
