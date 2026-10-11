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

use super::info::SmmuInfo;
use super::irqs::named_spi;

const SMMU_V3: &[&[u8]] = &[b"arm,smmu-v3"];

/// The first enabled SMMUv3 with a register window. A window too small for
/// both 64 KiB register pages is not one the driver can program, so it is
/// refused rather than passed on.
pub fn find(fdt: &Fdt) -> Result<Option<SmmuInfo>, FdtError> {
    for found in fdt.devices(SMMU_V3) {
        let (node, _) = found?;
        let Some((base, size)) = fdt.mmio(&node, 0)? else {
            continue;
        };
        if size < 0x2_0000 {
            return Err(FdtError::BadCells);
        }
        let phandle = match fdt.property(&node, b"phandle")? {
            Some(p) if p.data.len() == 4 => p.u32()?,
            _ => 0,
        };
        let coherent = fdt.property(&node, b"dma-coherent")?.is_some();
        let eventq_intid = named_spi(fdt, &node, b"eventq")?;
        let gerror_intid = named_spi(fdt, &node, b"gerror")?;
        return Ok(Some(SmmuInfo { base, size, phandle, coherent, eventq_intid, gerror_intid }));
    }
    Ok(None)
}
