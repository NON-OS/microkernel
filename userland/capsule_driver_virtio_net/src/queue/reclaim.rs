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

//! Taking transmit buffers back from the device, and telling a stopped
//! device from a busy one.

use core::sync::atomic::{fence, Ordering};

use super::clear::zero;
use super::{used, TxQueue};
use crate::constants::{RING_SLOTS, TX_STALL_MS};

impl TxQueue {
    /// Take back every buffer the device has finished with. Only a buffer
    /// posted and not yet returned is taken back, so an id named twice or
    /// never posted frees nothing, and at most one ring's worth of entries
    /// is read per call however far the device moved its index. A buffer
    /// that comes back is wiped: nothing of the frame stays behind. Called
    /// on every send and every receive poll.
    pub fn reclaim(&mut self) {
        let mut budget = RING_SLOTS;
        while budget > 0 && self.last_used != used::idx(self.ring) {
            budget -= 1;
            // The entry is only valid once the index covering it is seen.
            fence(Ordering::Acquire);
            let (id, _) = used::elem(self.ring, self.last_used);
            self.last_used = self.last_used.wrapping_add(1);
            let Some(slot) = u16::try_from(id).ok().filter(|&s| s < self.buf_count) else {
                continue;
            };
            if self.in_flight & (1 << slot) != 0 {
                self.in_flight &= !(1 << slot);
                self.completed = true;
                let len = self.buf_len as usize;
                zero(self.bufs, usize::from(slot) * len, len);
            }
        }
    }

    /// The lowest buffer the driver holds.
    pub fn free_slot(&self) -> Option<u16> {
        (0..self.buf_count).find(|&s| self.in_flight & (1 << s) == 0)
    }

    /// Whether a ring found full at `now_ms` has been full too long: once it
    /// has stayed full TX_STALL_MS with the link up. A link that is down
    /// holds frames for a reason, and restarts the count.
    pub fn stalled(&mut self, now_ms: u64, link_up: bool) -> bool {
        if !link_up {
            self.full_since = None;
            return false;
        }
        let since = *self.full_since.get_or_insert(now_ms);
        now_ms.saturating_sub(since) >= TX_STALL_MS
    }
}
