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

//! The interrupt one event ring's waits block on. Each controller's ring
//! holds its own: one process-wide grant, overwritten by every controller
//! brought up, left the primary's waits blocked on a secondary's vector
//! and woken only by the park ceiling, or polling when that bind failed.

use nonos_libc::{mk_idle_ms, mk_irq_wait, mk_yield};

use super::step::{step, Step};

// Ceiling on a single park. The controller interrupt wakes the capsule the
// moment the event lands, so this only bounds the path where no interrupt
// arrives at all; a device that never answers then costs one wakeup per
// scheduler tick instead of a whole core.
const PARK_MS: u64 = 2;

pub struct IrqWait {
    /// 0 when no interrupt is bound, or the kernel refused the grant.
    grant: u64,
    seq: u64,
}

impl IrqWait {
    pub fn new(grant: u64) -> Self {
        Self { grant, seq: 0 }
    }

    /// One step of a completion poll: spin while the budget lasts, then
    /// block off the run queue until the interrupt or the park ceiling.
    /// Without an interrupt, yield for a while and then sleep.
    pub fn park(&mut self, spins: u32) {
        match step(spins, self.grant != 0) {
            Step::Spin => core::hint::spin_loop(),
            Step::Yield => {
                mk_yield();
            }
            Step::Sleep => {
                let _ = mk_idle_ms(PARK_MS);
            }
            Step::Wait => {
                let mut seq = self.seq;
                if mk_irq_wait(self.grant, seq, PARK_MS, &mut seq) < 0 {
                    // The kernel refused the grant: poll without it from now
                    // on, and sleep this step rather than spin straight back.
                    self.grant = 0;
                    let _ = mk_idle_ms(PARK_MS);
                    return;
                }
                self.seq = seq;
            }
        }
    }
}
