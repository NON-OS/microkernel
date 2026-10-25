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
use super::request::{
    REQ_PENDING_ACKS, SHOOTDOWN_TIMEOUT_FALLBACK_TICKS, SHOOTDOWN_TIMEOUT_MS,
    SHOOTDOWN_WARN_FALLBACK_TICKS, SHOOTDOWN_WARN_MS,
};
use super::slow::{fail_timed_out, warn_slow};

/// `ms` as counter ticks, or `fallback` when the counter is not calibrated.
fn budget(ms: u64, fallback: u64) -> u64 {
    match crate::sys::timer::tsc::tsc_frequency() / 1000 * ms {
        0 => fallback,
        ticks => ticks,
    }
}

/// Returns how many acknowledgements the waiter paid for abandoned targets.
pub(super) fn wait_for_acks(selected: &[u64]) -> u32 {
    let start = read_tsc();
    let warn_at = start.wrapping_add(budget(SHOOTDOWN_WARN_MS, SHOOTDOWN_WARN_FALLBACK_TICKS));
    let deadline =
        start.wrapping_add(budget(SHOOTDOWN_TIMEOUT_MS, SHOOTDOWN_TIMEOUT_FALLBACK_TICKS));
    let mut warned = false;
    let me = crate::smp::percpu::current();
    let mut answered_for = 0u32;
    while REQ_PENDING_ACKS.load(Ordering::Acquire) > 0 {
        /*
         * Keep answering while waiting. `SHOOTDOWN_LOCK` admits one round at
         * a time and a round never targets its originator, so nothing should
         * be marked for this cpu now; but a waiter that stops listening is
         * exactly how two cpus end up waiting on each other, so the wait never
         * relies on that. The pending flag makes a spurious call a no-op.
         */
        serve_for(me);
        answered_for += answer_for_abandoned(selected);
        let now = read_tsc();
        if !warned && now > warn_at {
            warned = true;
            warn_slow();
        }
        if now > deadline {
            let outstanding = REQ_PENDING_ACKS.load(Ordering::Acquire);
            if outstanding == 0 {
                return answered_for;
            }
            fail_timed_out(outstanding);
        }
        core::hint::spin_loop();
    }
    answered_for
}

/// Acknowledge for each target the boot CPU gave up on after `select` read it
/// as `Starting`: one that never claimed its start, or one that claimed,
/// parked at a checkpoint and was reset (`ap_unit::park`). Either way INIT
/// emptied its TLB, and it will never answer; without this the round waits to
/// its deadline and stops the machine. Taking the mark is what makes exactly one side acknowledge:
/// the cpu itself, had it got there, would have taken it first.
fn answer_for_abandoned(selected: &[u64]) -> u32 {
    let mut answered = 0;
    for cpu in 0..crate::smp::MAX_CPUS {
        if selected[cpu / 64] & (1u64 << (cpu % 64)) == 0 || !crate::smp::cpu_is_offline(cpu) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if d.tlb_flush_pending.swap(0, Ordering::AcqRel) != 0 {
            REQ_PENDING_ACKS.fetch_sub(1, Ordering::Release);
            answered += 1;
        }
    }
    answered
}

#[inline]
fn read_tsc() -> u64 {
    // SAFETY: eK@nonos.systems — rdtsc has no side effects and is
    // unconditionally available on every x86_64 CPU NØNOS supports.
    crate::arch::read_time_counter()
}
