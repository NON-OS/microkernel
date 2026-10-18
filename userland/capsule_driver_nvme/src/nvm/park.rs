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

//! What an I/O wait does between rounds of polling its completion ring. It
//! used to spin a core for a command's whole latency, up to the 30 s a
//! stalled DRAM-less drive may take. Now each round of polls (the shared
//! loop's DEADLINE_CHECK_SPINS) ends in a park on the controller's MSI-X
//! interrupt, as the xHCI driver's waits do, or, with no vector bound, one
//! yield. A command that completes within the round never parks.

use nonos_libc::{mk_irq_wait, mk_yield};

use crate::clock::{self, Budget};

/// Ceiling on one park. The interrupt wakes the wait as the completion
/// lands, so this bounds only the path where none comes: a command that
/// never completes costs a wakeup every 2 ms, not a core.
const PARK_MS: u64 = 2;

#[derive(Clone, Copy)]
pub(crate) struct Park {
    /// The MSI-X grant the I/O completion queue signals; 0 for none.
    grant: u64,
    /// The delivery count last seen on it.
    seq: u64,
}

impl Park {
    pub(crate) const fn on(grant: u64) -> Self {
        Self { grant, seq: 0 }
    }

    /// Park until the interrupt or PARK_MS, whichever is first; spurious
    /// wakes only end the park early. A grant the kernel refuses is dropped,
    /// and the queue yields from then on.
    fn step(&mut self) {
        if self.grant != 0 && mk_irq_wait(self.grant, self.seq, PARK_MS, &mut self.seq) >= 0 {
            return;
        }
        self.grant = 0;
        let _ = mk_yield();
    }
}

/// The deadline of a wait on `budget`, parking before each check of the
/// clock. A clock that cannot be read spends the budget at once.
pub(super) fn parked(budget: Budget, park: &mut Park) -> impl FnMut() -> bool + '_ {
    move || {
        park.step();
        budget.spent(clock::uptime_ms())
    }
}
