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

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::find::cpus;
use crate::arch::fdt::Fdt;

/// Ceiling on how many CPUs one device tree can describe to us. The stack bank
/// is sized for the same number.
const MAX_CPUS: usize = crate::arch::aarch64::boot::stack::MAX_CPUS;

pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) -> Result<(), &'static [u8]> {
    let mut affinities = [0u64; MAX_CPUS];
    let n = cpus::find(fdt, &mut affinities).map_err(super::describe)?;
    if n == 0 {
        return Ok(());
    }
    info.cpu_count = n as u32;
    // Keep the affinities, not just the count. They are what PSCI and the GIC
    // are addressed by, and the position of each one is the kernel's dense CPU
    // index from here on.
    let take = n.min(info.cpu_affinities.len());
    info.cpu_affinities[..take].copy_from_slice(&affinities[..take]);
    info.cpu_affinity_count = take;
    // Both walks skip the same nodes, so release address `i` is CPU `i`'s.
    let mut release = [None; MAX_CPUS];
    let r = cpus::release_addresses(fdt, &mut release).map_err(super::describe)?;
    let take = r.min(take);
    info.cpu_release[..take].copy_from_slice(&release[..take]);
    Ok(())
}
