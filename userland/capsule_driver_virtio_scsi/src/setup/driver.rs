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

//! What the capsule holds for its life: the units it serves over the
//! adapter, and what it gives back if it leaves.

use nonos_libc::{mk_device_release, mk_dma_unmap};
use nonos_scsi_serve::Units;
use nonos_virtio::map::Window;
use nonos_virtio::Mmio;

use super::broker::LibcBroker;
use crate::hba::VirtioHba;

pub struct Driver {
    pub units: Units<VirtioHba<Mmio>>,
    pub window: Window,
    pub device_id: u64,
    pub epoch: u64,
    pub dma_grant: u64,
}

impl Driver {
    /// Reset the device, so it stops touching the ring and the buffers, then
    /// give back the register regions, the DMA grant and the claim.
    pub fn release(&self) {
        abandon(self.device_id, self.epoch, &self.window, Some(self.dma_grant));
    }
}

/// The same, for a bring-up that fails after the regions are mapped.
pub fn abandon(device_id: u64, epoch: u64, window: &Window, dma_grant: Option<u64>) {
    let _ = nonos_virtio::common::reset(&window.common.region);
    let _ = window.unmap(&mut LibcBroker::new(device_id, epoch));
    if let Some(g) = dma_grant {
        let _ = mk_dma_unmap(g);
    }
    let _ = mk_device_release(device_id);
}
