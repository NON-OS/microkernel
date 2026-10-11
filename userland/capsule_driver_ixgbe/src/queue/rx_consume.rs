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

use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::RX_DESC_COUNT;

use super::rx::RxRing;
use super::rx_wb::{parse, Rx};

/*
 * SAFETY anchor for every unsafe in this file: `descriptor(self.head)` is in
 * the ring grant because `head` only ever moves modulo RX_DESC_COUNT here.
 */
impl RxRing {
    /// The slot at the head once the part has written it back, with the
    /// frame length or 0 for a slot to recycle. The descriptor is put back
    /// in read format at once, since write-back overwrote the buffer address;
    /// the part only sees it again when the caller moves RDT onto it.
    pub fn consume(&mut self) -> Option<(u16, u16)> {
        let desc = unsafe { self.descriptor(self.head) };
        // The part writes by DMA: every read goes to memory, none is hoisted.
        if unsafe { read_volatile(addr_of!((*desc).staterr)) } == 0 {
            return None;
        }
        // Length and the full status are the part's only once status is seen.
        fence(Ordering::Acquire);
        let staterr = unsafe { read_volatile(addr_of!((*desc).staterr)) };
        let length = unsafe { read_volatile(addr_of!((*desc).length)) };
        let len = match parse(staterr, length) {
            Rx::NotDone => return None,
            Rx::Drop => 0,
            Rx::Frame(n) => n,
        };
        let idx = self.head;
        unsafe {
            write_volatile(addr_of_mut!((*desc).buffer_addr), self.buffer_phys(idx));
            write_volatile(addr_of_mut!((*desc).staterr), 0);
            write_volatile(addr_of_mut!((*desc).length), 0);
            write_volatile(addr_of_mut!((*desc).vlan), 0);
        }
        self.head = (self.head + 1) % (RX_DESC_COUNT as u16);
        Some((idx, len))
    }
}
