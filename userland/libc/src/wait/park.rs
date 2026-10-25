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

//! Carrying out a step. A capsule keeps one `IrqPark` per interrupt grant,
//! in a static if its waits are spread over many functions; the grant and
//! the delivery sequence it last saw are per park, so two devices in one
//! capsule never consume each other's wakeups.

use core::sync::atomic::{AtomicU64, Ordering};

use super::step::{step, Step, PARK_MS};
use crate::{mk_idle_ms, mk_irq_wait, mk_yield};

pub struct IrqPark {
    grant: AtomicU64,
    seq: AtomicU64,
}

impl IrqPark {
    /// A park with no grant: it yields and then sleeps until one is set.
    pub const fn new() -> Self {
        Self { grant: AtomicU64::new(0), seq: AtomicU64::new(0) }
    }

    /// Block on `grant_id` from now on; 0 goes back to polling without one.
    pub fn set_grant(&self, grant_id: u64) {
        self.grant.store(grant_id, Ordering::Relaxed);
    }

    /// One poll's wait, `spins` being the polls made so far. The caller
    /// bounds the whole wait by its own `Deadline`.
    pub fn park_step(&self, spins: u32) {
        let grant = self.grant.load(Ordering::Relaxed);
        match step(spins, grant != 0) {
            Step::Spin => core::hint::spin_loop(),
            Step::Yield => {
                mk_yield();
            }
            Step::Sleep => {
                let _ = mk_idle_ms(PARK_MS);
            }
            Step::Wait => {
                let mut seq = self.seq.load(Ordering::Relaxed);
                if mk_irq_wait(grant, seq, PARK_MS, &mut seq) < 0 {
                    // The kernel refused the grant: poll without it from now
                    // on, and sleep this step rather than spin straight back.
                    self.grant.store(0, Ordering::Relaxed);
                    let _ = mk_idle_ms(PARK_MS);
                    return;
                }
                self.seq.store(seq, Ordering::Relaxed);
            }
        }
    }
}

impl Default for IrqPark {
    fn default() -> Self {
        Self::new()
    }
}
