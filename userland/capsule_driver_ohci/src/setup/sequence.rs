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

//! One bring-up attempt: claim, PCI side, the registers and the schedule
//! region granted, the controller started on them and its ports powered.

use nonos_libc::{mk_dma_map, mk_mmio_map, DmaMapOut, MmioMapOut};
use nonos_libc::{MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::claim::Claim;
use super::discover::Found;
use super::pci;
use crate::facts::pci::MMIO_BAR;
use crate::hc::power::power;
use crate::hc::start::start;
use crate::platform::{Region, Registers, Uptime};
use crate::sched::layout::REGION;
use crate::server::context::Ctx;

const PAGE: u64 = 0xFFF;

/// Field order is drop order: nothing here outlives the claim, and the
/// broker takes back every grant with it.
pub struct Driver {
    pub cx: Ctx<Registers, Uptime, Region>,
    pub found: Found,
    _claim: Claim,
}

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim = Claim::take(dev.device_id)?;
    pci::enable(&claim)?;
    let mut regs = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let len = (dev.size + PAGE) & !PAGE;
    if mk_mmio_map(claim.device_id, claim.epoch, MMIO_BAR, 0, 0, len, &mut regs) < 0 {
        return Err("ohci: the broker refused the register grant");
    }
    let mut map = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let flags = MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT;
    if mk_dma_map(claim.device_id, claim.epoch, REGION as u64, flags, &mut map) < 0 {
        return Err("ohci: the broker refused the schedule region");
    }
    let region = Region::new(&map).ok_or("ohci: schedule region not below 4 GiB")?;
    let registers = Registers::new(&regs);
    let mapped = registers.mapped();
    let mut cx = Ctx::new(registers, Uptime, region);
    start(&mut cx.io, &cx.clock, &cx.mem, mapped)?;
    cx.table.ports = power(&mut cx.io, &cx.clock);
    Ok(Driver { cx, found: dev, _claim: claim })
}
