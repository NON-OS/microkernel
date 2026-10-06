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

//! Taking the frame at `head` once the part has marked it DD. A frame that
//! is split, errored, empty or oversized comes back with length 0 so the
//! caller still hands the slot back.

use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{RX_DESC_COUNT, RX_STATUS_DD, RX_STATUS_EOP};
use crate::constants::MAX_ETHERNET_FRAME;

use super::rx::RxRing;

// SAFETY anchor: `head` is held below RX_DESC_COUNT, and the ring is the DMA
// grant taken in `setup::dma`, RX_DESC_COUNT descriptors long.
impl RxRing {
    pub fn consume(&mut self) -> Option<(u16, u16)> {
        let desc = unsafe { self.descriptor(self.head) };
        /*
         * The part writes these fields by DMA, so every read goes to memory. A
         * status read hoisted out of the poll loop would never see the
         * descriptor complete.
         */
        let status = unsafe { read_volatile(addr_of!((*desc).status)) };
        if status & RX_STATUS_DD == 0 {
            return None;
        }
        // Length, errors and the frame are only the part's once DD is seen.
        fence(Ordering::Acquire);
        let errors = unsafe { read_volatile(addr_of!((*desc).errors)) };
        let len = unsafe { read_volatile(addr_of!((*desc).length)) };
        let idx = self.head;
        unsafe {
            write_volatile(addr_of_mut!((*desc).status), 0);
            write_volatile(addr_of_mut!((*desc).errors), 0);
        }
        self.head = (self.head + 1) % (RX_DESC_COUNT as u16);
        if status & RX_STATUS_EOP == 0
            || errors != 0
            || len == 0
            || len as usize > MAX_ETHERNET_FRAME
        {
            return Some((idx, 0));
        }
        Some((idx, len))
    }
}
