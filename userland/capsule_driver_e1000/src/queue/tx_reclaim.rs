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

//! Completion on the TX ring: which descriptors the part has marked DD,
//! moving `clean` over them in order, and whether the ring has room.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{TX_DESC_COUNT, TX_STATUS_DD};

use super::tx::TxRing;

// SAFETY anchor: see tx.rs; `idx` is `clean`, below TX_DESC_COUNT.
impl TxRing {
    pub fn done(&self, idx: u16) -> bool {
        let dd = unsafe { read_volatile(addr_of!((*self.descriptor(idx)).status)) } & TX_STATUS_DD;
        fence(Ordering::Acquire);
        dd != 0
    }

    /// Advance `clean` over every descriptor the part has finished, in order.
    pub fn reclaim(&mut self) {
        while self.clean != self.tail && self.done(self.clean) {
            self.clean = (self.clean + 1) % (TX_DESC_COUNT as u16);
        }
    }

    /// Whether posting one more descriptor would reach one the part still owns.
    pub fn full(&self) -> bool {
        (self.tail + 1) % (TX_DESC_COUNT as u16) == self.clean
    }
}
