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

//! The control queue, then DRIVER_OK. The ring region is laid out for 256
//! entries at fixed offsets (descriptors, available ring, used ring, a page
//! each), which hold any smaller ring too, so the device is told the
//! largest power of two it offers up to 256. A command takes two
//! descriptors, so a ring of one is refused. Any refusal leaves the device
//! FAILED, and its doorbell must fall inside the mapped notify region.

use nonos_virtio::common::{driver_ok, queue_max, setup_queue, NotifyArea, QueueSpec};
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::{CTRLQ_INDEX, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_MAX_SIZE, VQ_USED_OFFSET};

pub struct Started {
    pub queue_size: u16,
    /// The control queue's doorbell, an offset into the notify region.
    pub doorbell: usize,
}

pub fn program(
    common: &Mmio,
    notify: NotifyArea,
    queue_phys: u64,
) -> Result<Started, &'static str> {
    let offered = core::cmp::min(queue_max(common, CTRLQ_INDEX), VQ_MAX_SIZE);
    let size = match offered.checked_ilog2() {
        Some(log) => 1 << log,
        None => 0,
    };
    let at = |off: usize| queue_phys.checked_add(off as u64).ok_or("virtio-gpu: ring address");
    let spec = QueueSpec {
        index: CTRLQ_INDEX,
        size,
        min_size: 2,
        desc: at(VQ_DESC_OFFSET)?,
        driver: at(VQ_AVAIL_OFFSET)?,
        device: at(VQ_USED_OFFSET)?,
        vector: None,
    };
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    driver_ok(common);
    Ok(Started { queue_size: ready.size, doorbell: ready.notify_offset })
}
