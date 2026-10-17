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
//! A driver brought up on the host model the way bring-up does it, through
//! the shipping steps: scanouts read, the primary made and shown, and the
//! blank made. The model answers on every yield of the driver's own
//! thread; a cued kill panics there, as the capsule dying mid-wait.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_libc::dma_reset;

use super::hang::hang;
use crate::driver::Driver;
use crate::regs::Regs;
use crate::setup::{blank_surface, create_primary, scanouts};
use crate::state::{FenceCounter, Owner, ResourceTable, ScanoutTable};
use crate::tests::harness::harness;
use crate::tests::host::gpu::Gpu;
use crate::tests::model::QUEUE_SIZE;

pub struct Rig {
    pub gpu: Rc<RefCell<Gpu>>,
    pub d: Driver,
    pub(super) _bars: [Arc<nonos_devmodel::FakeBar>; 2],
}

pub fn rig() -> Rig {
    dma_reset();
    let h = harness();
    let gpu = Rc::new(RefCell::new(Gpu::default()));
    hang(&gpu, h.region.clone());
    let (q, ids) = (h.queue, (7u64, 1u64));
    let (scanouts, fences, resources) =
        (ScanoutTable::new(), FenceCounter::new(), ResourceTable::new());
    scanouts::seed(&q, &scanouts, &fences).expect("display info");
    let primary = create_primary::create(ids.0, ids.1, &q, &fences, &resources, &scanouts)
        .expect("a primary");
    let blank = primary.and_then(|p| blank_surface::create(ids, &q, &fences, &resources, &p));
    let d = Driver {
        device_id: ids.0,
        pci_device: 0x1050,
        claim_epoch: ids.1,
        mmio_grant: 0,
        queue_grant: 0,
        queue_user_va: h.region.base(),
        queue_device_addr: h.region.base(),
        queue_size: QUEUE_SIZE,
        host_features: 0,
        virgl_ready: false,
        regs: Regs::mmio(h.regs.base()),
        control_queue: q,
        resources,
        scanouts,
        fences,
        primary,
        blank,
        owner: Owner::new(),
    };
    Rig { gpu, d, _bars: [h.region, h.regs] }
}
