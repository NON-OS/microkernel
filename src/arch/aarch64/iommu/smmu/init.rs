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

//! Bringing the device tree's SMMUv3 into service.

use crate::arch::fdt::find::smmu::{IommuMap, SmmuInfo};
use crate::memory::addr::PhysAddr;

use super::cmdq::Cmdq;
use super::domain::MAX_DOMAINS;
use super::encode::idr::decode;
use super::eventq::Eventq;
use super::mmio::Regs;
use super::regs::{IDR0, IDR1, IDR5, WINDOW};
use super::reset::reset;
use super::state::{Smmu, DESCRIBED, SMMU};
use super::strtab::StreamTable;

/// Record what the tree describes. Boot calls this before paging exists;
/// bring-up, which maps the registers, comes later.
pub fn describe(info: SmmuInfo, map: Option<IommuMap>) {
    DESCRIBED.call_once(|| (info, map));
}

/// Map, check and enable the unit. On any refusal it stays off, every
/// device keeps physical addresses, and the reason is the caller's to state.
pub fn bring_up() -> Result<(), &'static str> {
    let &(info, map) = DESCRIBED.get().ok_or("the device tree names no SMMUv3")?;
    if !info.coherent {
        return Err("the SMMUv3 is not dma-coherent, and the driver does not clean its tables");
    }
    let va = crate::memory::mmio::map_device_memory(PhysAddr::new(info.base), WINDOW)
        .map_err(|_| "the SMMUv3 register window cannot be mapped")?;
    // SAFETY: `map_device_memory` returned a live device mapping of both
    // register pages, and it is never unmapped.
    let regs = unsafe { Regs::new(va.as_u64()) };
    let features = decode(regs.read32(IDR0), regs.read32(IDR1), regs.read32(IDR5))
        .map_err(super::refusal::reason)?;
    let oom = |_| "no memory for the SMMUv3's queues and tables";
    let mut cmdq = Cmdq::new(features.cmdq_log2).map_err(oom)?;
    let eventq = Eventq::new(features.eventq_log2).map_err(oom)?;
    let strtab = StreamTable::new(features.sid_bits).map_err(oom)?;
    reset(regs, &mut cmdq, &eventq, &strtab)?;
    *SMMU.lock() = Some(Smmu {
        regs,
        cmdq,
        strtab,
        domains: [None; MAX_DOMAINS],
        features,
        map,
        phandle: info.phandle,
        eventq,
    });
    super::irq::wire(regs, info.eventq_intid, info.gerror_intid);
    Ok(())
}
