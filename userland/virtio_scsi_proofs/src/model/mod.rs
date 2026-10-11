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

//! A virtio-scsi device as QEMU's virtio-scsi-pci answers the driver: the
//! transport proofs' common configuration, the device configuration with
//! its writable request sizes, and the request queue, which walks each
//! chain through the grant and answers it from `nonos_scsi`'s simulated
//! logical units.

mod queue;
mod regs;
#[path = "../../../nonos_scsi/src/sim.rs"]
pub mod sim;
#[path = "../../../virtio_transport_proofs/src/tests/device.rs"]
pub mod transport;

pub use queue::Chain;
pub use regs::{Regs, Which};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use nonos_virtio::features::VIRTIO_F_VERSION_1;

use sim::Unit;
use transport::ModelDevice;

pub struct Cfg {
    pub num_queues: u32,
    pub max_sectors: u32,
    pub sense_size: u32,
    pub cdb_size: u32,
    pub max_target: u16,
    pub max_lun: u32,
    /// The request sizes stay as they are whatever is written.
    pub sizes_fixed: bool,
}

pub struct Rest {
    pub cfg: Cfg,
    pub units: BTreeMap<(u16, u16), Unit>,
    /// Requests taken from the ring but never returned.
    pub stall: bool,
    /// The next request is answered with this response code alone.
    pub response: Option<u8>,
    pub seen_avail: u16,
    pub used_idx: u16,
    /// Every chain the device walked, for the shapes.
    pub chains: Vec<Chain>,
    pub notifies: u32,
}

pub struct Scsi {
    pub common: ModelDevice,
    pub rest: RefCell<Rest>,
}

pub type Shared = Rc<Scsi>;

/// QEMU's defaults: one request queue, the control and event queues
/// beside it, every queue up to 128 entries, notify offset N for queue N.
pub fn device(units: Vec<((u16, u16), Unit)>) -> Shared {
    Rc::new(Scsi {
        common: ModelDevice::new(VIRTIO_F_VERSION_1, &[128, 128, 128]),
        rest: RefCell::new(Rest {
            cfg: Cfg {
                num_queues: 1,
                max_sectors: 0xFFFF,
                sense_size: 96,
                cdb_size: 32,
                max_target: 255,
                max_lun: 16383,
                sizes_fixed: false,
            },
            units: units.into_iter().collect(),
            stall: false,
            response: None,
            seen_avail: 0,
            used_idx: 0,
            chains: Vec::new(),
            notifies: 0,
        }),
    })
}

impl Scsi {
    pub fn unit(&self, t: u16, l: u16) -> std::cell::RefMut<'_, Unit> {
        std::cell::RefMut::map(self.rest.borrow_mut(), |r| {
            r.units.get_mut(&(t, l)).expect("the unit")
        })
    }
}
