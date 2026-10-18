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

//! The controller's waits park through libc's shared policy, on one park for
//! the controller interrupt; the step and budgets are libc's.

use nonos_libc::wait::IrqPark;

pub use nonos_libc::wait::SPIN_BUDGET;
// Reached by the host proofs (userland/xhci_proofs), not by the capsule.
#[allow(unused_imports)]
pub use nonos_libc::wait::{step, Step, YIELD_BUDGET};

static PARK: IrqPark = IrqPark::new();

pub fn set_irq_grant(grant_id: u64) {
    PARK.set_grant(grant_id);
}

/// One step of a completion poll; see `IrqPark::park_step`.
pub fn park_step(spins: u32) {
    PARK.park_step(spins);
}
