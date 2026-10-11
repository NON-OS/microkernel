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

//! Setting the receive rings up and putting buffers on the standard ring
//! (tg3_rx_prodring_alloc, tg3_alloc_rx_data).

use core::ptr::{addr_of_mut, write_bytes, write_volatile};

use crate::constants::desc::{RXD_FLAG_END, RXD_OPAQUE_RING_STD};
use crate::constants::layout::RX_BUFFERS;
use crate::constants::ring::{RX_RET_RING_SIZE, RX_STD_DMA_SZ, RX_STD_RING_SIZE};

use super::rx::RxQueue;

/*
 * SAFETY: `std_bd` and `ret_bd` reduce every index into their ring inside
 * the rings grant (rx.rs).
 */
impl RxQueue {
    /// Every standard slot gets its invariant words, both rings start
    /// empty, and RX_BUFFERS buffers go on the first slots.
    pub fn init(&mut self) {
        unsafe {
            write_bytes(self.ret_bd(0), 0, RX_RET_RING_SIZE);
            write_bytes(self.std_bd(0), 0, RX_STD_RING_SIZE);
        }
        for i in 0..RX_STD_RING_SIZE {
            let bd = self.std_bd(i);
            unsafe {
                write_volatile(addr_of_mut!((*bd).idx_len), RX_STD_DMA_SZ);
                write_volatile(addr_of_mut!((*bd).type_flags), RXD_FLAG_END);
                let opaque = RXD_OPAQUE_RING_STD | (i as u32);
                write_volatile(addr_of_mut!((*bd).opaque), opaque);
            }
        }
        self.carries = [None; RX_STD_RING_SIZE];
        (self.std_prod, self.ret_cons) = (0, 0);
        for buf in 0..RX_BUFFERS {
            self.post(buf as u8);
        }
    }

    /// Put `buf` on the next standard slot; the caller moves the producer
    /// mailbox to `std_prod` once its batch is posted.
    pub fn post(&mut self, buf: u8) {
        let slot = self.std_prod as usize;
        let at = self.buffer_device_addr(buf);
        let bd = self.std_bd(slot);
        unsafe {
            write_volatile(addr_of_mut!((*bd).addr_hi), (at >> 32) as u32);
            write_volatile(addr_of_mut!((*bd).addr_lo), at as u32);
        }
        self.carries[slot] = Some(buf);
        self.std_prod = ((slot + 1) % RX_STD_RING_SIZE) as u16;
    }
}

// A buffer is named by a byte in `carries`.
const _: () = assert!(RX_BUFFERS <= u8::MAX as usize + 1);
