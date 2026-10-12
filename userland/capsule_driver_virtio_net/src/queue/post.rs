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

//! Giving receive buffers to the device.

use super::ring::{publish, write_desc};
use super::RxQueue;
use crate::constants::VRING_DESC_F_WRITE;

impl RxQueue {
    /// One device-writable descriptor per buffer, each offered once.
    pub fn prime(&mut self) {
        for slot in 0..self.buf_count {
            // The grant was checked not to wrap (setup::grant), so neither does this.
            let addr = self.buf_phys.wrapping_add(u64::from(self.buf_len) * u64::from(slot));
            write_desc(self.ring, slot, addr, self.buf_len, VRING_DESC_F_WRITE);
            publish(self.ring, &mut self.next_avail, slot);
        }
    }

    /// Offer buffer `slot` again; its descriptor never changed.
    pub fn refill(&mut self, slot: u16) {
        publish(self.ring, &mut self.next_avail, slot);
    }
}
