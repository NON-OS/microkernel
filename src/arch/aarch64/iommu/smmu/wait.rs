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

//! How the driver waits on the unit: against the generic counter with a
//! budget, so a unit that never answers costs a refused bring-up or a
//! failed call instead of a hung CPU.

use crate::arch::aarch64::timer::generic::{current_count, milliseconds_to_ticks};

/// The architecture bounds none of these waits; real units answer in
/// microseconds, so this is a ceiling on patience, not an estimate.
const BUDGET_MS: u64 = 10;
/// The ceiling when the counter has no frequency.
const MAX_POLLS: u32 = 1 << 24;

pub(super) fn until(mut done: impl FnMut() -> bool) -> bool {
    let budget = milliseconds_to_ticks(BUDGET_MS);
    let start = current_count();
    for _ in 0..MAX_POLLS {
        if done() {
            return true;
        }
        if budget != 0 && current_count().wrapping_sub(start) >= budget {
            return done();
        }
        core::hint::spin_loop();
    }
    done()
}

/// Make the kernel's stores to tables and queues visible to the unit before
/// the register write that tells it to look. The unit is coherent, so
/// ordering them is enough; nothing needs cleaning to memory.
pub(super) fn publish() {
    // SAFETY: a barrier; it touches no memory and changes no state.
    unsafe { core::arch::asm!("dsb ishst", options(nostack, preserves_flags)) }
}
