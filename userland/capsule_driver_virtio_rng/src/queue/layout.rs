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

//! The request queue: the ring and the entropy buffer, each a bounds-checked
//! view of its DMA grant, and the driver's own count of what it posted and
//! what it took back. The copies of those counts in shared memory are the
//! device's to write, so none is believed until it agrees with these.

use nonos_virtio::Mmio;

pub struct Queue {
    pub(super) ring: Mmio,
    pub(super) buf: Mmio,
    pub(super) buf_phys: u64,
    pub(super) avail_offset: usize,
    /// Ring entries the device was told: positions run modulo this.
    pub(super) queue_size: u16,
    /// Requests published, and completions taken. One request is in flight
    /// at most, so they differ by one while it is.
    pub(super) posted: u16,
    pub(super) used: u16,
    /// A request timed out, or the device answered one it was never given:
    /// the device may still write the buffer, so nothing is posted or read
    /// until it is reset and the queue built afresh.
    pub(super) stalled: bool,
}

impl Queue {
    /// `ring` holds VQ_REGION_SIZE bytes and `buf` the bytes the descriptor
    /// offers, at device address `buf_phys`.
    pub fn new(ring: Mmio, buf: Mmio, buf_phys: u64, queue_size: u16) -> Self {
        const DESC_BYTES: usize = 16;
        Self {
            ring,
            buf,
            buf_phys,
            avail_offset: usize::from(queue_size) * DESC_BYTES,
            queue_size,
            posted: 0,
            used: 0,
            stalled: false,
        }
    }

    /// Whether the device must be reset before the queue is used again.
    pub fn stalled(&self) -> bool {
        self.stalled
    }

    /// The request in flight is given up on: it stays the device's.
    pub fn stall(&mut self) {
        self.stalled = true;
    }

    /// A ring position reduced to the ring. A zero size (which bring-up
    /// never hands over) is taken as one entry rather than divided by.
    pub fn ring_pos(&self, idx: u16) -> usize {
        usize::from(idx % self.queue_size.max(1))
    }
}
