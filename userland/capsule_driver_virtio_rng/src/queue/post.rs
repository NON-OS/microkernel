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

//! Push descriptor 0 into the available ring. virtio-rng has one virtqueue
//! and one request in flight at most: the device fills the buffer
//! descriptor 0 points at and posts a used element with the byte count.

use super::fault::Fault;
use super::layout::Queue;
use super::wipe::wipe;
use crate::constants::{VQ_DESC_OFFSET, VRING_DESC_F_WRITE};

impl Queue {
    /// Point descriptor 0 at the emptied buffer, device-writable, and
    /// publish it at the available ring's next position. Refused while an
    /// earlier request may still be the device's: posting over it would let
    /// its completion pass for this one.
    pub fn post(&mut self) -> Result<(), Fault> {
        if self.stalled || self.posted != self.used {
            self.stalled = true;
            return Err(Fault::Stalled);
        }
        // A completion that wrote less than the buffer leaves zeros, never
        // bytes an earlier request left behind.
        wipe(self.buf);
        let desc = VQ_DESC_OFFSET;
        self.ring.w32(desc, self.buf_phys as u32);
        self.ring.w32(desc + 4, (self.buf_phys >> 32) as u32);
        self.ring.w32(desc + 8, u32::try_from(self.buf.len()).unwrap_or(u32::MAX));
        self.ring.w16(desc + 12, VRING_DESC_F_WRITE);
        self.ring.w16(desc + 14, 0);
        // VirtqAvail: u16 flags, u16 idx, u16 ring[queue_size].
        let slot = self.avail_offset + 4 + 2 * self.ring_pos(self.posted);
        self.ring.w16(slot, 0);
        // The device may read the entry and its descriptor the moment the
        // index covers them.
        nonos_libc::dma_wmb();
        self.posted = self.posted.wrapping_add(1);
        self.ring.w16(self.avail_offset + 2, self.posted);
        Ok(())
    }
}
