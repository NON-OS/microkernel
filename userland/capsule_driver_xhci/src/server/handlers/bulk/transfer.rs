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
use crate::server::handlers::recover::recover_after;
const CC_STALL: u8 = 6;
pub fn transfer_out(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let data = body.get(BULK_HEADER_LEN..).unwrap_or(&[]);
    let len =
        if body.len() >= BULK_HEADER_LEN { u16::from_le_bytes([body[2], body[3]]) } else { 0 };
    if data.is_empty() || data.len() != len as usize || data.len() > BULK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    let moved = run(ctx, body[0], data, len as u32, None);
    recover_timeout(ctx, body[0], false, moved);
    finish(moved, req, tx);
}
pub fn transfer_in(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let len =
        if body.len() == BULK_HEADER_LEN { u16::from_le_bytes([body[2], body[3]]) } else { 0 };
    if len == 0 || len as usize > BULK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    let moved = run(ctx, body[0], &[], len as u32, Some(&mut tx[DATA_AT..]));
    recover_timeout(ctx, body[0], true, moved);
    finish(moved, req, tx);
}
/// A transfer that never completed leaves its TRB on the ring, and the next
/// one would queue behind it into the same page. The endpoint is stopped and
/// moved past it. A STALL is left to the class driver's OP_RESET_BULK.
fn recover_timeout(ctx: &mut Context, slot: u8, dir_in: bool, moved: XhciResult<u32>) {
    if moved != Err(XhciError::TransferCompletionTimeout) {
        return;
    }
    let max_slots = ctx.driver.layout.max_slots;
    let dci = match ctx.driver.slots.resources_mut(slot, max_slots).and_then(|r| r.bulk.as_ref()) {
        Some(p) if dir_in => p.dci_in,
        Some(p) => p.dci_out,
        None => return,
    };
    recover_after(ctx, slot, dci, XhciError::TransferCompletionTimeout);
}
fn finish(moved: XhciResult<u32>, req: &Request, tx: &mut [u8]) {
    match moved {
        Ok(n) => send_with_data(tx, req, [n as u8, (n >> 8) as u8, 0, 0], n as usize),
        Err(XhciError::TransferCompletionFailed(CC_STALL)) => reply_with_status(tx, req, E_PIPE),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
