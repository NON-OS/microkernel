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
//! `OP_CONFIGURE_BULK`: add a bulk IN and a bulk OUT endpoint to an
//! addressed slot. Request `slot, ep_in, ep_out, bursts, mps_in_le16,
//! mps_out_le16`; reply `dci_in, dci_out, 0, 0`. `bursts` holds the
//! SuperSpeed Endpoint Companion's bMaxBurst of the IN endpoint in bits 3:0
//! and of the OUT endpoint in bits 7:4; it was a reserved zero, which still
//! means one packet per burst, and it is ignored below SuperSpeed.
use super::configure_pipes::{run, Bursts};
use super::reply::send_prefix;
use crate::error::XhciError;
use crate::protocol::{Request, CONFIGURE_BULK_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::dci_from_ep_address;
/// OP_CONFIGURE_BULK: the first pair.
pub fn configure(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    configure_pair(ctx, req, body, tx, false)
}

/// OP_CONFIGURE_BULK2 (0x23): the second pair, on endpoints the first
/// does not hold; the same request and reply.
pub fn configure_second(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    configure_pair(ctx, req, body, tx, true)
}

fn configure_pair(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8], second: bool) {
    let well_formed = body.len() == CONFIGURE_BULK_REQUEST_LEN
        && body[1] & 0x80 != 0
        && body[2] & 0x80 == 0
        && body[1] & 0x0F != 0
        && body[2] & 0x0F != 0;
    if !well_formed {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (dci_in, dci_out) = (dci_from_ep_address(body[1]), dci_from_ep_address(body[2]));
    let mps = |at: usize| u16::from_le_bytes([body[at], body[at + 1]]);
    let bursts = Bursts { bulk_in: body[3] & 0x0F, bulk_out: body[3] >> 4 };
    match run(ctx, (body[0], second), (dci_in, mps(4)), (dci_out, mps(6)), bursts) {
        Ok(()) => send_prefix(tx, req, [dci_in, dci_out, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
