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
use super::buffer::data_buffer;
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

const DEVICE_TO_HOST: u8 = 0x80;
const CC_STALL: u8 = 6;

/// `OP_CONTROL_TRANSFER` (`slot, 0, bmRequestType, bRequest, wValue,
/// wIndex, wLength`, little-endian): one EP0 request with no data stage or
/// an IN one, whose reply carries the bytes the device really sent.
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONTROL_TRANSFER_REQUEST_LEN {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (slot, bm_rt) = (body[0], body[2]);
    let le = |at: usize| u16::from_le_bytes([body[at], body[at + 1]]);
    // A host-to-device data stage needs bytes this request carries none
    // of: sent, it was the zeroed buffer, and reported a success.
    if bm_rt & DEVICE_TO_HOST == 0 && le(8) > 0 {
        return reply_with_status(tx, req, E_INVAL);
    }
    // Clamped, so a client's wLength of up to 65535 still fits `tx`.
    let w_len = le(8).min(CONTROL_DATA_MAX);
    let Ok(region) = data_buffer(ctx, w_len) else { return reply_with_status(tx, req, E_IO) };
    let request = ControlRequest {
        bm_request_type: bm_rt,
        b_request: body[3],
        w_value: le(4),
        w_index: le(6),
        data_phys: region.as_ref().map_or(0, |r| r.phys()),
        data_len: w_len,
    };
    match do_transfer(ctx, slot, request) {
        Ok(moved) => send_reply(tx, req, moved, region.as_ref()),
        Err(e) => {
            recover_after(ctx, slot, DCI_EP0, e, region);
            // A STALL is a request the device does not support, which the
            // class driver may take as an answer rather than a fault.
            let stall = e == XhciError::TransferCompletionFailed(CC_STALL);
            reply_with_status(tx, req, if stall { E_PIPE } else { E_IO })
        }
    }
}
