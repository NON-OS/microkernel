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

//! Everything after the register window: the transport, the scanouts, the
//! panel's size, the 3D probe and the primary surface.

use nonos_virtio::Window;

use super::{create_primary, edid, probe_3d, scanouts, transport};
use crate::constants::{VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL};
use crate::discover::Found;
use crate::driver::Driver;
use crate::state::{FenceCounter, Health, ResourceTable, ScanoutTable};

pub fn run(dev: Found, claim_epoch: u64, window: &Window) -> Result<Driver, &'static str> {
    let ready = transport::start(dev.device_id, claim_epoch, window)?;
    let q = &ready.control_queue;
    let (scanouts, fences, resources) =
        (ScanoutTable::new(), FenceCounter::new(), ResourceTable::new());
    scanouts::seed(q, &scanouts, &fences)?;
    edid::fetch(q, ready.features & u64::from(VIRTIO_GPU_F_EDID) != 0);
    // 3D bring-up is best-effort: a host without a GL backend still gets the
    // full 2D scanout path, and the boot log records which one this is.
    let virgl = ready.features & u64::from(VIRTIO_GPU_F_VIRGL) != 0;
    let virgl_ready = probe_3d::probe(q, &fences, virgl, dev.device_id, claim_epoch);
    let primary =
        create_primary::create(dev.device_id, claim_epoch, q, &fences, &resources, &scanouts)?;
    Ok(Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: window.common.grant_id,
        queue_grant: ready.queue.grant_id,
        queue_user_va: ready.queue.user_va,
        queue_device_addr: ready.queue.device_addr,
        queue_size: q.layout.queue_size,
        // The low feature word, as OP_CONTROLLER_INFO reports it.
        host_features: ready.features as u32,
        virgl_ready,
        control_queue: ready.control_queue,
        resources,
        scanouts,
        fences,
        primary,
        notify: window.notify_area(),
        health: Health::Live,
        recoveries: 0,
    })
}
