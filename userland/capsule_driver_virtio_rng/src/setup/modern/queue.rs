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

//! The request queue on the modern transport.
//!
//! The layout is the legacy one: descriptors at the start of the region,
//! the available ring right after a table of `queue_size` descriptors (where
//! `Queue::new` puts it) and the used ring at VQ_USED_OFFSET. The size is
//! the device's maximum held to QUEUE_SIZE, as on the legacy path, and is
//! chosen before the addresses because the available ring's place follows
//! from it.

use nonos_virtio::common::{choose_size, queue_max, setup_queue, NotifyArea, QueueSpec};
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::{QUEUE_SIZE, VQ_DESC_OFFSET, VQ_USED_OFFSET};

const DESC_BYTES: u64 = 16;

pub struct Ready {
    pub queue_size: u16,
    pub doorbell: usize,
}

pub fn setup(common: &Mmio, notify: NotifyArea, region: u64) -> Result<Ready, &'static str> {
    let size = choose_size(queue_max(common, 0), QUEUE_SIZE, 1).map_err(VirtioError::message)?;
    let spec = QueueSpec {
        index: 0,
        size,
        min_size: size,
        desc: region.checked_add(VQ_DESC_OFFSET as u64).ok_or("virtio-rng: ring address")?,
        driver: region
            .checked_add(VQ_DESC_OFFSET as u64 + size as u64 * DESC_BYTES)
            .ok_or("virtio-rng: ring address")?,
        device: region.checked_add(VQ_USED_OFFSET as u64).ok_or("virtio-rng: ring address")?,
        vector: None,
    };
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    Ok(Ready { queue_size: ready.size, doorbell: ready.notify_offset })
}
