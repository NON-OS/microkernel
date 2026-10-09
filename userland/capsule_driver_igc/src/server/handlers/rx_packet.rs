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

//! `OP_RX_PACKET`. Hands up the frame at the ring head, or `E_AGAIN` when
//! the part has written nothing back, or `E_IO` for a slot it wrote back
//! with an error or a frame this driver does not take. Either way the slot
//! goes back to the part through RDT once its bytes are copied out.

use core::sync::atomic::{fence, Ordering};

use crate::constants::regs::REG_RDT;
use crate::protocol::{
    encode_response_header, write_status, Request, E_AGAIN, E_IO, RESP_HDR_LEN,
    RX_PAYLOAD_PREFIX_LEN, STATUS_LEN,
};
use crate::server::error::{reply, reply_with_status};
use crate::setup::Driver;

/*
 * SAFETY for every unsafe in this file: `driver.regs` carries the broker
 * MmioMap base for BAR0 with REG_RDT inside it, and `buffer_va(idx)` is the
 * slot `idx` of the RX buffer grant, RX_BUFFER_LEN bytes, which `consume`
 * only reports a length for when it fits MAX_ETHERNET_FRAME.
 */
pub fn handle(sender: u32, driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let (idx, len) = match driver.rx.consume() {
        Some(p) => p,
        None => {
            reply_with_status(sender, tx, req, E_AGAIN);
            return;
        }
    };
    if len == 0 {
        unsafe { driver.regs.w32(REG_RDT, idx as u32) };
        reply_with_status(sender, tx, req, E_IO);
        return;
    }
    let body_len = RX_PAYLOAD_PREFIX_LEN + len as usize;
    encode_response_header(tx, req, STATUS_LEN as u32 + body_len as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let prefix_off = RESP_HDR_LEN + STATUS_LEN;
    let body_off = prefix_off + RX_PAYLOAD_PREFIX_LEN;
    tx[prefix_off..body_off].copy_from_slice(&(len as u32).to_le_bytes());
    // Bounded by the reply buffer too, so a drift among the size constants
    // can never let a device-written length overflow it.
    let n = (len as usize).min(tx.len().saturating_sub(body_off));
    let src = driver.rx.buffer_va(idx) as *const u8;
    unsafe { core::ptr::copy_nonoverlapping(src, tx[body_off..].as_mut_ptr(), n) };
    // The copy out of the buffer ends before the part may write it again.
    fence(Ordering::Release);
    unsafe { driver.regs.w32(REG_RDT, idx as u32) };
    reply(sender, tx, prefix_off + body_len);
}
