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
//! len_le16`). Both reply `moved_le16, 0, 0`; a bulk IN follows it with the
//! bytes moved. A STALL replies `E_PIPE`, so the class driver can recover.
use super::reply::{send_with_data, DATA_AT};
use super::run::run;
use crate::error::{XhciError, XhciResult};
use crate::protocol::{Request, BULK_HEADER_LEN, BULK_MAX, E_INVAL, E_IO, E_PIPE};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
const CC_STALL: u8 = 6;
pub fn transfer_out(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let data = body.get(BULK_HEADER_LEN..).unwrap_or(&[]);
    let len =
        if body.len() >= BULK_HEADER_LEN { u16::from_le_bytes([body[2], body[3]]) } else { 0 };
    if data.is_empty() || data.len() != len as usize || data.len() > BULK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    finish(run(ctx, body[0], data, len as u32, None), req, tx);
}
pub fn transfer_in(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let len =
        if body.len() == BULK_HEADER_LEN { u16::from_le_bytes([body[2], body[3]]) } else { 0 };
    if len == 0 || len as usize > BULK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    let moved = run(ctx, body[0], &[], len as u32, Some(&mut tx[DATA_AT..]));
    finish(moved, req, tx);
}
fn finish(moved: XhciResult<u32>, req: &Request, tx: &mut [u8]) {
    match moved {
        Ok(n) => send_with_data(tx, req, [n as u8, (n >> 8) as u8, 0, 0], n as usize),
        Err(XhciError::TransferCompletionFailed(CC_STALL)) => reply_with_status(tx, req, E_PIPE),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
