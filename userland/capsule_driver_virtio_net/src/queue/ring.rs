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

//! The split-ring parts the driver writes: a descriptor, and the next
//! available entry. Each goes through the region's bounds checks, so no
//! index, however it was reached, writes outside the ring.

use nonos_virtio::Mmio;

use crate::constants::{RING_SLOTS, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET};

const DESC_SIZE: usize = 16;
const AVAIL_IDX: usize = VQ_AVAIL_OFFSET + 2;
const AVAIL_RING: usize = VQ_AVAIL_OFFSET + 4;

/// Descriptor `index`: one buffer of `len` bytes at device address `addr`.
pub fn write_desc(ring: Mmio, index: u16, addr: u64, len: u32, flags: u16) {
    let at = VQ_DESC_OFFSET + DESC_SIZE * usize::from(index);
    ring.w32(at, addr as u32);
    ring.w32(at + 4, (addr >> 32) as u32);
    ring.w32(at + 8, len);
    ring.w16(at + 12, flags);
    ring.w16(at + 14, 0);
}

/// Offer descriptor `head` as available entry `*next`, then publish the
/// index. `next` is the driver's own count: the copy in shared memory is
/// the device's to read, not the driver's to trust.
pub fn publish(ring: Mmio, next: &mut u16, head: u16) {
    ring.w16(AVAIL_RING + 2 * usize::from(*next % RING_SLOTS), head);
    // The device may read the entry, its descriptor and the buffer the
    // moment the index covers them.
    nonos_libc::dma_wmb();
    *next = next.wrapping_add(1);
    ring.w16(AVAIL_IDX, *next);
}
