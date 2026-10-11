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

//! `OP_TX_PACKET`. A frame is answered once it is queued, not once it is on
//! the wire: completion is the status block's send consumer index, and a
//! ring with every slot in flight answers `E_AGAIN` rather than overwrite a
//! buffer the part may still be reading.

use core::sync::atomic::{fence, Ordering};

use crate::constants::mailbox::SNDHOST_PROD_MBOX_0;
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::protocol::{Request, E_AGAIN, E_INVAL, E_MSGSIZE, MAX_TX_PAYLOAD_BYTES};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

/*
 * SAFETY for every unsafe in this file: `next_buffer_va` is a slot of the
 * TX buffer grant, TX_BUFFER_LEN bytes, and the body is at most
 * MAX_ETHERNET_FRAME; `d.regs` carries the BAR0 base with the send
 * producer mailbox inside it.
 */
pub fn handle(sender: u32, d: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if req.payload_len as usize != body.len() {
        return reply_with_status(sender, tx, req, E_MSGSIZE);
    }
    if body.len() < MIN_ETHERNET_FRAME
        || body.len() > MAX_ETHERNET_FRAME
        || body.len() as u32 > MAX_TX_PAYLOAD_BYTES
    {
        return reply_with_status(sender, tx, req, E_INVAL);
    }
    d.tx.reclaim(d.status.tx_consumer());
    if d.tx.full() {
        return reply_with_status(sender, tx, req, E_AGAIN);
    }
    let dst = d.tx.next_buffer_va() as *mut u8;
    unsafe { core::ptr::copy_nonoverlapping(body.as_ptr(), dst, body.len()) };
    let prod = d.tx.post(body.len() as u16);
    // The frame and its BD are plain stores; the mailbox lets the part read them.
    fence(Ordering::Release);
    unsafe { d.regs.w32(SNDHOST_PROD_MBOX_0, prod as u32) };
    reply_with_status(sender, tx, req, 0);
}
