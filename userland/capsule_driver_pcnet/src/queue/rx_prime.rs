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

//! Handing receive descriptors to the part. Every field is stored before
//! OWN, and OWN alone is stored last: the part may take the descriptor the
//! moment it sees that bit.

use core::ptr::{addr_of_mut, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{bcnt, BUFFER_LEN, OWN, RX_DESC_COUNT};

use super::rx::RxRing;

// SAFETY anchor: `idx` is below RX_DESC_COUNT, see rx.rs.
impl RxRing {
    /// Give slot `idx` back to the part, empty.
    pub fn recycle(&mut self, idx: u16) {
        let d = unsafe { self.descriptor(idx) };
        unsafe {
            write_volatile(addr_of_mut!((*d).addr), self.buffer_device_addr(idx));
            write_volatile(addr_of_mut!((*d).bcnt), bcnt(BUFFER_LEN));
            write_volatile(addr_of_mut!((*d).misc), 0);
            write_volatile(addr_of_mut!((*d).reserved), 0);
        }
        fence(Ordering::Release);
        unsafe { write_volatile(addr_of_mut!((*d).status), OWN) };
    }

    /// Every slot handed to the part, before the part is told where the
    /// ring is.
    pub fn prime(&mut self) {
        self.head = 0;
        for idx in 0..RX_DESC_COUNT as u16 {
            self.recycle(idx);
        }
    }
}
