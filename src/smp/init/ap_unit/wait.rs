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

//! How long the boot CPU waits on an AP at each step, and the wait itself.

use crate::smp::boot_budget::budget_ticks;

/*
 * Two deadlines, because there are two different questions.
 *
 * Entry: did the AP run at all? A woken AP reaches Rust in microseconds on
 * real hardware. The budget is generous for emulators with many vCPUs, and it
 * is the one that decides whether the trampoline may be reused: past it the
 * boot CPU takes the AP's claim and parks it with INIT.
 *
 * Online: the AP has entered and is setting itself up (GDT, IDT, guard pages,
 * LAPIC timer). That takes locks the boot CPU may hold, so it can be slower,
 * and an AP past entry is never sent INIT: it may be holding one of them.
 */
pub(super) const AP_ENTRY_TIMEOUT_MS: u64 = 1000;
pub(super) const AP_ONLINE_TIMEOUT_MS: u64 = 1000;
/// How long an AP asked to park has to answer. It checks between every step
/// of its bring-up, each a few microseconds on hardware, so an AP that has not
/// answered by then is inside a step, not slow between two. Short against the
/// boot: a wedged AP costs this once.
pub(super) const AP_PARK_TIMEOUT_MS: u64 = 100;
/// Used when the counter rate is unknown: about a second at up to 5 GHz.
pub(super) const AP_TIMEOUT_FALLBACK_TICKS: u64 = 5_000_000_000;

pub(super) fn wait_for(done: impl Fn() -> bool, ms: u64) -> bool {
    let start = super::super::time::read_tsc();
    let budget =
        budget_ticks(ms, crate::sys::timer::tsc::tsc_frequency(), AP_TIMEOUT_FALLBACK_TICKS);
    while !done() {
        if super::super::time::read_tsc().wrapping_sub(start) > budget {
            return done();
        }
        core::hint::spin_loop();
    }
    true
}
