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

//! How the UART waits on its FIFO: against the generic counter, with a
//! budget, so a transmitter held off by flow control or a receiver that
//! never drains costs dropped bytes instead of a hung CPU.

use crate::arch::aarch64::timer::generic::{current_count, milliseconds_to_ticks};

/// One byte time at 1200 baud is about 8 ms; nothing this kernel boots on is
/// slower, so a FIFO with no room after this is not going to make any.
const BUDGET_MS: u64 = 10;

/// The ceiling when the counter has no frequency yet: firmware is meant to
/// set CNTFRQ_EL0, and a console is exactly where a missing one must not
/// turn into either a hang or silence.
const MAX_POLLS: u32 = 1 << 22;

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
