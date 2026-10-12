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

//! Handing a frame to the device without waiting for it: copied into a free
//! buffer behind a zeroed header (no offloads, one buffer), padded to the
//! Ethernet minimum and offered. Buffers come back in `reclaim`.

use super::clear::zero;
use super::ring::{publish, write_desc};
use super::{TxError, TxQueue};
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};

impl TxQueue {
    pub fn post(&mut self, frame: &[u8]) -> Result<(), TxError> {
        let wire = core::cmp::max(frame.len(), MIN_ETHERNET_FRAME);
        let total = self.hdr_len.checked_add(wire).filter(|&t| t <= self.buf_len as usize);
        let Some(total) = total.filter(|_| frame.len() <= MAX_ETHERNET_FRAME) else {
            return Err(TxError::TooLong);
        };
        self.reclaim();
        let Some(slot) = self.free_slot() else {
            return Err(TxError::Full);
        };
        // A buffer came back since the ring was last full: not stalled.
        self.full_since = None;
        let base = usize::from(slot) * self.buf_len as usize;
        zero(self.bufs, base, self.hdr_len);
        for (i, &b) in frame.iter().enumerate() {
            self.bufs.w8(base + self.hdr_len + i, b);
        }
        zero(self.bufs, base + self.hdr_len + frame.len(), wire - frame.len());
        // The grant was checked not to wrap (setup::grant), so neither does this.
        let addr = self.buf_phys.wrapping_add(base as u64);
        write_desc(self.ring, slot, addr, total as u32, 0);
        self.in_flight |= 1 << slot;
        publish(self.ring, &mut self.next_avail, slot);
        Ok(())
    }
}
