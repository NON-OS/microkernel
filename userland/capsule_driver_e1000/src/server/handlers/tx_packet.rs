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

use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::regs::REG_TDT;
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::protocol::{Request, E_AGAIN, E_INVAL, E_MSGSIZE, MAX_TX_PAYLOAD_BYTES};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

/*
 * A frame is answered once it is queued, not once it is on the wire. Waiting
 * for DD held the caller for as long as the link was down, reported E_IO for a
 * frame the part still sent later (so a retry sent it twice), and left the
 * descriptor posted for the next call to overwrite. Completion is now what
 * `reclaim` observes, and a ring with no free slot says so.
 */
pub fn handle(sender: u32, driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if req.payload_len as usize != body.len() {
        reply_with_status(sender, tx, req, E_MSGSIZE);
        return;
    }
    if body.len() < MIN_ETHERNET_FRAME
        || body.len() > MAX_ETHERNET_FRAME
        || body.len() as u32 > MAX_TX_PAYLOAD_BYTES
    {
        reply_with_status(sender, tx, req, E_INVAL);
        return;
    }
    driver.tx.reclaim();
    if driver.tx.full() {
        reply_with_status(sender, tx, req, E_AGAIN);
        return;
    }
    let dst = driver.tx.buffer_va(driver.tx.tail) as *mut u8;
    // SAFETY: `dst` is slot `tail` of the TX buffer grant, TX_BUFFER_LEN
    // bytes long, and `body` was held to MAX_ETHERNET_FRAME, which is
    // shorter; the caller's message and the grant never overlap.
    unsafe {
        core::ptr::copy_nonoverlapping(body.as_ptr(), dst, body.len());
    }
    let idx = driver.tx.post(body.len() as u16);
    let next_tdt = ((idx as u32) + 1) % (TX_DESC_COUNT as u32);
    // The frame bytes are plain stores; the tail write is what lets the part read them.
    fence(Ordering::Release);
    // SAFETY: `driver.regs` carries the broker MmioMap base for BAR0 and TDT
    // is a 32-bit aligned offset in it.
    unsafe {
        driver.regs.w32(REG_TDT, next_tdt);
    }
    reply_with_status(sender, tx, req, 0);
}
