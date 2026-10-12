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

//! The control queue's region: descriptors, available ring and used ring a
//! page each, then a page of staging for a command and its answer. The
//! driver reaches it only through a bounds-checked view of its DMA grant,
//! so no index, however it was reached, touches memory outside it.

use nonos_virtio::Mmio;

use crate::constants::{VQ_MAX_SIZE, VQ_REGION_SIZE, VQ_STAGING_LEN, VQ_STAGING_OFFSET};

#[derive(Clone, Copy)]
pub struct QueueLayout {
    pub queue_size: u16,
    /// The driver's view of the whole region.
    pub ring: Mmio,
    /// Where the device finds it.
    pub region_device_addr: u64,
}

impl QueueLayout {
    pub fn new(queue_size: u16, ring: Mmio, region_device_addr: u64) -> Result<Self, &'static str> {
        // A command is a chain of two descriptors, so a ring of one would
        // chain a descriptor to itself.
        if !(2..=VQ_MAX_SIZE).contains(&queue_size) {
            return Err("virtio-gpu: queue size out of range");
        }
        if !queue_size.is_power_of_two() {
            return Err("virtio-gpu: queue size not power of two");
        }
        let end = region_device_addr.checked_add(VQ_REGION_SIZE as u64);
        if ring.len() < VQ_REGION_SIZE || region_device_addr == 0 || end.is_none() {
            return Err("virtio-gpu: vq region not mapped");
        }
        Ok(Self { queue_size, ring, region_device_addr })
    }

    /// Where the device finds the staging page; inside the region, which
    /// `new` checked does not wrap.
    pub fn staging_device_addr(self) -> u64 {
        self.region_device_addr + VQ_STAGING_OFFSET as u64
    }

    pub const fn staging_len(self) -> usize {
        VQ_STAGING_LEN
    }
}
