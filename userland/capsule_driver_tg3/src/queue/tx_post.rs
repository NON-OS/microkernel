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

//! Where a frame's bytes go and the send BD that points the part at them.

use core::ptr::{addr_of_mut, write_volatile};

use crate::constants::desc::{TXD_FLAG_END, TXD_LEN_SHIFT};
use crate::constants::layout::{TX_BUFFERS, TX_BUFFER_LEN};
use crate::constants::ring::TX_RING_SIZE;

use super::tx::TxRing;

impl TxRing {
    pub(super) fn slot_offset(idx: u16) -> u64 {
        (idx as usize % TX_BUFFERS * TX_BUFFER_LEN) as u64
    }

    /// Where the next frame's bytes go.
    pub fn next_buffer_va(&self) -> u64 {
        self.buffer_va + Self::slot_offset(self.prod)
    }

    /// One BD for a whole frame in the next slot (tg3_tx_set_bd with
    /// TXD_FLAG_END and no offload); returns the producer index to write.
    pub fn post(&mut self, len: u16) -> u16 {
        let at = self.buffer_device_addr + Self::slot_offset(self.prod);
        let bd = self.bd(self.prod as usize);
        // SAFETY: `bd` reduced the index into the send ring inside the grant.
        unsafe {
            write_volatile(addr_of_mut!((*bd).addr_hi), (at >> 32) as u32);
            write_volatile(addr_of_mut!((*bd).addr_lo), at as u32);
            write_volatile(
                addr_of_mut!((*bd).len_flags),
                (len as u32) << TXD_LEN_SHIFT | TXD_FLAG_END,
            );
            write_volatile(addr_of_mut!((*bd).vlan_tag), 0);
        }
        self.prod = ((self.prod as usize + 1) % TX_RING_SIZE) as u16;
        self.prod
    }
}
