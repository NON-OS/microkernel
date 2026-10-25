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

//! A secondary filling in its own descriptor once it is up (aarch64).
//!
//! x86_64's boot CPU fills each AP's descriptor before starting it
//! (init/ap_unit.rs). aarch64's secondaries are started by the arch layer,
//! and until this each one ran with an empty descriptor: no hardware id for
//! an IPI to find it by, and never Online, so no shootdown waited on it.

use super::state::CPU_DESCRIPTORS;
use super::types::CpuState;

/// The calling CPU, at roster index `index`, with hardware id `hw_id` (its
/// packed affinity). The index comes from the roster, which holds at most
/// MAX_CPUS; one past the table is refused, never folded onto another's.
pub fn register_secondary(index: usize, hw_id: u32) {
    let Some(descriptor) = CPU_DESCRIPTORS.get(index) else {
        return;
    };
    descriptor.set_identity(index as u32, hw_id, 0);
    descriptor.set_state(CpuState::Online);
}
