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

//! A ring region, a register window, and the driver the server is given.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::NotifyArea;

use super::model::{modern_window, regs, view, NOTIFY_LEN, NOTIFY_MULT, QUEUE_SIZE};
use crate::constants::VQ_REGION_SIZE;
use crate::device::virtqueue::{ControlQueue, QueueLayout};
use crate::driver::Driver;
use crate::regs::Regs;
use crate::setup::Primary;
use crate::state::{FenceCounter, Health, ResourceTable, ScanoutTable};

/// Where in the notify region the control queue's doorbell is.
pub const DOORBELL: usize = 12;

/// A ring region and the modern window (offering VERSION_1, for a recovery
/// to negotiate). The region's device address is its host address.
pub struct Harness {
    pub region: Arc<FakeBar>,
    pub regs: Arc<FakeBar>,
    pub queue: ControlQueue,
}

pub fn harness() -> Harness {
    let region = Arc::new(FakeBar::new(VQ_REGION_SIZE));
    let window = modern_window(1, QUEUE_SIZE);
    let ring = view(&region, 0, VQ_REGION_SIZE);
    let layout = QueueLayout::new(QUEUE_SIZE, ring, region.base()).expect("a valid layout");
    let r = Regs { doorbell: DOORBELL, ..regs(&window) };
    Harness { region, regs: window, queue: ControlQueue::new(layout, r) }
}

/// The driver as bring-up leaves it, over `h`'s queue, with `primary`.
pub fn driver(h: Harness, primary: Option<Primary>) -> (Driver, Arc<FakeBar>, Arc<FakeBar>) {
    let d = Driver {
        device_id: 1,
        pci_device: 0x1050,
        claim_epoch: 1,
        mmio_grant: 0,
        queue_grant: 0,
        queue_user_va: 0,
        queue_device_addr: 0,
        queue_size: QUEUE_SIZE,
        host_features: 0,
        virgl_ready: false,
        control_queue: h.queue,
        resources: ResourceTable::new(),
        scanouts: ScanoutTable::new(),
        fences: FenceCounter::new(),
        primary,
        notify: NotifyArea { multiplier: NOTIFY_MULT, len: NOTIFY_LEN },
        health: Health::Live,
        recoveries: 0,
    };
    (d, h.region, h.regs)
}
