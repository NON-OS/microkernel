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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::compatible::{version, NEEDLES};
use super::info::GicInfo;

// arm,gic-v3 reg = (dist, redist[, ...]); GICv2 reg = (dist, cpu[, ...]).
// Both come back as CPU physical addresses, through any bus above the GIC.
pub fn find(fdt: &Fdt) -> Result<Option<GicInfo>, FdtError> {
    for found in fdt.devices(NEEDLES) {
        let (node, needle) = found?;
        let (Some((dist_base, _)), Some((second, _))) = (fdt.mmio(&node, 0)?, fdt.mmio(&node, 1)?)
        else {
            continue;
        };
        return Ok(Some(GicInfo {
            version: version(needle),
            dist_base,
            redist_or_cpu_base: second,
        }));
    }
    Ok(None)
}
