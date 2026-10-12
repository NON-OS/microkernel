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

//! Starting one secondary by whichever method the device tree gave it.

use crate::arch::aarch64::asm::{_aarch64_secondary_start, _aarch64_spin_table_start};
use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::psci;

use super::spin_table;

/// Start the CPU at dense `index`: released from its spin table when firmware
/// parked it on one, powered on through PSCI otherwise.
pub(super) fn start(
    info: &BootInfo,
    index: usize,
    affinity: u64,
    stack_top: u64,
) -> Result<(), &'static [u8]> {
    match info.cpu_release.get(index).copied().flatten() {
        Some(word) => release_parked(info, index, affinity, stack_top, word),
        None => power_on(affinity, stack_top),
    }
}

/// PSCI names a core by its MPIDR affinity. Passing the loop counter instead
/// only lands on the right core when a board has a single cluster numbered
/// from zero.
fn power_on(affinity: u64, stack_top: u64) -> Result<(), &'static [u8]> {
    let entry = _aarch64_secondary_start as unsafe extern "C" fn() as u64;
    psci::cpu_on(affinity, entry, stack_top).map_err(|_| &b"PSCI CPU_ON refused it"[..])
}

fn release_parked(
    info: &BootInfo,
    index: usize,
    affinity: u64,
    stack_top: u64,
    word: u64,
) -> Result<(), &'static [u8]> {
    // The stack first: the CPU looks for it the moment the word changes.
    spin_table::assign_stack(index, affinity, stack_top)?;
    let entry = _aarch64_spin_table_start as unsafe extern "C" fn() as u64;
    spin_table::release(info, word, entry)
}
