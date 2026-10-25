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
use super::region::{data_region, stage_fits};
use super::reply::send_reply;
use super::transfer::do_transfer;
use crate::controller::ControlRequest;
use crate::error::XhciError;
use crate::protocol::{
    Request, CONTROL_DATA_MAX, CONTROL_TRANSFER_REQUEST_LEN, E_INVAL, E_IO, E_PIPE,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::recover::{recover_after, DCI_EP0};

const CC_STALL: u8 = 6;

/// `OP_CONTROL_TRANSFER` (`slot, 0, bmRequestType, bRequest, wValue,
/// wIndex, wLength`, little-endian): one EP0 request. One OUT carries its
/// data stage after these ten bytes, wLength of them, and is refused
/// without them: the stage went out as zeros, so a class request that sets
/// a value (a sampling rate) set 0, and was reported a success. One IN
/// carries none, and its reply the bytes the device really sent.
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((head, out)) = body.split_first_chunk::<CONTROL_TRANSFER_REQUEST_LEN>() else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let (slot, bm_rt) = (head[0], head[2]);
    let le = |at: usize| u16::from_le_bytes([head[at], head[at + 1]]);
    if !stage_fits(bm_rt, le(8), out.len(), CONTROL_DATA_MAX) {
        return reply_with_status(tx, req, E_INVAL);
    }
    // Clamped, so a client's wLength of up to 65535 still fits `tx`.
    let w_len = le(8).min(CONTROL_DATA_MAX);
    let Ok(region) = data_region(&ctx.driver.dma_pool, w_len, out) else {
        return reply_with_status(tx, req, E_IO);
    };
    let request = ControlRequest {
        bm_request_type: bm_rt,
        b_request: head[3],
        w_value: le(4),
        w_index: le(6),
        data_phys: region.as_ref().map_or(0, |r| r.phys()),
        data_len: w_len,
    };
    match do_transfer(ctx, slot, request) {
        Ok(moved) => send_reply(tx, req, moved, region.as_ref()),
        Err(e) => {
            let clear = recover_after(ctx, slot, DCI_EP0, e, region);
            // A STALL is a request the device does not support, which the
            // class driver may take as an answer rather than a fault, but
            // only once EP0 is back; otherwise the pipe is not clear.
            let stall = e == XhciError::TransferCompletionFailed(CC_STALL);
            reply_with_status(tx, req, if stall && clear { E_PIPE } else { E_IO })
        }
    }
}
