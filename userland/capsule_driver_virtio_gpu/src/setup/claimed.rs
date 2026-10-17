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
//! scanouts, the primary and the blank (G-73), into one driver.

use super::{blank_surface, create_primary, dma, edid, mmio, pci, probe_3d, scanouts};
use crate::device::virtqueue::QueueLayout;
use crate::device::ControlQueue;
use crate::discover::Found;
use crate::driver::Driver;
use crate::init::bring_up;
use crate::state::{FenceCounter, Owner, ResourceTable, ScanoutTable};

pub fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    /*
     * No interrupt is bound: the driver polls, and `pci::enable` takes the
     * device off its legacy line so it cannot hold a shared line up.
     */
    pci::enable(dev.device_id, claim_epoch)?;
    let registers = mmio::grant(dev, claim_epoch)?;
    let queue = dma::map_queue(dev.device_id, claim_epoch, registers)?;
    let init = bring_up(registers.regs(dev.pci_device), queue.device_addr, dev.pci_device)?;
    let regs = init.regs;
    let layout = QueueLayout::new(init.queue_size, queue.user_va, queue.device_addr)?;
    let control_queue = ControlQueue::new(layout, regs);
    let (scanouts, fences, resources) =
        (ScanoutTable::new(), FenceCounter::new(), ResourceTable::new());
    scanouts::seed(&control_queue, &scanouts, &fences)?;
    edid::fetch(&control_queue, init.edid);
    // 3D bring-up is best-effort: a host without a GL backend still gets the
    // full 2D scanout path, and the boot log records which one this is.
    let virgl_ready =
        probe_3d::probe(&control_queue, &fences, init.virgl, dev.device_id, claim_epoch);
    let (q, ids) = (&control_queue, (dev.device_id, claim_epoch));
    let primary = create_primary::create(ids.0, ids.1, q, &fences, &resources, &scanouts)?;
    let blank = primary.and_then(|p| blank_surface::create(ids, q, &fences, &resources, &p));
    Ok(Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: registers.grant_id(),
        queue_grant: queue.grant_id,
        queue_user_va: queue.user_va,
        queue_device_addr: queue.device_addr,
        queue_size: init.queue_size,
        host_features: init.host_features,
        virgl_ready,
        regs,
        control_queue,
        resources,
        scanouts,
        fences,
        primary,
        blank,
        owner: Owner::new(),
    })
}
