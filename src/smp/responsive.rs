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

//! Taking a lock without going deaf to the other CPUs.
//!
//! Several scheduler locks are held with interrupts disabled, and for a good
//! single-CPU reason: the timer handler reaches the same queues, so a tick
//! landing mid-critical-section would spin on a lock the interrupted code can
//! never release. Masking interrupts closes that.
//!
//! On more than one CPU the same masking opens a worse hole. A TLB shootdown
//! is a request plus a wait: the originator holds every other CPU responsible
//! for acknowledging, and the acknowledgement arrives as an interrupt. A CPU
//! spinning for a lock with interrupts masked cannot answer, and if the CPU it
//! is waiting on is the one spinning in `wait_for_acks`, neither ever moves
//! and the shootdown deadline halts the machine. That is not hypothetical: it
//! is what `[FATAL] TLB shootdown timeout` was, the first time secondary CPUs
//! actually ran.
//!
//! So the spin answers shootdowns itself rather than waiting to be
//! interrupted. Interrupts stay masked, so the single-CPU reasoning above is
//! untouched, and the cross-CPU request is still serviced.

use core::panic::Location;
use spin::{Mutex, MutexGuard};

use super::spin_holders::{note, stack_mark};
use super::spin_wait::wait;

/// Acquire `lock`, servicing TLB shootdowns while it is contended.
///
/// Use this in place of `lock()` anywhere the caller holds interrupts
/// disabled. `handle_shootdown_ipi` is written to be driven either by its
/// vector or by hand: it clears this CPU's pending flag before acknowledging,
/// so being called from both cannot acknowledge one round twice.
///
/// A wait that lasts `REPORT_MS` says so once, naming this call site and the
/// one that took the lock (spin_report.rs): a spin that can never end, such as
/// a holder taking its own lock again, is otherwise a CPU stopped in silence.
#[track_caller]
pub fn lock_responsive<T>(lock: &Mutex<T>) -> MutexGuard<'_, T> {
    let site = Location::caller();
    let sp = stack_mark();
    let addr = lock as *const Mutex<T> as usize;
    let guard = match lock.try_lock() {
        Some(guard) => guard,
        None => wait(lock, addr, site, sp),
    };
    note(addr, site, sp);
    guard
}
