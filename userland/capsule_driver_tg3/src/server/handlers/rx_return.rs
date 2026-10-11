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

//! The return ring's side of a receive: the next entry this driver posted,
//! and giving a buffer back with both mailboxes brought up to date.

use core::sync::atomic::{fence, Ordering};

use crate::constants::mailbox::{RCVRET_CON_MBOX_0, RX_STD_PROD_IDX_REG};
use crate::constants::ring::RX_RET_RING_SIZE;
use crate::queue::ret::Rx;
use crate::setup::Driver;

/// The next entry that is not foreign; a ring of nothing but foreign
/// entries ends after one pass.
pub fn next(d: &mut Driver) -> Option<Rx> {
    let hw = d.status.rx_producer();
    (0..RX_RET_RING_SIZE).map_while(|_| d.rx.consume(hw)).find(|e| *e != Rx::Skipped)
}

/// Back on the standard ring, if a buffer came off it, and the part told.
pub fn give_back(d: &mut Driver, buf: Option<u8>) {
    if let Some(buf) = buf {
        d.rx.post(buf);
    }
    // The copy out and the BD rewrite land before the part may reuse them.
    fence(Ordering::Release);
    // SAFETY: `d.regs` carries the broker MmioMap base for BAR0, and both
    // mailboxes are 32-bit registers inside it.
    unsafe {
        d.regs.w32(RCVRET_CON_MBOX_0, d.rx.ret_cons as u32);
        d.regs.w32(RX_STD_PROD_IDX_REG, d.rx.std_prod as u32);
    }
}
