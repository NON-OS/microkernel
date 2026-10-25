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

//! One bring-up attempt: claim, PCI side, the ports and the schedule region
//! granted, and the controller started on them.

use nonos_libc::{mk_dma_map, mk_pio_grant, DmaMapOut, PioGrantOut};
use nonos_libc::{MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::claim::Claim;
use super::discover::Found;
use super::pci;
use crate::facts::pci::IO_BAR;
use crate::hc::start::start;
use crate::platform::{Ports, Region, Uptime};
use crate::sched::layout::REGION;
use crate::server::context::Ctx;

/// Field order is drop order: nothing here outlives the claim, and the
/// broker takes back every grant with it.
pub struct Driver {
    pub cx: Ctx<Ports, Uptime, Region>,
    pub found: Found,
    pub(super) claim: Claim,
}

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim = Claim::take(dev.device_id)?;
    pci::enable(&claim)?;
    let mut pio = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(claim.device_id, claim.epoch, IO_BAR as u8, 0, &mut pio) < 0 {
        return Err("uhci: the broker refused the port grant");
    }
    let mut map = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let flags = MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT;
    if mk_dma_map(claim.device_id, claim.epoch, REGION as u64, flags, &mut map) < 0 {
        return Err("uhci: the broker refused the schedule region");
    }
    let region = Region::new(&map).ok_or("uhci: schedule region not below 4 GiB")?;
    let mut cx = Ctx::new(Ports::new(pio.grant_id), Uptime, region);
    start(&mut cx.io, &cx.clock, &cx.mem)?;
    Ok(Driver { cx, found: dev, claim })
}
