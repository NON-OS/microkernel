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

//! Reset, features and the request queue on the modern transport.
//!
//! The ring layout is the legacy one, built by `Queue::new` from the ring
//! size: descriptors at the start of the region, the available ring right
//! after them, the used ring on the next page. The size is the device's
//! maximum held to 256, and at least the three descriptors one request
//! chains, as on the legacy path; it is chosen first and the device is told
//! where `Queue::new` put each ring.
//!
//! With MSI-X bound, the device resets every vector to NO_VECTOR and raises
//! nothing until told otherwise, so the queue is pointed at table entry 0
//! (the one the kernel bound) and configuration changes at no vector, as
//! `vectors::assign` does on the legacy path. A refusal reads back as
//! NO_VECTOR; requests then complete by the wait's timeout, as there.

use nonos_libc::{mk_debug, DmaMapOut};
use nonos_virtio::common::{
    accept, choose_size, queue_max, set_config_vector, setup_queue, start, NotifyArea, QueueSpec,
    NO_VECTOR,
};
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::VQ_DESC_OFFSET;
use crate::queue::Queue;

/// VIRTIO_BLK_F_FLUSH, the one device feature the legacy path takes too.
const VIRTIO_BLK_F_FLUSH: u64 = 1 << 9;
/// A request chains a header, a data buffer and a status byte.
const MIN_QUEUE_SIZE: u16 = 3;
/// The MSI-X table entry the kernel bound.
const MSIX_ENTRY: u16 = 0;

pub struct Ready {
    pub queue: Queue,
    pub doorbell: usize,
}

pub struct Dma<'a> {
    pub queue: &'a DmaMapOut,
    pub header: &'a DmaMapOut,
    pub data: &'a DmaMapOut,
}

pub fn run(common: &Mmio, notify: NotifyArea, dma: Dma, msix: bool) -> Result<Ready, &'static str> {
    start(common).map_err(VirtioError::message)?;
    accept(common, VIRTIO_BLK_F_FLUSH).map_err(VirtioError::message)?;
    let max = queue_max(common, 0);
    let size = choose_size(max, Queue::max_supported_size(), MIN_QUEUE_SIZE)
        .map_err(VirtioError::message)?;
    let queue = Queue::new(
        dma.queue.user_va,
        dma.queue.device_addr,
        size,
        dma.header.user_va,
        dma.header.device_addr,
        dma.data.user_va,
        dma.data.device_addr,
    );
    let spec = spec(dma.queue.device_addr, &queue, size, msix).ok_or("virtio-blk: ring address")?;
    if msix {
        let _ = set_config_vector(common, NO_VECTOR);
    }
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    if msix && !ready.vector_taken {
        let line = b"[BLK] device refused MSI-X entry 0; completions are found by timeout\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    Ok(Ready { queue, doorbell: ready.notify_offset })
}

fn spec(base: u64, queue: &Queue, size: u16, msix: bool) -> Option<QueueSpec> {
    Some(QueueSpec {
        index: 0,
        size,
        min_size: size,
        desc: base.checked_add(VQ_DESC_OFFSET as u64)?,
        driver: base.checked_add(queue.avail_offset as u64)?,
        device: base.checked_add(queue.used_offset as u64)?,
        vector: msix.then_some(MSIX_ENTRY),
    })
}
