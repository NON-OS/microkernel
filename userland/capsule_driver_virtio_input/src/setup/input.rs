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

//! One virtio-input function as the driver holds it: its register regions,
//! the event queue, the axes it reports on, and the pointer report being
//! gathered. It runs for the life of the capsule; the kernel takes its
//! claim and grants back when the capsule ends.

use nonos_libc::{mk_device_release, mk_dma_unmap};
use nonos_virtio::map::Window;
use nonos_virtio::Mmio;

use super::broker::LibcBroker;
use crate::events::Report;
use crate::init::Axes;
use crate::queue::EventRing;

pub struct Input {
    pub window: Window,
    pub notify: Mmio,
    pub doorbell: usize,
    pub ring: EventRing,
    pub axes: Axes,
    pub report: Report,
}

impl Input {
    /// Tell the device the event queue has buffers again.
    pub fn kick(&self) {
        self.notify.w16(self.doorbell, 0);
    }
}

/// Reset the device, so it stops writing into the queue, then give back the
/// register regions, the DMA grant and the claim. Used on any failure after
/// the regions are mapped; a release refused because the grant is already
/// gone is harmless.
pub fn abandon(device_id: u64, epoch: u64, window: &Window, dma_grant: Option<u64>) {
    let _ = nonos_virtio::common::reset(&window.common.region);
    let _ = window.unmap(&mut LibcBroker::new(device_id, epoch));
    if let Some(g) = dma_grant {
        let _ = mk_dma_unmap(g);
    }
    let _ = mk_device_release(device_id);
}
