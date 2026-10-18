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
//! The park itself is libc's (`IrqPark`), so the policy is the one every
//! driver shares.

use nonos_libc::wait::IrqPark;

pub struct IrqWait(IrqPark);

impl IrqWait {
    /// 0 when no interrupt is bound; a grant the kernel refuses is dropped
    /// at its first wait.
    pub fn new(grant: u64) -> Self {
        let park = IrqPark::new();
        park.set_grant(grant);
        Self(park)
    }

    /// One step of a completion poll: spin while the budget lasts, then
    /// block off the run queue until the interrupt or the park ceiling.
    /// Without an interrupt, yield for a while and then sleep.
    pub fn park(&mut self, spins: u32) {
        self.0.park_step(spins);
    }
}
