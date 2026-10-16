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

//! Keeping the tree's idle states at boot, while the tree is there.

use super::table::{Tree, TREE};
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::fdt::find::cpus::idle_lists;
use crate::arch::fdt::find::idle::{states, CpuStates, IdleState, Refusal, MAX_STATES};
use crate::arch::fdt::Fdt;

/// Keep the tree's states and each CPU's list; how many states were kept.
/// Read while the tree is, since it is gone by the time a CPU idles.
pub fn describe(fdt: &Fdt) -> Result<usize, Refusal> {
    let mut found = [IdleState::default(); MAX_STATES];
    let count = states(fdt, &mut found)?;
    let mut cpus = [CpuStates::default(); MAX_CPUS];
    idle_lists(fdt, &found[..count], &mut cpus)?;
    TREE.call_once(|| Tree { states: found, count, cpus });
    Ok(count)
}
