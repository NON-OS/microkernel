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
//! `OP_BULK_OUT` (`slot, 0, len_le16, data`) and `OP_BULK_IN` (`slot, 0,
//! `OP_BULK_IN_POLL` (`slot, 0, len_le16`): one look at the slot's polled
//! bulk IN. Data replies as `OP_BULK_IN` does, `moved_le16, 0, 0` and the
//! bytes; nothing yet replies `E_AGAIN` at once; a STALL replies `E_PIPE`.
use super::reply::{send_with_data, DATA_AT};
use crate::controller::{poll_bulk_in, BulkPoll};
use crate::error::{XhciError, XhciResult};
use crate::protocol::{Request, BULK_HEADER_LEN, BULK_MAX, E_AGAIN, E_INVAL, E_IO, E_PIPE};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
const CC_STALL: u8 = 6;
pub fn poll_in(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let len =
        if body.len() == BULK_HEADER_LEN { u16::from_le_bytes([body[2], body[3]]) } else { 0 };
    if len == 0 || len as usize > BULK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    match run(ctx, body[0], len as u32, &mut tx[DATA_AT..]) {
        Ok(BulkPoll::Complete(n)) => {
            send_with_data(tx, req, [n as u8, (n >> 8) as u8, 0, 0], n as usize)
        }
        Ok(BulkPoll::Pending) => reply_with_status(tx, req, E_AGAIN),
        Err(XhciError::TransferCompletionFailed(CC_STALL)) => reply_with_status(tx, req, E_PIPE),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
fn run(ctx: &mut Context, slot: u8, len: u32, out: &mut [u8]) -> XhciResult<BulkPoll> {
    let (doorbell, intr) = (ctx.driver.layout.doorbell_base, ctx.driver.layout.primary_intr_base);
    let max_slots = ctx.driver.layout.max_slots;
    let pipes = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .and_then(|r| r.bulk.as_mut())
        .ok_or(XhciError::ControllerUnsupported)?;
    let got = poll_bulk_in(doorbell, intr, &mut ctx.driver.event_ring, pipes, slot, len)?;
    if let BulkPoll::Complete(n) = got {
        // What came in is at most the armed request, which is at most BULK_MAX.
        let n = (n as usize).min(out.len()).min(BULK_MAX);
        /*
         * SAFETY: `poll_buf` is the slot's own DMA page of BULK_MAX bytes, and
         * its TRB has completed, so the device writes it no more.
         */
        unsafe {
            core::ptr::copy_nonoverlapping(pipes.poll_buf.as_mut_ptr::<u8>(), out.as_mut_ptr(), n)
        };
        return Ok(BulkPoll::Complete(n as u32));
    }
    Ok(got)
}
