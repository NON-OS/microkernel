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

/// A process's kernel stack, 32 KiB: the span a call chain can cover.
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

/// Who holds a lock a wait has gone on too long for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Holder {
    /// Somebody else, on another stack: the wait may yet end.
    Other,
    /// The holder's frame is just above the waiter's, but the waiter's own
    /// stack bounds are not known, so two stacks side by side could look so.
    Likely,
    /// The holder's frame is on the waiter's own kernel stack, above it: the
    /// waiter's own caller holds the lock, and the wait can never end. Live
    /// stacks never overlap, so a hold on any other CPU is never this.
    ReTaken,
}

/// Judge the holder's frame at `holder_sp` against the waiter's at
/// `waiter_sp`, whose kernel stack ends at `stack_top` (0 when unknown).
pub(crate) const fn judge(holder_sp: usize, waiter_sp: usize, stack_top: usize) -> Holder {
    let above = holder_sp > waiter_sp;
    // A top of 0 is no stack: no frame lies below it.
    let on_known_stack = waiter_sp < stack_top && waiter_sp >= stack_top.saturating_sub(CHAIN_SPAN);
    if on_known_stack {
        return if above && holder_sp < stack_top { Holder::ReTaken } else { Holder::Other };
    }
    if above && holder_sp - waiter_sp < CHAIN_SPAN {
        Holder::Likely
    } else {
        Holder::Other
    }
}
