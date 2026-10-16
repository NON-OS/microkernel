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

//! The driver the server is given: as bring-up leaves it, over a harness.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::NotifyArea;

use super::harness::Harness;
use super::model::{NOTIFY_LEN, NOTIFY_MULT, QUEUE_SIZE};
use crate::driver::Driver;
use crate::setup::Primary;
use crate::state::{FenceCounter, Health, ResourceTable, ScanoutTable};

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
        in_a_row: 0,
        recovered_at: 0,
        recovered_ms: 0,
    };
    (d, h.region, h.regs)
}
