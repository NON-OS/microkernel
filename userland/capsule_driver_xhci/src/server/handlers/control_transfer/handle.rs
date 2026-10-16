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
use crate::error::XhciError;
use crate::protocol::{Request, CONTROL_TRANSFER_REQUEST_LEN, E_INVAL, E_IO, E_PIPE};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::recover::{recover_after, DCI_EP0};

// Largest data stage this handler will accept. The reply is copied into the
// fixed-size `tx` buffer, so w_len must be clamped or a client-chosen w_len up
// to 65535 overflows tx (heap OOB write). 512 matches the config-descriptor cap
// and leaves ample headroom in tx (512 + header < 2068).
const CONTROL_TRANSFER_DATA_MAX: u16 = 512;

/// A request OUT carries its data stage after the ten setup bytes, wLength
/// of them; one IN carries none. The OUT stage used to go out as zeros, so
/// a class request that sets a value (a sampling rate) set 0.
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some((head, out)) = body.split_first_chunk::<CONTROL_TRANSFER_REQUEST_LEN>() else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let (slot, bm_rt, b_req) = (head[0], head[2], head[3]);
    let w_value = u16::from_le_bytes([head[4], head[5]]);
    let w_index = u16::from_le_bytes([head[6], head[7]]);
    let asked = u16::from_le_bytes([head[8], head[9]]);
    if !stage_fits(bm_rt, asked, out.len(), CONTROL_TRANSFER_DATA_MAX) {
        return reply_with_status(tx, req, E_INVAL);
    }
    let w_len = asked.min(CONTROL_TRANSFER_DATA_MAX);
    let Ok(region) = data_region(&ctx.driver.dma_pool, w_len, out) else {
        return reply_with_status(tx, req, E_IO);
    };
    let (data_phys, data_len) = region.as_ref().map_or((0u64, 0u16), |r| (r.phys(), w_len));
    let request = crate::controller::ControlRequest {
        bm_request_type: bm_rt,
        b_request: b_req,
        w_value,
        w_index,
        data_phys,
        data_len,
    };
    match do_transfer(ctx, slot, request) {
        Ok(actual) => send_reply(tx, req, actual, region.as_ref()),
        Err(e) => {
            recover_after(ctx, slot, DCI_EP0, e);
            reply_with_status(tx, req, if is_stall(e) { E_PIPE } else { E_IO })
        }
    }
}

/// The device answered with a STALL: a request it does not support, which
/// the class driver may take as an answer rather than a fault.
fn is_stall(e: XhciError) -> bool {
    e == XhciError::TransferCompletionFailed(CC_STALL)
}
const CC_STALL: u8 = 6;
