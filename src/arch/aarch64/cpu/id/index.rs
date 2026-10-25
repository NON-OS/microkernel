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

//! Which CPU is executing, as the dense index every per-CPU table takes.

use super::affinity::mpidr_affinity;

/// This CPU's dense index: its place in the roster, the boot CPU's 0.
///
/// Below `roster::len()`, which is at most MAX_CPUS, so it indexes every
/// per-CPU table without a fold. Zero before the roster is latched, when
/// only the boot CPU runs. A CPU the roster does not hold has no index that
/// is not another CPU's, and parks here instead. It is not an affinity:
/// never hand it to the GIC or to PSCI, which want `cpu_affinity` or
/// `mpidr_affinity`.
pub fn cpu_id() -> usize {
    use crate::arch::aarch64::boot::multicore::{park_outside_roster, roster};
    let affinity = mpidr_affinity();
    match roster::index_of(affinity) {
        Some(index) => index,
        None if roster::len() == 0 => 0,
        None => park_outside_roster(affinity),
    }
}
