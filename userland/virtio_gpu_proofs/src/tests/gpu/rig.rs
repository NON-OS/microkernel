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
//! A driver brought up on the host model as bring-up does it: the scanouts
//! read, every enabled one taken (`display::take`), the wire server on top
//! (its primaries on the shared heap) and what a recovery needs. The model
//! answers on every yield of the driver's own thread; a kill panics there.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_gfx_serve::Server;
use nonos_libc::{dma_reset, off_yield, on_yield};

use crate::display::take::take;
use crate::driver::Driver;
use crate::setup::scanouts::seed;
use crate::state::{FenceCounter, ScanoutTable, Tally};
use crate::tests::harness::harness;
use crate::tests::host::state::Host;
use crate::tests::probe_stall::NOTIFY;
use crate::tests::wire_sweep::heap::Heap;

pub struct Rig {
    pub host: Rc<RefCell<Host>>,
    pub d: Driver<Heap>,
    pub regs: Arc<FakeBar>,
    _region: Arc<FakeBar>,
}

impl Drop for Rig {
    fn drop(&mut self) {
        off_yield();
        dma_reset();
    }
}

/// A rig whose host shows `displays` (width, height) on scanouts 0, 1, ...
pub fn rig_with(displays: &[(u32, u32)]) -> Rig {
    let host = Rc::new(RefCell::new(Host::default()));
    host.borrow_mut().displays[..displays.len()].copy_from_slice(displays);
    rig_on(host)
}

/// A driver brought up on `host` as it stands, its DMA starting afresh.
pub fn rig_on(host: Rc<RefCell<Host>>) -> Rig {
    dma_reset();
    let h = harness();
    let (gpu_host, region) = (Rc::clone(&host), Arc::clone(&h.region));
    on_yield(move || assert!(!gpu_host.borrow_mut().serve(&region), "the driver is killed"));
    let (table, fences) = (ScanoutTable::new(), FenceCounter::new());
    seed(&h.queue, &table, &fences).expect("display info");
    let gpu = take((h.queue, fences), (7, 1), &table, Default::default()).expect("taken");
    let server = Server::new(gpu, Heap::new(1 << 16));
    let d = Driver { device_id: 7, server, notify: NOTIFY, tally: Tally::default() };
    Rig { host, d, regs: h.regs, _region: h.region }
}

pub fn rig() -> Rig {
    rig_with(&[crate::tests::host::words::PANEL])
}
