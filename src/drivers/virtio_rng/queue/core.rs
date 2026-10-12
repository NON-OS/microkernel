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

//! One virtqueue of the device's own size, one descriptor at a time: each
//! request is a single device-writable buffer of up to a page.

use super::dma::DmaRegion;
use super::layout::{layout, Layout};

/// Available-ring flag: the driver polls, so the device raises no
/// interrupt. Its INTx line is shared on q35 and no handler is bound.
const AVAIL_F_NO_INTERRUPT: u16 = 1;

pub(crate) struct RngQueue {
    pub(super) ring: DmaRegion,
    pub(super) buf: DmaRegion,
    pub(super) layout: Layout,
    pub(super) size: u16,
    pub(super) last_used_idx: u16,
    pub(super) next_avail_idx: u16,
    pub(super) pending_len: usize,
    pub(super) notify_port: u16,
}

impl RngQueue {
    /// A queue of `size` entries, the device's own, kicked through `notify_port`.
    pub(crate) fn new(size: u16, notify_port: u16) -> Result<Self, &'static str> {
        let layout = layout(size).ok_or("virtio-rng: queue size not supported")?;
        let ring = DmaRegion::new(layout.bytes.div_ceil(4096))?;
        let buf = DmaRegion::new(1)?;
        let queue = Self {
            ring,
            buf,
            layout,
            size,
            last_used_idx: 0,
            next_avail_idx: 0,
            pending_len: 0,
            notify_port,
        };
        queue.put16(queue.layout.avail, AVAIL_F_NO_INTERRUPT);
        Ok(queue)
    }

    /// The ring's page frame number for QUEUE_PFN, a 32-bit register.
    pub(crate) fn pfn(&self) -> Option<u32> {
        u32::try_from(self.ring.phys >> 12).ok()
    }
}
