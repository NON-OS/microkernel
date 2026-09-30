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

use spin::{Mutex, MutexGuard};

use crate::memory::paging::manager::handle_shootdown_ipi;
use crate::smp::cpus_online;

/// Acquire `lock`, servicing TLB shootdowns while it is contended.
///
/// Use this in place of `lock()` anywhere the caller holds interrupts
/// disabled. `handle_shootdown_ipi` is written to be driven either by its
/// vector or by hand: it clears this CPU's pending flag before acknowledging,
/// so being called from both cannot acknowledge one round twice.
pub fn lock_responsive<T>(lock: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        if let Some(guard) = lock.try_lock() {
            return guard;
        }
        serve_shootdowns();
        core::hint::spin_loop();
    }
}

/// Answer a TLB shootdown aimed at this CPU, if one is waiting.
///
/// Every system call runs with interrupts masked (SFMASK clears IF), so a
/// call that does a long stretch of work without taking a lock never hears
/// the shootdown vector either. A full-screen present is one: a 1920x1080
/// blit moves 8 MiB with interrupts off, which under emulation outlasts the
/// shootdown budget, and the originator halted the machine while the
/// presenting CPU was still copying. Such loops call this between units of
/// work. Answering early is always safe: the flush only drops translations,
/// and the pending flag makes a later delivery of the vector a no-op.
#[inline]
pub fn serve_shootdowns() {
    /*
     * Nothing can be pending on a uniprocessor, and resolving the current
     * CPU costs an interrupt-controller read, so the common case pays only
     * the compare.
     */
    if cpus_online() > 1 {
        handle_shootdown_ipi();
    }
}
