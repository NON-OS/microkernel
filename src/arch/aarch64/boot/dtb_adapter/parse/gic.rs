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

use super::describe;
use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::gic::GicVersion;
use crate::arch::fdt::find::gic::{find, GicVersion as FdtGicVersion};
use crate::arch::fdt::Fdt;

/// The GIC the tree names. Every board this kernel boots on has one, so a
/// tree without one we drive is refused rather than met with virt's.
pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) -> Result<(), &'static [u8]> {
    let g = find(fdt)
        .map_err(describe)?
        .ok_or(&b"the device tree names no GIC this kernel drives"[..])?;
    info.gic_dist_base = g.dist_base;
    // The second `reg` entry is the CPU interface on GICv2 and the first
    // redistributor region on GICv3.
    if g.version == FdtGicVersion::V3 {
        info.gic_version = Some(GicVersion::V3);
        info.gic_redist_base = g.redist_or_cpu_base;
    } else {
        info.gic_version = Some(GicVersion::V2);
        info.gic_cpu_base = g.redist_or_cpu_base;
    }
    Ok(())
}
