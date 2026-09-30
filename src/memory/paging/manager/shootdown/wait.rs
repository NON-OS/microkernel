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

use core::sync::atomic::Ordering;

use super::handle::serve_for;
use super::report::report_stuck;
use super::request::{
    REQ_PENDING_ACKS, SHOOTDOWN_TIMEOUT_FALLBACK_TICKS, SHOOTDOWN_TIMEOUT_MS,
    SHOOTDOWN_WARN_FALLBACK_TICKS, SHOOTDOWN_WARN_MS,
};

/// `ms` as counter ticks, or `fallback` when the counter is not calibrated.
fn budget(ms: u64, fallback: u64) -> u64 {
    match crate::sys::timer::tsc::tsc_frequency() / 1000 * ms {
        0 => fallback,
        ticks => ticks,
    }
}

pub(super) fn wait_for_acks() {
    let start = read_tsc();
    let warn_at = start.wrapping_add(budget(SHOOTDOWN_WARN_MS, SHOOTDOWN_WARN_FALLBACK_TICKS));
    let deadline =
        start.wrapping_add(budget(SHOOTDOWN_TIMEOUT_MS, SHOOTDOWN_TIMEOUT_FALLBACK_TICKS));
    let mut warned = false;
    let me = crate::smp::percpu::current();
    while REQ_PENDING_ACKS.load(Ordering::Acquire) > 0 {
        /*
         * Keep answering while waiting. `SHOOTDOWN_LOCK` admits one round at
         * a time and a round never targets its originator, so nothing should
         * be marked for this cpu now; but a waiter that stops listening is
         * exactly how two cpus end up waiting on each other, so the wait never
         * relies on that. The pending flag makes a spurious call a no-op.
         */
        serve_for(me);
        let now = read_tsc();
        if !warned && now > warn_at {
            warned = true;
            let outstanding = REQ_PENDING_ACKS.load(Ordering::Acquire);
            if outstanding != 0 {
                let mut line = crate::sys::serial::Line::new();
                line.str(b"[SMP] tlb shootdown slow: acks outstanding=").dec(outstanding as u64);
                line.str(b" after ms=").dec(SHOOTDOWN_WARN_MS);
                line.end();
            }
        }
        if now > deadline {
            let outstanding = REQ_PENDING_ACKS.load(Ordering::Acquire);
            if outstanding == 0 {
                return;
            }
            let mut line = crate::sys::serial::Line::new();
            line.str(b"[FATAL] TLB shootdown timeout outstanding=").dec(outstanding as u64);
            line.str(b" ms=").dec(SHOOTDOWN_TIMEOUT_MS);
            line.end();
            report_stuck();
            crate::smp::send_panic_ipi();
            crate::arch::halt_loop();
        }
        core::hint::spin_loop();
    }
}

#[inline]
fn read_tsc() -> u64 {
    // SAFETY: eK@nonos.systems — rdtsc has no side effects and is
    // unconditionally available on every x86_64 CPU NØNOS supports.
    crate::arch::read_time_counter()
}
