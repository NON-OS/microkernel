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

//! The request queue on the modern transport. The layout is the legacy one
//! (`queue::ring`), so its size is chosen first: the largest power of two
//! the device offers up to MODERN_QUEUE_SIZE, and at least the four a
//! request's three descriptors need. With MSI-X bound, the device resets
//! every vector to NO_VECTOR and raises nothing until told otherwise, so
//! the queue is pointed at table entry 0 (the one the kernel bound) and
//! configuration changes at no vector: a resize is not handled (plan/57
//! names it its own unit). A refusal reads back as NO_VECTOR; requests
//! then complete by the wait's slices.

use nonos_libc::mk_debug;
use nonos_virtio::common::{set_config_vector, setup_queue, NotifyArea, QueueSpec, NO_VECTOR};
use nonos_virtio::{Mmio, VirtioError};

use super::super::dma::Grants;
use crate::constants::VQ_DESC_OFFSET;
use crate::queue::{avail_offset, used_offset, Queue};

/// The MSI-X table entry the kernel bound.
const MSIX_ENTRY: u16 = 0;

pub struct Ready {
    pub queue: Queue,
    pub doorbell: usize,
}

pub fn setup(
    c: &Mmio,
    notify: NotifyArea,
    dma: &Grants,
    size: u16,
    msix: bool,
) -> Result<Ready, &'static str> {
    let queue = Queue::new(dma.ring.view, size, dma.header, dma.data).ok_or("virtio-blk: ring")?;
    let at = |off: usize| dma.ring.phys.checked_add(off as u64).ok_or("virtio-blk: ring address");
    let spec = QueueSpec {
        index: 0,
        size,
        min_size: size,
        desc: at(VQ_DESC_OFFSET)?,
        driver: at(avail_offset(size))?,
        device: at(used_offset(size))?,
        vector: msix.then_some(MSIX_ENTRY),
    };
    if msix {
        let _ = set_config_vector(c, NO_VECTOR);
    }
    // The emptied ring reaches the device before the writes that hand it
    // over, as a legacy window write orders itself (`regs::write`).
    nonos_libc::dma_wmb();
    let ready = setup_queue(c, &spec, notify).map_err(VirtioError::message)?;
    if msix && !ready.vector_taken {
        let line = b"[BLK] device refused MSI-X entry 0; completions are found by timeout\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    Ok(Ready { queue, doorbell: ready.notify_offset })
}
