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

use super::report::report_stuck;
use super::request::{REQ_PENDING_ACKS, SHOOTDOWN_TIMEOUT_FALLBACK_TICKS, SHOOTDOWN_TIMEOUT_MS};

fn shootdown_timeout_ticks() -> u64 {
    let ticks = crate::sys::timer::tsc::tsc_frequency() / 1000 * SHOOTDOWN_TIMEOUT_MS;
    if ticks == 0 {
        return SHOOTDOWN_TIMEOUT_FALLBACK_TICKS;
    }
    ticks
}

pub(super) fn wait_for_acks() {
    let budget = shootdown_timeout_ticks();
    let deadline = read_tsc().wrapping_add(budget);
    while REQ_PENDING_ACKS.load(Ordering::Acquire) > 0 {
        if read_tsc() > deadline {
            let outstanding = REQ_PENDING_ACKS.load(Ordering::Acquire);
            if outstanding == 0 {
                return;
            }
            let mut line = crate::sys::serial::Line::new();
            line.str(b"[FATAL] TLB shootdown timeout outstanding=").dec(outstanding as u64);
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
