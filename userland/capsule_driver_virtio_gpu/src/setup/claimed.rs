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
//! One claimed device brought up: registers, queue, reset and handshake,
//! scanouts, output 0's monitor, and every enabled scanout taken (G-77),
//! into the wire server.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;

use super::{dma, edid, mmio, pci, probe_3d, scanouts};
use crate::device::virtqueue::QueueLayout;
use crate::device::ControlQueue;
use crate::discover::Found;
use crate::display::take::take;
use crate::driver::Driver;
use crate::init::bring_up;
use crate::state::{FenceCounter, ScanoutTable};

pub fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    /*
     * No interrupt is bound: the driver polls, and `pci::enable` takes the
     * device off its legacy line so it cannot hold a shared line up.
     */
    pci::enable(dev.device_id, claim_epoch)?;
    let registers = mmio::grant(dev, claim_epoch)?;
    let queue = dma::map_queue(dev.device_id, claim_epoch, registers)?;
    let init = bring_up(registers.regs(dev.pci_device), queue.device_addr, dev.pci_device)?;
    let layout = QueueLayout::new(init.queue_size, queue.user_va, queue.device_addr)?;
    let control_queue = ControlQueue::new(layout, init.regs);
    let (scanouts, fences) = (ScanoutTable::new(), FenceCounter::new());
    scanouts::seed(&control_queue, &scanouts, &fences)?;
    // 3D bring-up is best-effort: a host without a GL backend still gets the
    // full 2D scanout path, and the boot log records which one this is.
    probe_3d::probe(&control_queue, &fences, init.virgl, dev.device_id, claim_epoch);
    let panel = edid::panel(&control_queue, init.edid);
    let device = (dev.device_id, claim_epoch);
    let gpu = take((control_queue, fences), device, &scanouts, panel)?;
    Ok(Driver { device_id: dev.device_id, server: Server::new(gpu, Pool) })
}
